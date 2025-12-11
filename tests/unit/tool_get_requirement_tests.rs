// Tests for Tool: reqlix_get_requirement (G.REQLIX_GET_REQUIREMENT.*)
// Covers Requirements: G.REQLIX_GET_REQUIREMENT.1, G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4, G.R.5

use reqlix::RequirementsServer;
use tempfile::TempDir;

use super::common::{
    create_agents_file_in_req_dir, create_category_file, create_category_file_in_req_dir,
    create_requirements_dir,
};

// =============================================================================
// Tests for reqlix_get_requirement (G.REQLIX_GET_REQUIREMENT.*)
// =============================================================================

/// Test: reqlix_get_requirement finds requirement by index
/// Precondition: System has category file with requirement
/// Action: Call reqlix_get_requirement with valid index
/// Result: Function returns requirement with title and text
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.1, G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4
#[test]
fn test_get_requirement_by_index() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"# Test Chapter

## G.T.1: Test Requirement

This is the requirement text.
It can span multiple lines.
"#;
    create_category_file(&temp_dir, "general", content);

    let requirement = RequirementsServer::find_requirement_streaming(
        &temp_dir.path().join("general.md"),
        "general",
        "G.T.1",
    )
    .unwrap();
    assert_eq!(requirement.index, "G.T.1");
    assert_eq!(requirement.title, "Test Requirement");
    assert!(requirement.text.contains("This is the requirement text"));
}

/// Test: reqlix_get_requirement returns error for non-existent requirement
/// Precondition: System has category file without the specified requirement
/// Action: Call reqlix_get_requirement with non-existent index
/// Result: Function returns error "Requirement not found"
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3
#[test]
fn test_get_requirement_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"# Test Chapter

## G.T.1: Test Requirement

Content.
"#;
    create_category_file(&temp_dir, "general", content);

    let result = RequirementsServer::find_requirement_streaming(
        &temp_dir.path().join("general.md"),
        "general",
        "G.T.999",
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

/// Test: find_requirement_streaming correctly identifies boundaries before next chapter
/// Precondition: System has category file with requirement followed by level-1 heading (new chapter)
/// Action: Call find_requirement_streaming for last requirement before new chapter
/// Result: Requirement text does NOT include the next chapter heading (G.R.5: ends at same or higher level)
/// Covers Requirement: G.R.5, G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4
#[test]
fn test_requirement_boundary_before_next_chapter() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"# First Chapter

## G.F.1: Last Requirement In Chapter

Content of last requirement.

# Second Chapter

## G.S.1: First Requirement In Second Chapter

Content of second chapter requirement.
"#;
    create_category_file(&temp_dir, "general", content);

    // Get the last requirement in first chapter
    let requirement = RequirementsServer::find_requirement_streaming(
        &temp_dir.path().join("general.md"),
        "general",
        "G.F.1",
    )
    .unwrap();

    // Verify requirement text does NOT include next chapter (G.R.5)
    assert!(requirement.text.contains("Content of last requirement"));
    assert!(
        !requirement.text.contains("# Second Chapter"),
        "Level-1 heading should end requirement, not be included in text"
    );
    assert!(
        !requirement.text.contains("G.S.1"),
        "Content after level-1 heading should not be in requirement"
    );
    assert!(
        !requirement
            .text
            .contains("Content of second chapter requirement"),
        "Content from next chapter should not be in requirement"
    );
}

// =============================================================================
// Batch operation tests (G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4, G.REQLIX_GET_REQUIREMENT.5, G.P.4)
// =============================================================================

/// Test: batch get_requirement with empty array returns empty result (G.P.4)
#[test]
fn test_batch_get_requirement_empty_array() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(vec![]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["data"], serde_json::json!([]));
}

/// Test: batch get_requirement with single element (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_get_requirement_single_element() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.1".to_string()]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert!(parsed["data"].is_array());
    assert_eq!(parsed["data"].as_array().unwrap().len(), 1);
    // Each element has success/data structure (G.REQLIX_GET_REQUIREMENT.4)
    assert_eq!(parsed["data"][0]["success"], true);
    assert_eq!(parsed["data"][0]["data"]["index"], "G.C.1");
}

/// Test: batch get_requirement with multiple elements (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_get_requirement_multiple_elements() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: First

Content one.

## G.C.2: Second

