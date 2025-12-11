// Markdown parsing helpers (G.R.2, G.R.3)

use crate::filesystem::{is_file_empty_or_whitespace, read_file_utf8};
use crate::models::{RequirementFull, RequirementSummary};
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};
use std::path::PathBuf;

/// Parse markdown level-1 heading according to G.R.2
/// Returns Some(chapter_name) if line is a valid level-1 heading, None otherwise
#[cfg_attr(test, allow(dead_code))]
pub fn parse_level1_heading(line: &str) -> Option<String> {
    // Remove up to 3 leading spaces (indentation) - G.R.2
    let space_count = line.chars().take_while(|&c| c == ' ').count();
    let trimmed = if space_count > 0 && space_count <= 3 {
        &line[space_count..]
    } else {
        line
    };

    // Must start with exactly one `#` followed by space - G.R.2
    if !trimmed.starts_with("# ") {
        return None;
    }
    // Must not be level-2 or higher
    if trimmed.starts_with("##") {
        return None;
    }

    // Extract chapter name using standard markdown parsing - G.R.2
    // Use pulldown-cmark to parse according to standard markdown rules
    let parser = Parser::new(trimmed);
    let events: Vec<Event> = parser.collect();

    // Check if we have a level-1 heading
    if events.len() >= 2 {
        match (&events[0], &events[1]) {
            (Event::Start(Tag::Heading(level, _, _)), Event::Text(text)) => {
                if level == &HeadingLevel::H1 {
                    // Check if there are more events (would indicate level-2 or higher)
                    if events.len() == 3 {
                        if let Event::End(Tag::Heading(end_level, _, _)) = &events[2] {
                            if end_level == &HeadingLevel::H1 {
                                return Some(text.to_string());
                            }
                        }
                    }
                }
            }
            (
                Event::Start(Tag::Heading(level, _, _)),
                Event::End(Tag::Heading(end_level, _, _)),
            ) => {
                // Handle empty heading: Start + End without Text
                if level == &HeadingLevel::H1 && end_level == &HeadingLevel::H1 {
                    // Empty heading content
                    return Some(String::new());
                }
            }
            _ => {}
        }
    }

    None
}

