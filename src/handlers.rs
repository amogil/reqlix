// Tool handlers

use crate::constants::MAX_BATCH_SIZE;
use crate::embeddings::MODEL_NAME;
use crate::filesystem::{
    find_or_create_requirements_file, get_requirements_dir, read_file_utf8, write_file_utf8,
};
use crate::helpers::{
    find_category_by_prefix, get_next_requirement_number, get_or_calculate_category_prefix,
    get_or_calculate_chapter_prefix, list_categories, title_exists_in_chapter,
};
use crate::models::{DeletedRequirement, RequirementFull};
use crate::params::*;
use crate::parsing::{
    find_requirement_streaming, parse_index, parse_level1_heading, parse_level2_heading,
    read_chapters_streaming, read_requirements_streaming,
};
use crate::response::{json_error, json_success};
use crate::validation::{
    validate_category, validate_chapter, validate_index, validate_keywords,
    validate_operation_description, validate_project_root, validate_text, validate_title,
};
use serde_json::json;
use std::fs;

/// Validate common parameters (project_root and operation_description)
/// Returns error string if validation fails, None otherwise
fn validate_common_params(project_root: &str, operation_description: &str) -> Option<String> {
    if let Err(e) = validate_project_root(project_root) {
        return Some(e);
    }
    if let Err(e) = validate_operation_description(operation_description) {
        return Some(e);
    }
    None
}

/// reqlix_get_instructions (T.R)
pub fn handle_get_instructions(params: GetInstructionsParams) -> String {
    if let Some(e) = validate_common_params(&params.project_root, &params.operation_description) {
        return json_error(&e);
    }

    // Find or create AGENTS.md
    let agents_path = match find_or_create_requirements_file(&params.project_root) {
        Ok(p) => p,
        Err(e) => return json_error(&e),
    };

    // Read AGENTS.md content
    let content = match read_file_utf8(&agents_path) {
        Ok(c) => c,
        Err(e) => return json_error(&e),
    };

    // Return JSON response (T.R.7)
    json_success(json!({ "content": content }))
}

/// reqlix_get_categories (T.REQLIXGETC)
pub fn handle_get_categories(params: GetCategoriesParams) -> String {
    if let Some(e) = validate_common_params(&params.project_root, &params.operation_description) {
        return json_error(&e);
    }

    // Get requirements directory
    let requirements_dir = match get_requirements_dir(&params.project_root) {
        Ok(d) => d,
        Err(e) => return json_error(&e),
    };

    // List categories (T.REQLIXGETC.3)
    let categories = match list_categories(&requirements_dir) {
        Ok(c) => c,
        Err(e) => return json_error(&e),
    };

    // Return JSON response (T.REQLIXGETC.3)
    json_success(json!({ "categories": categories }))
}

/// reqlix_get_chapters (T.REQLIXGETCH)
pub fn handle_get_chapters(params: GetChaptersParams) -> String {
    if let Some(e) = validate_common_params(&params.project_root, &params.operation_description) {
        return json_error(&e);
    }
    if let Err(e) = validate_category(&params.category) {
        return json_error(&e);
    }

    // Get requirements directory
    let requirements_dir = match get_requirements_dir(&params.project_root) {
        Ok(d) => d,
        Err(e) => return json_error(&e),
    };

    // Check if category file exists
    let category_path = requirements_dir.join(format!("{}.md", params.category));
    if !category_path.exists() {
        return json_error("Category not found");
    }

    // Read chapters (T.REQLIXGETCH.3)
    let chapters = match read_chapters_streaming(&category_path) {
        Ok(c) => c,
        Err(e) => return json_error(&e),
    };

    // Return JSON response (T.REQLIXGETCH.4)
    json_success(json!({
        "category": params.category,
        "chapters": chapters
    }))
}