Content two.

## G.C.3: Third

Content three.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(),
            "G.C.2".to_string(),
            "G.C.3".to_string(),
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 3);
    // Each element has success/data structure (G.REQLIX_GET_REQUIREMENT.4)
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[0]["data"]["index"], "G.C.1");
    assert_eq!(data[1]["success"], true);
    assert_eq!(data[1]["data"]["index"], "G.C.2");
    assert_eq!(data[2]["success"], true);
    assert_eq!(data[2]["data"]["index"], "G.C.3");
}

/// Test: batch get_requirement preserves order (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_get_requirement_preserves_order() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: First

Content one.

## G.C.2: Second

Content two.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    // Request in reverse order
    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.2".to_string(), "G.C.1".to_string()]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    // Each element has success/data structure (G.REQLIX_GET_REQUIREMENT.4)
    assert_eq!(data[0]["data"]["index"], "G.C.2");
    assert_eq!(data[1]["data"]["index"], "G.C.1");
}

/// Test: batch get_requirement processes all elements and returns individual errors (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_get_requirement_processes_all() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(),
            "G.C.999".to_string(), // Does not exist
            "G.C.1".to_string(),
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Top-level success is true (G.REQLIX_GET_REQUIREMENT.4)
    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 3);
    // First element: success
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[0]["data"]["index"], "G.C.1");
    // Second element: error
    assert_eq!(data[1]["success"], false);
    assert!(data[1]["error"].as_str().unwrap().contains("not found"));
    // Third element: success (all elements processed)
    assert_eq!(data[2]["success"], true);
    assert_eq!(data[2]["data"]["index"], "G.C.1");
}

/// Test: batch get_requirement exceeds limit (G.REQLIX_GET_REQUIREMENT.5)
#[test]
fn test_batch_get_requirement_exceeds_limit() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let indices: Vec<String> = (1..=101).map(|i| format!("G.C.{}", i)).collect();
    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(indices),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("100"));
}

// =============================================================================
// Additional batch operation tests
// =============================================================================

/// Test: batch get with duplicate indices works (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_get_with_duplicates() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(),
            "G.C.1".to_string(),
            "G.C.1".to_string(),
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 3);
}

/// Test: batch get from multiple categories (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_get_multiple_categories() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: General\n\nGeneral content.\n",
    );
    create_category_file_in_req_dir(
        &req_dir,
        "testing",
        "# Chapter\n\n## T.C.1: Testing\n\nTesting content.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.1".to_string(), "T.C.1".to_string()]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 2);
    // Each element has success/data structure (G.REQLIX_GET_REQUIREMENT.4)
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[0]["data"]["category"], "general");
    assert_eq!(data[1]["success"], true);
    assert_eq!(data[1]["data"]["category"], "testing");
}

/// Test: batch update preserves order (G.REQLIX_U.3)
#[test]
fn test_batch_update_preserves_order() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: First

Old one.

## G.C.2: Second

Old two.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    // Update in reverse order
    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![
            reqlix::UpdateItem {
                index: "G.C.2".to_string(),
                text: "New two".to_string(),
                title: None,
            },
            reqlix::UpdateItem {
                index: "G.C.1".to_string(),
                text: "New one".to_string(),
                title: None,
            },
        ]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    // Each element has success/data structure (G.REQLIX_U.4)
    assert_eq!(data[0]["data"]["index"], "G.C.2");
    assert_eq!(data[1]["data"]["index"], "G.C.1");
}

/// Test: batch delete preserves order (G.TOOLREQLIXD.3)
#[test]
fn test_batch_delete_preserves_order() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: First

Content one.

## G.C.2: Second

Content two.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    // Delete in reverse order
    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.2".to_string(), "G.C.1".to_string()]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    // Each element has success/data structure (G.TOOLREQLIXD.4)
    assert_eq!(data[0]["data"]["index"], "G.C.2");
    assert_eq!(data[1]["data"]["index"], "G.C.1");
}

/// Test: batch at exact limit of 100 works (G.REQLIX_GET_REQUIREMENT.5)
#[test]
fn test_batch_get_at_limit() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");

    // Create 100 requirements
    let mut content = String::from("# Chapter\n\n");
    for i in 1..=100 {
        content.push_str(&format!("## G.C.{}: Req{}\n\nContent {}.\n\n", i, i, i));
    }
    create_category_file_in_req_dir(&req_dir, "general", &content);

    let indices: Vec<String> = (1..=100).map(|i| format!("G.C.{}", i)).collect();
    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(indices),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 100);
}

