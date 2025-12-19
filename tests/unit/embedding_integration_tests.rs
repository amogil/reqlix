// Integration tests for embeddings with insert/update/get operations
// Covers Requirements: T.REQLIXI.6, T.REQLIXU.7, G.R.14

use reqlix::RequirementsServer;
use tempfile::TempDir;

use super::common::{
    create_agents_file_in_req_dir, create_category_file_in_req_dir, create_requirements_dir,
};

// =============================================================================
// Tests for insert requirement with embeddings (T.REQLIXI.6)
// =============================================================================

#[test]
fn test_insert_creates_embedding_after_heading() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Test".to_string(),
        text: "Content".to_string(),
    };
    let result = RequirementsServer::handle_insert_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    if !parsed["success"].as_bool().unwrap_or(false) {
        eprintln!("Insert failed. Full result: {}", result);
        if let Some(err) = parsed.get("error") {
            eprintln!("Error: {}", err);
        }
    }
    assert_eq!(parsed["success"], true, "Insert failed: {}", result);

    let content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    let heading_idx = lines.iter().position(|l| l.contains("## G.C.1: Test")).unwrap();
    let embedding_idx = heading_idx + 1;
    assert!(reqlix::is_embedding_comment(lines[embedding_idx]));
}

#[test]
fn test_insert_embedding_before_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Test".to_string(),
        text: "Content".to_string(),
    };
    RequirementsServer::handle_insert_requirement(params);

    let content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    let heading_pos = content.find("## G.C.1: Test").unwrap();
    let embedding_pos = content.find("<!--embedding:").unwrap();
    let text_pos = content.find("Content").unwrap();
    assert!(embedding_pos > heading_pos);
    assert!(text_pos > embedding_pos);
}

#[test]
fn test_insert_embedding_format_correct() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Test".to_string(),
        text: "Content".to_string(),
    };
    RequirementsServer::handle_insert_requirement(params);

    let content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    let embedding_line = content
        .lines()
        .find(|l| l.contains("<!--embedding:"))
        .unwrap();
    assert!(embedding_line.contains("paraphrase-MiniLM-L3-v2"));
    assert!(embedding_line.contains(':'));
    assert!(embedding_line.ends_with("-->"));
}

#[test]
fn test_insert_embedding_uses_title_and_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    let params1 = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Title1".to_string(),
        text: "Text1".to_string(),
    };
    RequirementsServer::handle_insert_requirement(params1);

    let params2 = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Title2".to_string(),
        text: "Text2".to_string(),
    };
    RequirementsServer::handle_insert_requirement(params2);

    let content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    let embedding_lines: Vec<&str> = content
        .lines()
        .filter(|l| l.contains("<!--embedding:"))
        .collect();
    assert_eq!(embedding_lines.len(), 2);
    // Embeddings should be different because title+text are different
    assert_ne!(embedding_lines[0], embedding_lines[1]);
}

#[test]
fn test_insert_embedding_same_title_different_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    let params1 = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Same Title".to_string(),
        text: "Text1".to_string(),
    };
    RequirementsServer::handle_insert_requirement(params1);

    let params2 = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Same Title".to_string(),
        text: "Text2".to_string(),
    };
    let result = RequirementsServer::handle_insert_requirement(params2);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    // Should fail due to duplicate title
    assert_eq!(parsed["success"], false);
}

#[test]
fn test_insert_embedding_empty_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Test".to_string(),
        text: "".to_string(), // Empty text
    };
    let result = RequirementsServer::handle_insert_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], false); // Empty text should fail validation
}

#[test]
fn test_insert_embedding_very_long_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    let long_text = "x".repeat(9999); // Just under 10000 limit
    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Test".to_string(),
        text: long_text,
    };
    let result = RequirementsServer::handle_insert_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);
}

#[test]
fn test_insert_embedding_special_characters_in_title() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Title: with-colon & dash".to_string(),
        text: "Content".to_string(),
    };
    let result = RequirementsServer::handle_insert_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);

    let content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(content.contains("<!--embedding:"));
}

#[test]
fn test_insert_embedding_multiline_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    let multiline_text = "Line 1\nLine 2\nLine 3";
    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Test".to_string(),
        text: multiline_text.to_string(),
    };
    let result = RequirementsServer::handle_insert_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);

    let content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(content.contains("<!--embedding:"));
    assert!(content.contains("Line 1"));
    assert!(content.contains("Line 3"));
}

// =============================================================================
// Tests for update requirement with embeddings (T.REQLIXU.7)
// =============================================================================

#[test]
fn test_update_replaces_existing_embedding() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Old Title
<!--embedding:paraphrase-MiniLM-L3-v2:old_vector-->

Old content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some("New content.".to_string()),
        title: Some("New Title".to_string()),
        items: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);

    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    let embeddings: Vec<&str> = file_content
        .lines()
        .filter(|l| l.contains("<!--embedding:"))
        .collect();
    assert_eq!(embeddings.len(), 1); // Should have exactly one embedding
    assert!(!embeddings[0].contains("old_vector")); // Old embedding should be replaced
}

#[test]
fn test_update_adds_embedding_if_missing() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test

Content without embedding.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some("Updated content.".to_string()),
        title: None,
        items: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);

    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(file_content.contains("<!--embedding:"));
}

#[test]
fn test_update_recalculates_on_title_only_change() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Old Title
<!--embedding:paraphrase-MiniLM-L3-v2:old_vector-->