/// reqlix_get_requirements (T.REQLIXGETR)
pub fn handle_get_requirements(params: GetRequirementsParams) -> String {
    if let Some(e) = validate_common_params(&params.project_root, &params.operation_description) {
        return json_error(&e);
    }
    if let Err(e) = validate_category(&params.category) {
        return json_error(&e);
    }
    if let Err(e) = validate_chapter(&params.chapter) {
        return json_error(&e);
    }

    // Get requirements directory
    let requirements_dir = match get_requirements_dir(&params.project_root) {
        Ok(d) => d,
        Err(e) => return json_error(&e),
    };

    // Check if category file exists
    let category_path = requirements_dir.join(format!("{}.md", params.category));
    if !category_path.exists() {
        return json_error("Category not found");
    }

    // Check if chapter exists
    let chapters = match read_chapters_streaming(&category_path) {
        Ok(c) => c,
        Err(e) => return json_error(&e),
    };
    if !chapters.contains(&params.chapter) {
        return json_error("Chapter not found");
    }

    // Read requirements (T.REQLIXGETR.3)
    let requirements = match read_requirements_streaming(&category_path, &params.chapter) {
        Ok(r) => r,
        Err(e) => return json_error(&e),
    };

    // Return JSON response (T.REQLIXGETR.4)
    json_success(json!({
        "category": params.category,
        "chapter": params.chapter,
        "requirements": requirements
    }))
}

/// Helper to get a single requirement by index (T.REQLIXGETREQUIREMENT.3)
fn get_single_requirement(project_root: &str, index: &str) -> Result<RequirementFull, String> {
    // Validate index
    validate_index(index)?;

    // Parse index (T.REQLIXGETREQUIREMENT.3)
    let (category_prefix, _chapter_prefix, _number) = parse_index(index)?;

    // Get requirements directory
    let requirements_dir = get_requirements_dir(project_root)?;

    // Find category by prefix (C.C.7)
    let category_name = find_category_by_prefix(&requirements_dir, &category_prefix)?;

    let category_path = requirements_dir.join(format!("{}.md", category_name));

    // Find requirement (T.REQLIXGETREQUIREMENT.3)
    find_requirement_streaming(&category_path, &category_name, index)
}

/// reqlix_get_requirement (T.REQLIXGETREQUIREMENT)
/// Supports single index or batch of up to 100 indices (T.REQLIXGETREQUIREMENT.2, T.REQLIXGETREQUIREMENT.5)
pub fn handle_get_requirement(params: GetRequirementParams) -> String {
    if let Some(e) = validate_common_params(&params.project_root, &params.operation_description) {
        return json_error(&e);
    }

    match params.index {
        // Single index (T.REQLIXGETREQUIREMENT.3 - single)
        IndexParam::Single(index) => match get_single_requirement(&params.project_root, &index) {
            Ok(requirement) => json_success(requirement),
            Err(e) => json_error(&e),
        },
        // Batch request (T.REQLIXGETREQUIREMENT.3 - batch)
        IndexParam::Batch(indices) => {
            // G.P.4: Empty array returns empty result
            if indices.is_empty() {
                return json_success(json!([]));
            }

            // T.REQLIXGETREQUIREMENT.5: Validate batch size
            if indices.len() > MAX_BATCH_SIZE {
                return json_error("Batch request exceeds maximum limit of 100 indices");
            }

            // Process ALL indices, return success/error for each (T.REQLIXGETREQUIREMENT.3, T.REQLIXGETREQUIREMENT.4)
            let mut results = Vec::with_capacity(indices.len());
            for index in &indices {
                match get_single_requirement(&params.project_root, index) {
                    Ok(requirement) => results.push(json!({
                        "success": true,
                        "data": requirement
                    })),
                    Err(e) => results.push(json!({
                        "success": false,
                        "error": e
                    })),
                }
            }

            // Return array of results (T.REQLIXGETREQUIREMENT.4)
            json_success(results)
        }
    }
}