// =============================================================================
// More batch operation tests
// =============================================================================

/// Test: batch get with mixed success and error results (G.REQLIX_GET_REQUIREMENT.4)
#[test]
fn test_batch_get_mixed_results_structure() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: First\n\nContent one.\n\n## G.C.3: Third\n\nContent three.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(),
            "G.C.2".to_string(), // Does not exist
            "G.C.3".to_string(),
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 3);
    // First: success
    assert_eq!(data[0]["success"], true);
    assert!(data[0]["data"].is_object());
    // Second: error
    assert_eq!(data[1]["success"], false);
    assert!(data[1]["error"].is_string());
    // Third: success (processed despite previous error)
    assert_eq!(data[2]["success"], true);
    assert!(data[2]["data"].is_object());
}

/// Test: batch delete where second delete fails because first already deleted (G.TOOLREQLIXD.3)
#[test]
fn test_batch_delete_same_index_twice() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.1".to_string(), "G.C.1".to_string()]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 2);
    // First: success
    assert_eq!(data[0]["success"], true);
    // Second: error (already deleted)
    assert_eq!(data[1]["success"], false);
}

/// Test: batch update with title conflict in same batch (G.REQLIX_U.3)
#[test]
fn test_batch_update_title_conflict_in_batch() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: First\n\nContent one.\n\n## G.C.2: Second\n\nContent two.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![
            reqlix::UpdateItem {
                index: "G.C.1".to_string(),
                text: "New content".to_string(),
                title: Some("Conflict Title".to_string()),
            },
            reqlix::UpdateItem {
                index: "G.C.2".to_string(),
                text: "New content".to_string(),
                title: Some("Conflict Title".to_string()), // Will conflict with G.C.1's new title
            },
        ]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 2);
    // First: success
    assert_eq!(data[0]["success"], true);
    // Second: error (title conflict)
    assert_eq!(data[1]["success"], false);
    assert!(data[1]["error"]
        .as_str()
        .unwrap()
        .contains("already exists"));
}

/// Test: batch delete leaves chapter if some requirements remain (G.TOOLREQLIXD.3)
#[test]
fn test_batch_delete_partial_chapter() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: First\n\nOne.\n\n## G.C.2: Second\n\nTwo.\n\n## G.C.3: Third\n\nThree.\n",
    );

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.1".to_string(), "G.C.3".to_string()]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);

    // Chapter should still exist with G.C.2
    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(file_content.contains("# Chapter"));
    assert!(file_content.contains("G.C.2"));
    assert!(!file_content.contains("G.C.1"));
    assert!(!file_content.contains("G.C.3"));
}

/// Test: batch get with invalid index format (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_get_invalid_index_format() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(),
            "INVALID".to_string(), // Invalid format
            "G.C.1".to_string(),
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[1]["success"], false);
    assert_eq!(data[2]["success"], true);
}

/// Test: batch update with empty text in one item (G.REQLIX_U.3)
#[test]
fn test_batch_update_empty_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: First\n\nOne.\n\n## G.C.2: Second\n\nTwo.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![
            reqlix::UpdateItem {
                index: "G.C.1".to_string(),
                text: "Valid text".to_string(),
                title: None,
            },
            reqlix::UpdateItem {
                index: "G.C.2".to_string(),
                text: "".to_string(), // Empty text - invalid
                title: None,
            },
        ]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[1]["success"], false);
}

/// Test: batch delete from non-existent category (G.TOOLREQLIXD.3)
#[test]
fn test_batch_delete_nonexistent_category() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(),
            "X.C.1".to_string(), // Non-existent category
        ]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[1]["success"], false);
}

/// Test: single get returns correct response format (G.REQLIX_GET_REQUIREMENT.4)
#[test]
fn test_single_get_response_format() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test Title\n\nTest content.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Single("G.C.1".to_string()),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Single request format - data is object, not array
    assert_eq!(parsed["success"], true);
    assert!(parsed["data"].is_object());
    assert!(!parsed["data"].is_array());
    assert_eq!(parsed["data"]["index"], "G.C.1");
    assert_eq!(parsed["data"]["title"], "Test Title");
    assert_eq!(parsed["data"]["category"], "general");
    assert_eq!(parsed["data"]["chapter"], "Chapter");
}