Same content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let file_before = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    let embedding_before = file_before
        .lines()
        .find(|l| l.contains("<!--embedding:"))
        .unwrap()
        .to_string();

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some("Same content.".to_string()), // Same text
        title: Some("New Title".to_string()),     // Only title changes
        items: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);

    let file_after = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    let embedding_after = file_after
        .lines()
        .find(|l| l.contains("<!--embedding:"))
        .unwrap()
        .to_string();

    assert_ne!(embedding_before, embedding_after, "Embedding should be recalculated");
}

#[test]
fn test_update_recalculates_on_text_only_change() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Same Title
<!--embedding:paraphrase-MiniLM-L3-v2:old_vector-->

Old content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let file_before = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    let embedding_before = file_before
        .lines()
        .find(|l| l.contains("<!--embedding:"))
        .unwrap()
        .to_string();

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some("New content.".to_string()), // Text changes
        title: None,                            // Title stays same
        items: None,
    };
    RequirementsServer::handle_update_requirement(params);

    let file_after = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    let embedding_after = file_after
        .lines()
        .find(|l| l.contains("<!--embedding:"))
        .unwrap()
        .to_string();

    assert_ne!(embedding_before, embedding_after);
}

#[test]
fn test_update_embedding_position_after_heading() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some("Updated".to_string()),
        title: None,
        items: None,
    };
    RequirementsServer::handle_update_requirement(params);

    let content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    let heading_idx = lines.iter().position(|l| l.contains("## G.C.1")).unwrap();
    let embedding_idx = heading_idx + 1;
    assert!(reqlix::is_embedding_comment(lines[embedding_idx]));
}

#[test]
fn test_update_multiple_requirements_embeddings() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: First
<!--embedding:paraphrase-MiniLM-L3-v2:old1-->

Content1.

## G.C.2: Second
<!--embedding:paraphrase-MiniLM-L3-v2:old2-->

Content2.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![
            reqlix::UpdateItem {
                index: "G.C.1".to_string(),
                text: "New1".to_string(),
                title: None,
            },
            reqlix::UpdateItem {
                index: "G.C.2".to_string(),
                text: "New2".to_string(),
                title: None,
            },
        ]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);

    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    let embeddings: Vec<&str> = file_content
        .lines()
        .filter(|l| l.contains("<!--embedding:"))
        .collect();
    assert_eq!(embeddings.len(), 2);
    assert!(!embeddings[0].contains("old1"));
    assert!(!embeddings[1].contains("old2"));
}

// =============================================================================
// Tests for get_requirement ignoring embeddings (G.R.14)
// =============================================================================

#[test]
fn test_get_requirement_excludes_embedding_from_text() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"# Chapter

## G.T.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Actual text content.
"#;
    super::common::create_category_file(&temp_dir, "general", content);

    let req = RequirementsServer::find_requirement_streaming(
        &temp_dir.path().join("general.md"),
        "general",
        "G.T.1",
    )
    .unwrap();

    assert!(!req.text.contains("<!--embedding:"));
    assert!(!req.text.contains("paraphrase-MiniLM-L3-v2"));
    assert!(req.text.contains("Actual text content"));
}

#[test]
fn test_get_requirement_multiple_embeddings_ignored() {
    let temp_dir = TempDir::new().unwrap();
    // Create requirement with multiple embedding-like comments (should only ignore real one)
    let content = r#"# Chapter

## G.T.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Text with <!--not-embedding:comment--> should be included.
"#;
    super::common::create_category_file(&temp_dir, "general", content);

    let req = RequirementsServer::find_requirement_streaming(
        &temp_dir.path().join("general.md"),
        "general",
        "G.T.1",
    )
    .unwrap();

    assert!(!req.text.contains("<!--embedding:"));
    assert!(req.text.contains("<!--not-embedding:comment-->"));
}

#[test]
fn test_get_requirement_embedding_not_in_boundaries() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"# Chapter

## G.T.1: First
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

First content.

## G.T.2: Second
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Second content.
"#;
    super::common::create_category_file(&temp_dir, "general", content);

    let req1 = RequirementsServer::find_requirement_streaming(
        &temp_dir.path().join("general.md"),
        "general",
        "G.T.1",
    )
    .unwrap();

    assert!(!req1.text.contains("<!--embedding:"));
    assert!(!req1.text.contains("G.T.2"));
    assert!(req1.text.contains("First content"));
}

#[test]
fn test_get_requirement_embedding_with_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"# Chapter

## G.T.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

```rust
fn main() {
    // <!--embedding: should be in code block
}
```

Text after code.
"#;
    super::common::create_category_file(&temp_dir, "general", content);

    let req = RequirementsServer::find_requirement_streaming(
        &temp_dir.path().join("general.md"),
        "general",
        "G.T.1",
    )
    .unwrap();

    assert!(!req.text.contains("<!--embedding:paraphrase-MiniLM-L3-v2"));
    assert!(req.text.contains("<!--embedding: should be in code block"));
    assert!(req.text.contains("Text after code"));
}

#[test]
fn test_get_requirement_batch_excludes_embeddings() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: First
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content1.

## G.C.2: Second
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content2.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.1".to_string(), "G.C.2".to_string()]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    for item in data {
        let text = item["data"]["text"].as_str().unwrap();
        assert!(!text.contains("<!--embedding:"));
    }
}