/// reqlix_insert_requirement (T.REQLIXI)
/// Title must be generated by the LLM and provided as parameter. Must be unique within chapter (T.REQLIXI.3).
pub fn handle_insert_requirement(params: InsertRequirementParams) -> String {
    // Step 0: Validate parameters (T.REQLIXI.5, T.REQLIXI.3 step 0)
    if let Some(e) = validate_common_params(&params.project_root, &params.operation_description) {
        return json_error(&e);
    }
    if let Err(e) = validate_category(&params.category) {
        return json_error(&e);
    }
    if let Err(e) = validate_chapter(&params.chapter) {
        return json_error(&e);
    }
    if let Err(e) = validate_text(&params.text) {
        return json_error(&e);
    }
    if let Err(e) = validate_title(&params.title, true) {
        return json_error(&e);
    }

    // Get requirements directory
    let requirements_dir = match get_requirements_dir(&params.project_root) {
        Ok(d) => d,
        Err(e) => return json_error(&e),
    };

    let category_path = requirements_dir.join(format!("{}.md", params.category));

    // Step 1: Find or create category (T.REQLIXI.3 step 1, G.R.10)
    if !category_path.exists() {
        // Create empty file (G.R.10)
        if let Err(e) = write_file_utf8(&category_path, "") {
            return json_error(&e);
        }
    }

    // Step 2: Find or create chapter (T.REQLIXI.3 step 2)
    let chapters = match read_chapters_streaming(&category_path) {
        Ok(c) => c,
        Err(e) => return json_error(&e),
    };

    if !chapters.contains(&params.chapter) {
        // Append chapter heading
        let mut content = match read_file_utf8(&category_path) {
            Ok(c) => c,
            Err(e) => return json_error(&e),
        };
        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(&format!("\n# {}\n", params.chapter));
        if let Err(e) = write_file_utf8(&category_path, &content) {
            return json_error(&e);
        }
    }

    // Step 3: Validate title uniqueness (T.REQLIXI.3 step 3)
    match title_exists_in_chapter(&category_path, &params.chapter, &params.title, None) {
        Ok(true) => {
            return json_error("Title already exists in chapter");
        }
        Err(e) => return json_error(&e),
        _ => {}
    }

    // Step 4: Generate index (T.REQLIXI.3 step 4)
    let all_categories = match list_categories(&requirements_dir) {
        Ok(c) => c,
        Err(e) => return json_error(&e),
    };

    let category_prefix =
        match get_or_calculate_category_prefix(&category_path, &params.category, &all_categories) {
            Ok(p) => p,
            Err(e) => return json_error(&e),
        };

    let chapter_prefix = match get_or_calculate_chapter_prefix(&category_path, &params.chapter) {
        Ok(p) => p,
        Err(e) => return json_error(&e),
    };

    let number = match get_next_requirement_number(&category_path, &params.chapter) {
        Ok(n) => n,
        Err(e) => return json_error(&e),
    };

    let index = format!("{}.{}.{}", category_prefix, chapter_prefix, number);

    // Step 5: Insert requirement (T.REQLIXI.3 step 5)
    let mut content = match read_file_utf8(&category_path) {
        Ok(c) => c,
        Err(e) => return json_error(&e),
    };

    // Find position to insert (after chapter heading or at end of chapter)
    // Must find exact chapter heading (not a substring of another chapter name)
    let chapter_heading_newline = format!("# {}\n", params.chapter);
    let chapter_pos = content.find(&chapter_heading_newline).or_else(|| {
        // Handle case where chapter is at end of file without trailing newline
        let chapter_heading = format!("# {}", params.chapter);
        if content.ends_with(&chapter_heading) {
            Some(content.len() - chapter_heading.len())
        } else {
            None
        }
    });

    if let Some(chapter_pos) = chapter_pos {
        // Find end of chapter (next # heading or end of file)
        let chapter_heading = format!("# {}", params.chapter);
        let after_chapter = chapter_pos + chapter_heading.len();
        let insert_pos = content[after_chapter..]
            .find("\n# ")
            .map(|p| after_chapter + p)
            .unwrap_or(content.len());

        // Calculate embedding (T.REQLIXI.6)
        let embedding_text = format!("{}: {}", params.title, params.text);
        let embedding = match crate::embeddings::calculate_embedding(&embedding_text) {
            Ok(emb) => emb,
            Err(e) => return json_error(&format!("Failed to calculate embedding: {}", e)),
        };
        let embedding_comment = format!("\n{}\n", crate::embeddings::format_embedding_comment(MODEL_NAME, &embedding));
        
        let requirement_text = format!("\n## {}: {}{}\n\n{}\n", index, params.title, embedding_comment, params.text);
        content.insert_str(insert_pos, &requirement_text);
    } else {
        return json_error("Chapter not found after creation");
    }

    if let Err(e) = write_file_utf8(&category_path, &content) {
        return json_error(&e);
    }

    // Step 6: Return result (T.REQLIXI.3 step 6, T.REQLIXI.4)
    json_success(RequirementFull {
        index,
        title: params.title,
        text: params.text,
        category: params.category,
        chapter: params.chapter,
    })
}

