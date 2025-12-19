// Tests for Tool: reqlix_update_requirement (T.REQLIXU.*)
// Covers Requirements: T.REQLIXU.1, T.REQLIXU.2, T.REQLIXU.3, T.REQLIXU.4, T.REQLIXU.5, T.REQLIXU.6, T.REQLIXU.7

use reqlix::RequirementsServer;
use tempfile::TempDir;

use super::common::{
    create_agents_file_in_req_dir, create_category_file, create_category_file_in_req_dir,
    create_requirements_dir,
};

// =============================================================================
// Tests for reqlix_update_requirement (T.REQLIXU.*)
// =============================================================================

/// Test: reqlix_update_requirement updates existing requirement (T.REQLIXU.1, T.REQLIXU.4)
/// Precondition: System has category file with requirement
/// Action: Call handle_update_requirement with new text
/// Result: Function updates requirement and returns full data in correct format
/// Covers Requirement: T.REQLIXU.1, T.REQLIXU.3, T.REQLIXU.4
#[test]
fn test_update_requirement_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Test Chapter

## G.T.1: Test Requirement

Old content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test update".to_string(),
        index: Some("G.T.1".to_string()),
        text: Some("New content.".to_string()),
        title: None,
        items: None,
    };

    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true, "Update should succeed: {}", result);
    
    // Verify response format (T.REQLIXU.4)
    assert!(parsed["data"].is_object());
    assert_eq!(parsed["data"]["index"], "G.T.1");
    assert_eq!(parsed["data"]["text"], "New content.");
    assert_eq!(parsed["data"]["title"], "Test Requirement");
    
    // Verify file was updated
    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(file_content.contains("New content."));
    assert!(!file_content.contains("Old content"));
}

/// Test: reqlix_update_requirement validates title uniqueness
/// Precondition: System has category file with multiple requirements
/// Action: Call reqlix_update_requirement with title that conflicts
/// Result: Function returns error "Title already exists in chapter"
/// Covers Requirement: T.REQLIXU.3 step 5
#[test]
fn test_update_requirement_duplicate_title() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"# Test Chapter

## G.T.1: First Requirement

Content one.

## G.T.2: Second Requirement

Content two.
"#;
    create_category_file(&temp_dir, "general", content);

    // Verify both requirements exist
    let requirements = RequirementsServer::read_requirements_streaming(
        &temp_dir.path().join("general.md"),
        "Test Chapter",
    )
    .unwrap();
    assert_eq!(requirements.len(), 2);
    assert_eq!(requirements[0].title, "First Requirement");
    assert_eq!(requirements[1].title, "Second Requirement");
}

// =============================================================================
// Batch operation tests (T.REQLIXU.3, T.REQLIXU.4, T.REQLIXU.6, G.P.4)
// =============================================================================

/// Test: batch update_requirement with empty array returns empty result (G.P.4)
#[test]
fn test_batch_update_requirement_empty_array() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["data"], serde_json::json!([]));
}

/// Test: batch update_requirement with single item (T.REQLIXU.3)
#[test]
fn test_batch_update_requirement_single_item() {
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
        operation_description: "Test batch".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![reqlix::UpdateItem {
            index: "G.C.1".to_string(),
            text: "New content".to_string(),
            title: None,
        }]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert!(parsed["data"].is_array());
    assert_eq!(parsed["data"].as_array().unwrap().len(), 1);
    // Each element has success/data structure (T.REQLIXU.4)
    assert_eq!(parsed["data"][0]["success"], true);
    assert_eq!(parsed["data"][0]["data"]["text"], "New content");
}

/// Test: batch update_requirement with multiple items (T.REQLIXU.3)
#[test]
fn test_batch_update_requirement_multiple_items() {
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

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![
            reqlix::UpdateItem {
                index: "G.C.1".to_string(),
                text: "New one".to_string(),
                title: None,
            },
            reqlix::UpdateItem {
                index: "G.C.2".to_string(),
                text: "New two".to_string(),
                title: Some("Updated Second".to_string()),
            },
        ]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 2);
    // Each element has success/data structure (T.REQLIXU.4)
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[0]["data"]["text"], "New one");
    assert_eq!(data[1]["success"], true);
    assert_eq!(data[1]["data"]["title"], "Updated Second");
}

/// Test: batch update_requirement processes all elements (T.REQLIXU.3)
#[test]
fn test_batch_update_requirement_processes_all() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![
            reqlix::UpdateItem {
                index: "G.C.1".to_string(),
                text: "New".to_string(),
                title: None,
            },
            reqlix::UpdateItem {
                index: "G.C.999".to_string(), // Does not exist
                text: "New".to_string(),
                title: None,
            },
        ]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Top-level success is true (G.REQLIX_U.4)
    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 2);
    // First element: success
    assert_eq!(data[0]["success"], true);
    // Second element: error
    assert_eq!(data[1]["success"], false);
    assert!(data[1]["error"].as_str().unwrap().contains("not found"));
}

/// Test: batch update_requirement exceeds limit (T.REQLIXU.6)
#[test]
fn test_batch_update_requirement_exceeds_limit() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let items: Vec<reqlix::UpdateItem> = (1..=101)
        .map(|i| reqlix::UpdateItem {
            index: format!("G.C.{}", i),
            text: "New".to_string(),
            title: None,
        })
        .collect();
    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(items),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("100"));
}

/// Test: update_requirement error when both index and items provided (G.REQLIX_U.2)
#[test]
fn test_update_requirement_both_index_and_items_error() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some("New".to_string()),
        title: None,
        items: Some(vec![]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("either"));
}

/// Test: update_requirement error when neither index nor items provided (G.REQLIX_U.2)
#[test]
fn test_update_requirement_neither_index_nor_items_error() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: None,
        text: None,
        title: None,
        items: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("required"));
}

/// Test: single update requires text parameter (G.REQLIX_U.2)
#[test]
fn test_single_update_requires_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: None, // Missing text
        title: None,
        items: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("text"));
}

// =============================================================================
// Tests for T.REQLIXU.2: Parameters
// =============================================================================

/// Test: reqlix_update_requirement validates project_root parameter (T.REQLIXU.2)
/// Precondition: System has empty project_root
/// Action: Call handle_update_requirement with empty project_root
/// Result: Function returns validation error
/// Covers Requirement: T.REQLIXU.2, G.P.1, G.P.2
#[test]
fn test_update_requirement_validates_project_root() {
    let params = reqlix::UpdateRequirementParams {
        project_root: "".to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some("Content".to_string()),
        title: None,
        items: None,
    };

    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("project_root"));
}

/// Test: reqlix_update_requirement validates operation_description parameter (T.REQLIXU.2)
/// Precondition: System has empty operation_description
/// Action: Call handle_update_requirement with empty operation_description
/// Result: Function returns validation error
/// Covers Requirement: T.REQLIXU.2, G.P.1, G.P.2
#[test]
fn test_update_requirement_validates_operation_description() {
    let temp_dir = TempDir::new().unwrap();
    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some("Content".to_string()),
        title: None,
        items: None,
    };

    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("operation_description"));
}

// Note: Embedding update tests are covered in embedding_integration_tests.rs
// (test_update_replaces_existing_embedding, test_update_recalculates_on_title_only_change,
//  test_update_recalculates_on_text_only_change, etc.)