/// Parse markdown level-2 heading according to G.R.3
/// Returns Some((index, title)) if line is a valid level-2 requirement heading, None otherwise
#[cfg_attr(test, allow(dead_code))]
pub fn parse_level2_heading(line: &str) -> Option<(String, String)> {
    // Remove up to 3 leading spaces (indentation) - G.R.3
    let space_count = line.chars().take_while(|&c| c == ' ').count();
    let trimmed = if space_count > 0 && space_count <= 3 {
        &line[space_count..]
    } else {
        line
    };

    // Must start with exactly two `##` followed by space - G.R.3
    if !trimmed.starts_with("## ") {
        return None;
    }
    // Must not be level-3 or higher
    if trimmed.starts_with("###") {
        return None;
    }

    // Extract content using standard markdown parsing - G.R.3
    // Use pulldown-cmark to parse according to standard markdown rules
    let parser = Parser::new(trimmed);
    let events: Vec<Event> = parser.collect();

    // Check if we have a level-2 heading
    if events.len() >= 2 {
        if let (Event::Start(Tag::Heading(level, _, _)), Event::Text(text)) =
            (&events[0], &events[1])
        {
            if level == &HeadingLevel::H2 {
                // Check if there are more events (would indicate level-3 or higher)
                if events.len() == 3 {
                    if let Event::End(Tag::Heading(end_level, _, _)) = &events[2] {
                        if end_level == &HeadingLevel::H2 {
                            // Parse format: {index}: {title}
                            let content = text.to_string();
                            if let Some(colon_pos) = content.find(':') {
                                let index = content[..colon_pos].trim().to_string();
                                let title = content[colon_pos + 1..].trim().to_string();
                                if !index.is_empty() && !title.is_empty() {
                                    return Some((index, title));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

/// Read chapters from a category file (streaming) (G.REQLIX_GET_CH.3, G.R.2, G.R.8, G.R.9, G.R.10)
/// Parses markdown level-1 headings correctly, ignoring those inside code blocks
/// Handles empty files and whitespace-only files (G.R.10)
#[cfg_attr(test, allow(dead_code))]
pub fn read_chapters_streaming(category_path: &PathBuf) -> Result<Vec<String>, String> {
    // Read file as UTF-8 (G.R.8, G.R.9)
    let content = read_file_utf8(category_path)?;

    // Handle empty files (G.R.10)
    if is_file_empty_or_whitespace(&content) {
        return Ok(Vec::new());
    }

    let parser = Parser::new(&content);
    let mut chapters = Vec::new();
    let mut in_code_block = false;
    let mut current_heading_text = String::new();
    let mut current_heading_level: Option<HeadingLevel> = None;

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
            }
            Event::End(Tag::CodeBlock(_)) => {
                in_code_block = false;
            }
            Event::Start(Tag::Heading(level, _, _)) if !in_code_block => {
                current_heading_level = Some(level);
                current_heading_text.clear();
            }
            Event::Text(text)
                if !in_code_block && current_heading_level == Some(HeadingLevel::H1) =>
            {
                current_heading_text.push_str(&text);
            }
            Event::End(Tag::Heading(level, _, _))
                if !in_code_block && level == HeadingLevel::H1 =>
            {
                chapters.push(current_heading_text.trim().to_string());
                current_heading_text.clear();
                current_heading_level = None;
            }
            _ => {}
        }
    }

    Ok(chapters)
}

/// Read requirements from a chapter (streaming) (G.REQLIX_GET_REQUIREMENTS.3, G.R.3, G.R.5, G.R.8, G.R.9, G.R.10)
/// Parses markdown level-2 headings correctly, ignoring those inside code blocks
/// Handles empty files and chapters with no requirements (G.R.10)
#[cfg_attr(test, allow(dead_code))]
pub fn read_requirements_streaming(
    category_path: &PathBuf,
    chapter: &str,
) -> Result<Vec<RequirementSummary>, String> {
    // Read file as UTF-8 (G.R.8, G.R.9)
    let content = read_file_utf8(category_path)?;

    // Handle empty files (G.R.10)
    if is_file_empty_or_whitespace(&content) {
        return Ok(Vec::new());
    }

    let parser = Parser::new(&content);
    let mut requirements = Vec::new();
    let mut in_target_chapter = false;
    let mut in_code_block = false;
    let mut current_heading_text = String::new();
    let mut current_heading_level: Option<HeadingLevel> = None;

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
            }
            Event::End(Tag::CodeBlock(_)) => {
                in_code_block = false;
            }
            Event::Start(Tag::Heading(level, _, _)) if !in_code_block => {
                current_heading_level = Some(level);
                current_heading_text.clear();
            }
            Event::Text(text) if !in_code_block => {
                if current_heading_level == Some(HeadingLevel::H1) {
                    let chapter_name = text.trim().to_string();
                    in_target_chapter = chapter_name == chapter;
                    current_heading_text.clear();
                    current_heading_level = None;
                } else if current_heading_level == Some(HeadingLevel::H2) && in_target_chapter {
                    current_heading_text.push_str(&text);
                }
            }
            Event::End(Tag::Heading(level, _, _))
                if !in_code_block && level == HeadingLevel::H2 && in_target_chapter =>
            {
                // Parse format: {index}: {title}
                let content = current_heading_text.trim();
                if let Some(colon_pos) = content.find(':') {
                    let index = content[..colon_pos].trim().to_string();
                    let title = content[colon_pos + 1..].trim().to_string();
                    if !index.is_empty() && !title.is_empty() {
                        requirements.push(RequirementSummary { index, title });
                    }
                }
                current_heading_text.clear();
                current_heading_level = None;
            }
            _ => {}
        }
    }

    Ok(requirements)
}

/// Find requirement by index (streaming) (G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4, G.R.5)
/// Parses requirement boundaries correctly according to G.R.5:
/// - Requirement starts with markdown level-2 heading and includes all lines until next level-2 heading or EOF
/// - Code blocks are handled correctly (content within ``` is part of requirement)
/// - Level-1 headings within requirement body are still part of the requirement
#[cfg_attr(test, allow(dead_code))]
pub fn find_requirement_streaming(
    category_path: &PathBuf,
    category_name: &str,
    search_index: &str,
) -> Result<RequirementFull, String> {
    // Read file as UTF-8 (G.R.8, G.R.9)
    let content = read_file_utf8(category_path)?;
    let lines: Vec<&str> = content.lines().collect();

    // Use pulldown-cmark to find the requirement and determine chapter
    let parser = Parser::new(&content);
    let mut current_chapter = String::new();
    let mut found_requirement: Option<(String, String)> = None;
    let mut in_code_block = false;
    let mut current_heading_text = String::new();
    let mut current_heading_level: Option<HeadingLevel> = None;

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
            }
            Event::End(Tag::CodeBlock(_)) => {
                in_code_block = false;
            }
            Event::Start(Tag::Heading(level, _, _)) if !in_code_block => {
                current_heading_level = Some(level);
                current_heading_text.clear();
            }
            Event::Text(text) if !in_code_block => {
                if current_heading_level == Some(HeadingLevel::H1) {
                    current_chapter = text.trim().to_string();
                    current_heading_text.clear();
                    current_heading_level = None;
                } else if current_heading_level == Some(HeadingLevel::H2) {
                    current_heading_text.push_str(&text);
                }
            }
            Event::End(Tag::Heading(level, _, _))
                if !in_code_block && level == HeadingLevel::H2 =>
            {
                // Parse format: {index}: {title}
                let heading_content = current_heading_text.trim();
                if let Some(colon_pos) = heading_content.find(':') {
                    let index = heading_content[..colon_pos].trim().to_string();
                    let title = heading_content[colon_pos + 1..].trim().to_string();

                    if !index.is_empty() && !title.is_empty() && index == search_index {
                        found_requirement = Some((title, current_chapter.clone()));
                        break;
                    }
                }
                current_heading_text.clear();
                current_heading_level = None;
            }
            _ => {}
        }
    }

    if let Some((title, chapter)) = found_requirement {
        // Find the requirement heading line in the file
        let mut requirement_start_idx: Option<usize> = None;
        let mut in_code_block_line = false;

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Track code blocks
            if trimmed.starts_with("```") {
                in_code_block_line = !in_code_block_line;
            }

            // Find the requirement heading (not in code block)
            if !in_code_block_line {
                if let Some((index, _)) = parse_level2_heading(line) {
                    if index == search_index {
                        requirement_start_idx = Some(idx);
                        break;
                    }
                }
            }
        }

        if let Some(start_idx) = requirement_start_idx {
            // Find the end: next level-2 heading (not in code block) or end of file
            let mut requirement_end_idx = lines.len();
            let mut in_code_block_line = false;

            for (idx, line) in lines.iter().enumerate().skip(start_idx + 1) {
                let trimmed = line.trim();

                // Track code blocks
                if trimmed.starts_with("```") {
                    in_code_block_line = !in_code_block_line;
                }

                // Find next heading of same or higher level (not in code block) - G.R.5
                if !in_code_block_line {
                    // Level-1 heading ends requirement (higher level than level-2)
                    if parse_level1_heading(line).is_some() {
                        requirement_end_idx = idx;
                        break;
                    }
                    // Level-2 heading also ends requirement (same level)
                    if let Some((index, _)) = parse_level2_heading(line) {
                        if index != search_index {
                            requirement_end_idx = idx;
                            break;
                        }
                    }
                }
            }

            // Extract text (skip the heading line)
            let text_lines: Vec<&str> = lines[start_idx + 1..requirement_end_idx].to_vec();
            let text = text_lines.join("\n").trim().to_string();

            return Ok(RequirementFull {
                index: search_index.to_string(),
                title,
                text,
                category: category_name.to_string(),
                chapter,
            });
        }
    }

    Err("Requirement not found".to_string())
}

/// Parse index into parts (G.REQLIX_GET_REQUIREMENT.3)
#[cfg_attr(test, allow(dead_code))]
pub fn parse_index(index: &str) -> Result<(String, String, String), String> {
    let parts: Vec<&str> = index.split('.').collect();
    if parts.len() != 3 {
        return Err(format!("Invalid index format: {}", index));
    }
    Ok((
        parts[0].to_string(),
        parts[1].to_string(),
        parts[2].to_string(),
    ))
}