/// Helper to update a single requirement (T.REQLIXU.3 steps 1-7)
fn update_single_requirement(
    project_root: &str,
    index: &str,
    text: &str,
    title: Option<&str>,
) -> Result<RequirementFull, String> {
    // Step 1: Validate parameters (T.REQLIXU.5, T.REQLIXU.3 step 1)
    validate_index(index)?;
    validate_text(text)?;
    if let Some(t) = title {
        validate_title(t, false)?;
    }

    // Step 2: Parse index (T.REQLIXU.3 step 2)
    let (category_prefix, _chapter_prefix, _number) = parse_index(index)?;

    // Get requirements directory
    let requirements_dir = get_requirements_dir(project_root)?;

    // Find category by prefix
    let category_name = find_category_by_prefix(&requirements_dir, &category_prefix)?;
    let category_path = requirements_dir.join(format!("{}.md", category_name));

    // Step 3: Find requirement (T.REQLIXU.3 step 3)
    let existing = find_requirement_streaming(&category_path, &category_name, index)?;

    // Step 4: Determine new title (T.REQLIXU.3 step 4)
    let title_provided = title.is_some();
    let new_title = title
        .map(|t| t.to_string())
        .unwrap_or(existing.title.clone());

    // Step 5: Validate title uniqueness (T.REQLIXU.3 step 5)
    if title_provided
        && title_exists_in_chapter(&category_path, &existing.chapter, &new_title, Some(index))?
    {
        return Err("Title already exists in chapter".to_string());
    }

    // Step 6: Update requirement (T.REQLIXU.3 step 6, T.REQLIXU.4, G.R.5)
    let content = fs::read_to_string(&category_path)
        .map_err(|e| format!("Failed to read category file: {}", e))?;

    // Find and replace the requirement using line-by-line parsing (G.R.5, G.R.3)
    let new_heading = format!("## {}: {}", index, new_title);
    let lines: Vec<&str> = content.lines().collect();

    let mut heading_start: Option<usize> = None;
    let mut req_end: Option<usize> = None;
    let mut in_code_block = false;
    let mut char_offset = 0;

    for line in &lines {
        let line_start = char_offset;
        let line_end = char_offset + line.len() + 1;

        if line.trim().starts_with("```") {
            in_code_block = !in_code_block;
        }

        if !in_code_block {
            if heading_start.is_some() && parse_level1_heading(line).is_some() {
                req_end = Some(line_start);
                break;
            }
            if let Some((idx, _)) = parse_level2_heading(line) {
                if idx == index {
                    heading_start = Some(line_start);
                } else if heading_start.is_some() {
                    req_end = Some(line_start);
                    break;
                }
            }
        }

        char_offset = line_end;
    }

    if let Some(start) = heading_start {
        let end = req_end.unwrap_or(content.len());
        
        // Calculate new embedding (T.REQLIXU.7)
        let embedding_text = format!("{}: {}", new_title, text);
        let embedding = match crate::embeddings::calculate_embedding(&embedding_text) {
            Ok(emb) => emb,
            Err(e) => return Err(format!("Failed to calculate embedding: {}", e)),
        };
        let embedding_comment = format!("\n{}\n", crate::embeddings::format_embedding_comment(MODEL_NAME, &embedding));

        let mut new_content = String::new();
        new_content.push_str(&content[..start]);
        new_content.push_str(&new_heading);
        new_content.push_str(&embedding_comment);
        new_content.push('\n');
        new_content.push_str(text);
        // G.R.11: Ensure blank line before next heading
        let remaining = &content[end..];
        if remaining.starts_with('#') || remaining.starts_with("\n#") {
            new_content.push_str("\n\n");
        } else {
            new_content.push('\n');
        }
        new_content.push_str(remaining);

        fs::write(&category_path, &new_content)
            .map_err(|e| format!("Failed to write category file: {}", e))?;
    } else {
        return Err("Could not find requirement to update".to_string());
    }

    // Step 7: Return result (T.REQLIXU.3 step 7)
    Ok(RequirementFull {
        index: index.to_string(),
        title: new_title,
        text: text.to_string(),
        category: category_name,
        chapter: existing.chapter,
    })
}

