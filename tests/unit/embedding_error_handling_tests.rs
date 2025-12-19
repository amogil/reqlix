// Tests for error handling in embedding operations
// Covers Requirements: T.REQLIXI.6, T.REQLIXU.7

use reqlix::RequirementsServer;
use tempfile::TempDir;

use super::common::{
    create_agents_file_in_req_dir, create_category_file_in_req_dir, create_requirements_dir,
};

// =============================================================================
// Tests for error handling in insert requirement (T.REQLIXI.6)
// =============================================================================

#[test]
fn test_insert_embedding_calculation_failure_aborts() {
    // Note: Current placeholder implementation doesn't fail, but when real Candle
    // implementation is added, this test verifies that failures abort the operation
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    // This test documents the expected behavior: if embedding calculation fails,
    // the insert operation should fail
    // TODO: When real embedding implementation is added, simulate failure condition
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
    // Currently succeeds because placeholder doesn't fail
    // When real implementation is added, this should test failure scenarios
    assert_eq!(parsed["success"], true);
}

#[test]
fn test_insert_embedding_must_exist_after_insertion() {
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
    assert_eq!(parsed["success"], true);

    // Verify embedding was created
    let content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(content.contains("<!--embedding:"));
}

#[test]
fn test_insert_embedding_format_matches_spec() {
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
    
    // Verify format: <!--embedding:<model_name>:<base64_vector>-->
    assert!(embedding_line.starts_with("<!--embedding:"));
    assert!(embedding_line.ends_with("-->"));
    assert!(embedding_line.matches(':').count() == 2); // Two colons: after embedding and after model
    assert!(embedding_line.contains("paraphrase-MiniLM-L3-v2"));
}

// =============================================================================
// Tests for error handling in update requirement (T.REQLIXU.7)
// =============================================================================

#[test]
fn test_update_embedding_calculation_failure_aborts() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    // This test documents expected behavior: if embedding calculation fails,
    // update should abort
    // TODO: When real implementation is added, simulate failure condition
    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some("New content.".to_string()),
        title: None,
        items: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    // Currently succeeds because placeholder doesn't fail
    assert_eq!(parsed["success"], true);
}

#[test]
fn test_update_embedding_replaced_not_appended() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:old_vector-->

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some("New content.".to_string()),
        title: None,
        items: None,
    };
    RequirementsServer::handle_update_requirement(params);

    let content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    let embeddings: Vec<&str> = content
        .lines()
        .filter(|l| l.contains("<!--embedding:"))
        .collect();
    assert_eq!(embeddings.len(), 1, "Should have exactly one embedding, not appended");
    assert!(!embeddings[0].contains("old_vector"), "Old embedding should be replaced");
}

#[test]
fn test_update_embedding_position_preserved() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
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
    RequirementsServer::handle_update_requirement(params);

    let content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    let heading_idx = lines.iter().position(|l| l.contains("## G.C.1")).unwrap();
    let embedding_idx = heading_idx + 1;
    assert!(reqlix::is_embedding_comment(lines[embedding_idx]));
}

#[test]
fn test_update_multiple_items_all_get_embeddings() {
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

## G.C.3: Third

Content3 without embedding.
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
            reqlix::UpdateItem {
                index: "G.C.3".to_string(),
                text: "New3".to_string(),
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
    assert_eq!(embeddings.len(), 3, "All three requirements should have embeddings");
}

#[test]
fn test_update_embedding_uses_title_and_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Old Title
<!--embedding:paraphrase-MiniLM-L3-v2:old_vector-->

Old text.
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
        text: Some("New text.".to_string()),
        title: Some("New Title".to_string()),
        items: None,
    };
    RequirementsServer::handle_update_requirement(params);

    let file_after = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    let embedding_after = file_after
        .lines()
        .find(|l| l.contains("<!--embedding:"))
        .unwrap()
        .to_string();

    // Embedding should be recalculated from "New Title: New text."
    assert_ne!(embedding_before, embedding_after);
}

#[test]
fn test_update_embedding_empty_text_handling() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some("".to_string()), // Empty text
        title: None,
        items: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], false); // Empty text should fail validation
}

#[test]
fn test_update_embedding_very_long_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let long_text = "x".repeat(9999);
    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some(long_text),
        title: None,
        items: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);
}

#[test]
fn test_update_embedding_special_characters() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let special_text = "Text with: colons, dashes-, and & symbols!";
    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some(special_text.to_string()),
        title: Some("Title: with-colon".to_string()),
        items: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);

    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(file_content.contains("<!--embedding:"));
}

#[test]
fn test_update_embedding_multiline_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let multiline_text = "Line 1\nLine 2\nLine 3\nWith\nMultiple\nLines";
    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some(multiline_text.to_string()),
        title: None,
        items: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);

    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(file_content.contains("<!--embedding:"));
    assert!(file_content.contains("Line 1"));
    assert!(file_content.contains("Line 3"));
}

#[test]
fn test_update_embedding_unicode_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let unicode_text = "Текст на русском 🚀 中文文本 日本語テキスト";
    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some(unicode_text.to_string()),
        title: Some("Заголовок".to_string()),
        items: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);

    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(file_content.contains("<!--embedding:"));
}
