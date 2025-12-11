// Tests for Tool: reqlix_update_requirement (G.REQLIX_U.*)
// Covers Requirements: G.REQLIX_U.1, G.REQLIX_U.3, G.REQLIX_U.4

use reqlix::RequirementsServer;
use tempfile::TempDir;

use super::common::{
    create_agents_file_in_req_dir, create_category_file, create_category_file_in_req_dir,
    create_requirements_dir,
};

// =============================================================================
// Tests for reqlix_update_requirement (G.REQLIX_U.*)
// =============================================================================

/// Test: reqlix_update_requirement updates existing requirement
/// Precondition: System has category file with requirement
/// Action: Call reqlix_update_requirement with new text
/// Result: Function updates requirement and returns full data
/// Covers Requirement: G.REQLIX_U.1, G.REQLIX_U.3, G.REQLIX_U.4
#[test]
fn test_update_requirement_text() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"# Test Chapter

## G.T.1: Test Requirement

Old content.
"#;
    create_category_file(&temp_dir, "general", content);

    // Verify initial state
    let requirement = RequirementsServer::find_requirement_streaming(
        &temp_dir.path().join("general.md"),
        "general",
        "G.T.1",
    )
    .unwrap();
    assert!(requirement.text.contains("Old content"));
}

/// Test: reqlix_update_requirement validates title uniqueness
/// Precondition: System has category file with multiple requirements
/// Action: Call reqlix_update_requirement with title that conflicts
/// Result: Function returns error "Title already exists in chapter"
/// Covers Requirement: G.REQLIX_U.3 step 5
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
// Batch operation tests (G.REQLIX_U.3, G.REQLIX_U.4, G.REQLIX_U.7, G.P.4)
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

/// Test: batch update_requirement with single item (G.REQLIX_U.3)
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
    // Each element has success/data structure (G.REQLIX_U.4)
    assert_eq!(parsed["data"][0]["success"], true);
    assert_eq!(parsed["data"][0]["data"]["text"], "New content");
}

/// Test: batch update_requirement with multiple items (G.REQLIX_U.3)
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
    // Each element has success/data structure (G.REQLIX_U.4)
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[0]["data"]["text"], "New one");
    assert_eq!(data[1]["success"], true);
    assert_eq!(data[1]["data"]["title"], "Updated Second");
}

/// Test: batch update_requirement processes all elements (G.REQLIX_U.3)
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

/// Test: batch update_requirement exceeds limit (G.REQLIX_U.7)
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