/// reqlix_update_requirement (T.REQLIXU)
/// Supports single update (index+text+title) or batch update (items array) (T.REQLIXU.2, T.REQLIXU.3, T.REQLIXU.6)
pub fn handle_update_requirement(params: UpdateRequirementParams) -> String {
    if let Some(e) = validate_common_params(&params.project_root, &params.operation_description) {
        return json_error(&e);
    }

    // Determine mode: single or batch (T.REQLIXU.2)
    match (&params.index, &params.items) {
        // Single update mode
        (Some(index), None) => {
            let text = match &params.text {
                Some(t) => t,
                None => return json_error("text is required for single update"),
            };
            match update_single_requirement(
                &params.project_root,
                index,
                text,
                params.title.as_deref(),
            ) {
                Ok(result) => json_success(result),
                Err(e) => json_error(&e),
            }
        }
        // Batch update mode (T.REQLIXU.3 batch)
        (None, Some(items)) => {
            // G.P.4: Empty array returns empty result
            if items.is_empty() {
                return json_success(json!([]));
            }

            // T.REQLIXU.6: Validate batch size
            if items.len() > MAX_BATCH_SIZE {
                return json_error("Batch update exceeds maximum limit of 100 items");
            }

            // Process ALL items, return success/error for each (T.REQLIXU.3, T.REQLIXU.4)
            let mut results = Vec::with_capacity(items.len());
            for item in items {
                match update_single_requirement(
                    &params.project_root,
                    &item.index,
                    &item.text,
                    item.title.as_deref(),
                ) {
                    Ok(result) => results.push(json!({
                        "success": true,
                        "data": result
                    })),
                    Err(e) => results.push(json!({
                        "success": false,
                        "error": e
                    })),
                }
            }

            // Return array of results (T.REQLIXU.4)
            json_success(results)
        }
        // Invalid: both provided
        (Some(_), Some(_)) => json_error(
            "Use either index+text+title for single update OR items for batch update, not both",
        ),
        // Invalid: neither provided
        (None, None) => {
            json_error("Either index (for single update) or items (for batch update) is required")
        }
    }
}

/// reqlix_get_version (T.REQLIXGETV)
/// Returns the version of the MCP server (T.REQLIXGETV.2, T.REQLIXGETV.3)
pub fn handle_get_version(_params: GetVersionParams) -> String {
    // T.REQLIXGETV.3: Use env!("CARGO_PKG_VERSION") macro at compile time
    let version = env!("CARGO_PKG_VERSION");

    // T.REQLIXGETV.2: Return success response
    json_success(json!({
        "version": version
    }))
}