/// Test: single get error returns correct format (G.REQLIX_GET_REQUIREMENT.4)
#[test]
fn test_single_get_error_format() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Single("G.C.999".to_string()),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Single request error - top level success is false
    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].is_string());
}

/// Test: batch operations each element has success field (G.REQLIX_GET_REQUIREMENT.4)
#[test]
fn test_batch_all_elements_have_success() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(),
            "G.C.999".to_string(),
            "G.C.1".to_string(),
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    let data = parsed["data"].as_array().unwrap();
    for item in data {
        assert!(
            item.get("success").is_some(),
            "Each batch element must have success field"
        );
    }
}

/// Test: batch get returns correct chapter for each element (G.REQLIX_GET_REQUIREMENT.4)
#[test]
fn test_batch_get_returns_correct_chapters() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter One\n\n## G.CHAPTERONE.1: Test\n\nContent.\n\n# Chapter Two\n\n## G.CHAPTERTWO.1: Test\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.CHAPTERONE.1".to_string(),
            "G.CHAPTERTWO.1".to_string(),
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data[0]["data"]["chapter"], "Chapter One");
    assert_eq!(data[1]["data"]["chapter"], "Chapter Two");
}

/// Test: batch update with too long title (G.REQLIX_U.3, G.P.1)
#[test]
fn test_batch_update_title_too_long() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: First\n\nOne.\n\n## G.C.2: Second\n\nTwo.\n",
    );

    let long_title = "A".repeat(101); // Exceeds 100 char limit
    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![
            reqlix::UpdateItem {
                index: "G.C.1".to_string(),
                text: "Valid text".to_string(),
                title: None,
            },
            reqlix::UpdateItem {
                index: "G.C.2".to_string(),
                text: "Valid text".to_string(),
                title: Some(long_title),
            },
        ]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[1]["success"], false);
}

/// Test: batch delete from multiple categories (G.TOOLREQLIXD.3)
#[test]
fn test_batch_delete_multiple_categories() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: General\n\nGeneral content.\n",
    );
    create_category_file_in_req_dir(
        &req_dir,
        "testing",
        "# Chapter\n\n## T.C.1: Testing\n\nTesting content.\n",
    );

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.1".to_string(), "T.C.1".to_string()]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[0]["data"]["category"], "general");
    assert_eq!(data[1]["success"], true);
    assert_eq!(data[1]["data"]["category"], "testing");
}

/// Test: batch preserves order with partial errors in middle (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_preserves_order_with_errors() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: First\n\nOne.\n\n## G.C.3: Third\n\nThree.\n\n## G.C.5: Fifth\n\nFive.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(),
            "G.C.2".to_string(), // error
            "G.C.3".to_string(),
            "G.C.4".to_string(), // error
            "G.C.5".to_string(),
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 5);
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[0]["data"]["index"], "G.C.1");
    assert_eq!(data[1]["success"], false);
    assert_eq!(data[2]["success"], true);
    assert_eq!(data[2]["data"]["index"], "G.C.3");
    assert_eq!(data[3]["success"], false);
    assert_eq!(data[4]["success"], true);
    assert_eq!(data[4]["data"]["index"], "G.C.5");
}

/// Test: batch update verifies text actually changed in file (G.REQLIX_U.3)
#[test]
fn test_batch_update_changes_file_content() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: First\n\nOld one.\n\n## G.C.2: Second\n\nOld two.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![
            reqlix::UpdateItem {
                index: "G.C.1".to_string(),
                text: "UPDATED ONE".to_string(),
                title: None,
            },
            reqlix::UpdateItem {
                index: "G.C.2".to_string(),
                text: "UPDATED TWO".to_string(),
                title: None,
            },
        ]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);

    // Verify file content actually changed
    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(file_content.contains("UPDATED ONE"));
    assert!(file_content.contains("UPDATED TWO"));
    assert!(!file_content.contains("Old one"));
    assert!(!file_content.contains("Old two"));
}