/// Helper to delete a single requirement (T.REQLIXD.3 steps 1-6)
fn delete_single_requirement(
    project_root: &str,
    index: &str,
) -> Result<DeletedRequirement, String> {
    // Step 1: Validate index (T.REQLIXD.5)
    validate_index(index)?;

    // Step 2: Parse index (T.REQLIXD.3 step 2, G.R.4)
    let (category_prefix, _chapter_prefix, _req_number) = parse_index(index)?;

    // Find category by prefix (C.C.7)
    let req_dir = get_requirements_dir(project_root)?;
    let category = find_category_by_prefix(&req_dir, &category_prefix)?;
    let category_path = req_dir.join(format!("{}.md", category));

    // Step 3: Find requirement (T.REQLIXD.3 step 3)
    let requirement = find_requirement_streaming(&category_path, &category, index)
        .map_err(|_| "Requirement not found".to_string())?;

    // Read file content for modification
    let content = read_file_utf8(&category_path)?;

    // Step 4: Delete requirement (T.REQLIXD.3 step 4, G.R.5)
    let search_heading = format!("## {}: ", index);
    let mut heading_start: Option<usize> = None;
    let mut req_end: Option<usize> = None;
    let mut in_code_block = false;
    let mut line_start = 0;

    for line in content.lines() {
        let line_end = line_start + line.len() + 1;

        if line.trim_start().starts_with("```") {
            in_code_block = !in_code_block;
        }

        if !in_code_block {
            if heading_start.is_some() && parse_level1_heading(line).is_some() {
                req_end = Some(line_start);
                break;
            }
            if line.starts_with(&search_heading) {
                heading_start = Some(line_start);
            } else if heading_start.is_some() && parse_level2_heading(line).is_some() {
                req_end = Some(line_start);
                break;
            }
        }

        line_start = line_end;
    }

    let start = heading_start.ok_or("Requirement not found")?;
    let end = req_end.unwrap_or(content.len());

    // Build new content without the requirement
    let mut new_content = String::new();
    new_content.push_str(&content[..start]);

    // G.R.11: Handle blank lines
    let remaining = &content[end..];
    while new_content.ends_with('\n') {
        new_content.pop();
    }
    let remaining_trimmed = remaining.trim_start_matches('\n');
    if !remaining_trimmed.is_empty() {
        new_content.push_str("\n\n");
    }
    new_content.push_str(remaining_trimmed);

    // Step 5: Delete empty chapter (T.REQLIXD.3 step 5)
    let chapter_heading = format!("# {}", requirement.chapter);
    let chapter_heading_newline = format!("# {}\n", requirement.chapter);
    let chapter_pos = new_content.find(&chapter_heading_newline).or_else(|| {
        if new_content.ends_with(&chapter_heading) {
            Some(new_content.len() - chapter_heading.len())
        } else {
            None
        }
    });
    if let Some(chapter_pos) = chapter_pos {
        let after_chapter = chapter_pos + chapter_heading.len();
        let chapter_end = new_content[after_chapter..]
            .find("\n# ")
            .map(|p| after_chapter + p)
            .unwrap_or(new_content.len());

        let chapter_content = &new_content[after_chapter..chapter_end];
        let has_requirements = chapter_content
            .lines()
            .any(|line| parse_level2_heading(line).is_some());

        if !has_requirements {
            let chapter_line_start = new_content[..chapter_pos]
                .rfind('\n')
                .map(|p| p + 1)
                .unwrap_or(0);
            new_content = format!(
                "{}{}",
                &new_content[..chapter_line_start],
                &new_content[chapter_end..]
            );
        }
    }

    // Write updated content
    fs::write(&category_path, &new_content)
        .map_err(|e| format!("Failed to write category file: {}", e))?;

    // Step 6: Return result (T.REQLIXD.3 step 6)
    Ok(DeletedRequirement {
        index: index.to_string(),
        title: requirement.title,
        category,
        chapter: requirement.chapter,
    })
}

/// reqlix_delete_requirement (T.REQLIXD)
/// Supports single index or batch of up to 100 indices (T.REQLIXD.2, T.REQLIXD.6)
pub fn handle_delete_requirement(params: DeleteRequirementParams) -> String {
    if let Some(e) = validate_common_params(&params.project_root, &params.operation_description) {
        return json_error(&e);
    }

    match params.index {
        // Single delete (T.REQLIXD.3 - single)
        IndexParam::Single(index) => {
            match delete_single_requirement(&params.project_root, &index) {
                Ok(result) => json_success(result),
                Err(e) => json_error(&e),
            }
        }
        // Batch delete (T.REQLIXD.3 - batch)
        IndexParam::Batch(indices) => {
            // G.P.4: Empty array returns empty result
            if indices.is_empty() {
                return json_success(json!([]));
            }

            // T.REQLIXD.6: Validate batch size
            if indices.len() > MAX_BATCH_SIZE {
                return json_error("Batch delete exceeds maximum limit of 100 indices");
            }

            // Process ALL indices, return success/error for each (T.REQLIXD.3, T.REQLIXD.4)
            let mut results = Vec::with_capacity(indices.len());
            for index in &indices {
                match delete_single_requirement(&params.project_root, index) {
                    Ok(result) => results.push(json!({
                        "success": true,
                        "data": result
                    })),
                    Err(e) => results.push(json!({
                        "success": false,
                        "error": e
                    })),
                }
            }

            // Return array of results (T.REQLIXD.4)
            json_success(results)
        }
    }
}

/// reqlix_search_requirements (T.REQLIXS)
/// Searches for requirements by keywords across all categories (T.REQLIXS.3)
pub fn handle_search_requirements(params: SearchRequirementsParams) -> String {
    // T.REQLIXS.6: Validate parameters in order
    if let Some(e) = validate_common_params(&params.project_root, &params.operation_description) {
        return json_error(&e);
    }
    // Step 3: Validate and filter keywords
    let keywords = match validate_keywords(&params.keywords) {
        Ok(k) => k,
        Err(e) => return json_error(&e),
    };

    // T.REQLIXS.5, G.P.4: Empty keywords returns success with empty results
    if keywords.is_empty() {
        let empty_results: Vec<RequirementFull> = Vec::new();
        return json_success(json!({
            "keywords": keywords,
            "results": empty_results
        }));
    }

    // Get requirements directory
    let requirements_dir = match get_requirements_dir(&params.project_root) {
        Ok(d) => d,
        Err(e) => return json_error(&e),
    };

    // List all categories (T.REQLIXS.3 step 1)
    let categories = match list_categories(&requirements_dir) {
        Ok(c) => c,
        Err(e) => return json_error(&e),
    };

    let mut results: Vec<RequirementFull> = Vec::new();

    // Convert keywords to lowercase for case-insensitive search (T.REQLIXS.3 step 5)
    let keywords_lower: Vec<String> = keywords.iter().map(|k| k.to_lowercase()).collect();

    // T.REQLIXS.3 steps 1-7: Iterate over all categories, chapters, requirements
    for category in &categories {
        let category_path = requirements_dir.join(format!("{}.md", category));

        // Read chapters (T.REQLIXS.3 step 2)
        let chapters = match read_chapters_streaming(&category_path) {
            Ok(c) => c,
            Err(_) => continue, // Skip categories with read errors
        };

        // For each chapter (T.REQLIXS.3 step 3)
        for chapter in &chapters {
            // Read requirements in chapter
            let requirements = match read_requirements_streaming(&category_path, chapter) {
                Ok(r) => r,
                Err(_) => continue, // Skip chapters with read errors
            };

            // For each requirement (T.REQLIXS.3 step 4)
            for req_summary in &requirements {
                // Get full requirement
                let requirement = match find_requirement_streaming(
                    &category_path,
                    category,
                    &req_summary.index,
                ) {
                    Ok(r) => r,
                    Err(_) => continue, // Skip requirements with read errors
                };

                // T.REQLIXS.3 step 5-6: Case-insensitive substring search in title OR text
                let title_lower = requirement.title.to_lowercase();
                let text_lower = requirement.text.to_lowercase();

                let matches = keywords_lower
                    .iter()
                    .any(|kw| title_lower.contains(kw) || text_lower.contains(kw));

                if matches {
                    results.push(requirement);
                }
            }
        }
    }

    // T.REQLIXS.3 step 7, T.REQLIXS.4: Return results
    // Note: Order is undefined (T.REQLIXS.3)
    json_success(json!({
        "keywords": keywords,
        "results": results
    }))
}