/// Test: batch delete removes chapter when all requirements deleted (G.TOOLREQLIXD.3)
#[test]
fn test_batch_delete_removes_empty_chapter() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter One\n\n## G.CHAPTERONE.1: Test\n\nContent.\n\n# Chapter Two\n\n## G.CHAPTERTWO.1: Keep\n\nContent.\n",
    );

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.CHAPTERONE.1".to_string()]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);

    // Chapter One should be removed, Chapter Two should remain
    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(!file_content.contains("# Chapter One"));
    assert!(file_content.contains("# Chapter Two"));
    assert!(file_content.contains("G.CHAPTERTWO.1"));
}

/// Test: batch processes all elements even after multiple errors (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_continues_after_multiple_errors() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.5: Last\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(), // error
            "G.C.2".to_string(), // error
            "G.C.3".to_string(), // error
            "G.C.4".to_string(), // error
            "G.C.5".to_string(), // success - must be processed
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 5);
    // First 4 are errors
    for item in data.iter().take(4) {
        assert_eq!(item["success"], false);
    }
    // Last one should be processed and succeed
    assert_eq!(data[4]["success"], true);
    assert_eq!(data[4]["data"]["index"], "G.C.5");
}

/// Test: single update response format (G.REQLIX_U.4)
#[test]
fn test_single_update_response_format() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nOld content.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some("New content".to_string()),
        title: None,
        items: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Single update - data is object, not array
    assert_eq!(parsed["success"], true);
    assert!(parsed["data"].is_object());
    assert!(!parsed["data"].is_array());
    assert_eq!(parsed["data"]["text"], "New content");
}

/// Test: single delete response format (G.TOOLREQLIXD.4)
#[test]
fn test_single_delete_response_format() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test Title\n\nContent.\n",
    );

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Single("G.C.1".to_string()),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Single delete - data is object, not array
    assert_eq!(parsed["success"], true);
    assert!(parsed["data"].is_object());
    assert!(!parsed["data"].is_array());
    assert_eq!(parsed["data"]["index"], "G.C.1");
    assert_eq!(parsed["data"]["title"], "Test Title");
}

/// Test: batch get does not modify file (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_get_readonly() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let original_content = "# Chapter\n\n## G.C.1: Test\n\nContent.\n";
    create_category_file_in_req_dir(&req_dir, "general", original_content);

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.1".to_string(), "G.C.1".to_string()]),
    };
    let _ = RequirementsServer::handle_get_requirement(params);

    // File should be unchanged
    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert_eq!(file_content, original_content);
}

/// Test: batch update with index not found in first item (G.REQLIX_U.3)
#[test]
fn test_batch_update_first_item_error() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.2: Second\n\nTwo.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![
            reqlix::UpdateItem {
                index: "G.C.1".to_string(), // Does not exist
                text: "New".to_string(),
                title: None,
            },
            reqlix::UpdateItem {
                index: "G.C.2".to_string(),
                text: "Updated".to_string(),
                title: None,
            },
        ]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    // First: error
    assert_eq!(data[0]["success"], false);
    // Second: success (continues despite first error)
    assert_eq!(data[1]["success"], true);
}

/// Test: batch with all errors returns success at top level (G.REQLIX_GET_REQUIREMENT.4)
#[test]
fn test_batch_all_errors_top_level_success() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(), // error
            "G.C.2".to_string(), // error
            "G.C.3".to_string(), // error
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Top-level success is true for batch
    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    // All elements are errors
    for item in data {
        assert_eq!(item["success"], false);
    }
}

// =============================================================================
// Additional tests
// =============================================================================

/// Test: get_requirement with multipart index
#[test]
fn test_get_requirement_multipart_index() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Long Chapter Name\n\n## G.LONGCHAPTER.1: Test\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Single("G.LONGCHAPTER.1".to_string()),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["data"]["index"], "G.LONGCHAPTER.1");
}

/// Test: requirement with code block in content (G.R.5)
#[test]
fn test_requirement_with_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Code Example

Here is code:

```rust
fn main() {
    ## This is not a heading
    # Neither is this
}
```

End of requirement.

## G.C.2: Next

Next content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Single("G.C.1".to_string()),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let text = parsed["data"]["text"].as_str().unwrap();
    assert!(text.contains("## This is not a heading"));
    assert!(text.contains("End of requirement."));
}

// =============================================================================
// Additional batch operation tests (G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_U.3, G.TOOLREQLIXD.3)
// =============================================================================