/// reqlix_fuzzy_search_requirements (T.REQLIXF)
/// Searches for requirements using semantic similarity (fuzzy search) across all categories (T.REQLIXF.1, T.REQLIXF.3)
pub fn handle_fuzzy_search_requirements(params: FuzzySearchRequirementsParams) -> String {
    // T.REQLIXF.5: Validate parameters in order
    if let Some(e) = validate_common_params(&params.project_root, &params.operation_description) {
        return json_error(&e);
    }
    
    // Validate query parameter (G.P.1, T.REQLIXF.5)
    if params.query.len() > 10000 {
        return json_error("query exceeds maximum limit of 10000 characters");
    }
    // Check if query is empty or only whitespace
    if params.query.trim().is_empty() {
        return json_error("query is required");
    }
    
    // Validate limit parameter (G.P.1, T.REQLIXF.2)
    let limit = params.limit.unwrap_or(10);
    if !(1..=1000).contains(&limit) {
        return json_error("limit must be between 1 and 1000");
    }

    // Get requirements directory
    let requirements_dir = match get_requirements_dir(&params.project_root) {
        Ok(d) => d,
        Err(e) => return json_error(&e),
    };

    // T.REQLIXF.3 step 1: Collect all embeddings from requirement files
    let embeddings_map = match crate::embeddings::collect_embeddings(&requirements_dir) {
        Ok(map) => map,
        Err(e) => return json_error(&format!("Failed to collect embeddings: {}", e)),
    };

    if embeddings_map.is_empty() {
        return json_success(json!({
            "query": params.query,
            "results": []
        }));
    }

    // T.REQLIXF.3 step 3: Calculate embedding for query
    // Always use paraphrase-MiniLM-L3-v2, ignoring model names from embedding comments
    let query_embedding = match crate::embeddings::calculate_embedding(&params.query) {
        Ok(emb) => emb,
        Err(e) => return json_error(&format!("Failed to calculate query embedding: {}", e)),
    };

    // T.REQLIXF.3 step 4: Calculate cosine similarity and collect results
    let mut results_with_similarity: Vec<(RequirementFull, f32)> = Vec::new();

    for (index, req_embedding) in &embeddings_map {
        let similarity = crate::embeddings::cosine_similarity(&query_embedding, req_embedding);
        
        // Get full requirement
        if let Ok(requirement) = get_single_requirement(&params.project_root, index) {
            results_with_similarity.push((requirement, similarity));
        }
        // Skip requirements that can't be found
    }

    // T.REQLIXF.3 step 5: Sort by similarity (highest first)
    results_with_similarity.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // T.REQLIXF.3 step 6, T.REQLIXF.4: Format results with similarity scores and apply limit
    let results: Vec<serde_json::Value> = results_with_similarity
        .into_iter()
        .take(limit as usize)
        .map(|(req, similarity)| {
            json!({
                "index": req.index,
                "title": req.title,
                "text": req.text,
                "category": req.category,
                "chapter": req.chapter,
                "similarity": similarity
            })
        })
        .collect();

    json_success(json!({
        "query": params.query,
        "results": results
    }))
}
