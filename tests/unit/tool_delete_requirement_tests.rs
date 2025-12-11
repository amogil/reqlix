// Tests for Tool: reqlix_delete_requirement (G.TOOLREQLIXD.*)
// Covers Requirements: G.TOOLREQLIXD.1, G.TOOLREQLIXD.3, G.TOOLREQLIXD.4, G.TOOLREQLIXD.5

use reqlix::RequirementsServer;
use tempfile::TempDir;

use super::common::{
    create_agents_file_in_req_dir, create_category_file_in_req_dir, create_requirements_dir,
};

// =============================================================================
// Tests for reqlix_delete_requirement (G.TOOLREQLIXD.*)
// =============================================================================

/// Test: reqlix_delete_requirement deletes existing requirement
/// Precondition: System has category file with requirement
/// Action: Call handle_delete_requirement with valid index
/// Result: Requirement is deleted and metadata is returned
/// Covers Requirement: G.TOOLREQLIXD.1, G.TOOLREQLIXD.3, G.TOOLREQLIXD.4
#[test]
fn test_delete_requirement_success() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Test Chapter

## G.T.1: First Requirement

Content of first requirement.

## G.T.2: Second Requirement

Content of second requirement.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test delete".to_string(),
        index: reqlix::IndexParam::Single("G.T.1".to_string()),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Verify success response (G.TOOLREQLIXD.4)
    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["data"]["index"], "G.T.1");
    assert_eq!(parsed["data"]["title"], "First Requirement");
    assert_eq!(parsed["data"]["category"], "general");
    assert_eq!(parsed["data"]["chapter"], "Test Chapter");

    // Verify requirement is deleted from file
    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(!file_content.contains("G.T.1"));
    assert!(!file_content.contains("First Requirement"));
    // Second requirement should still exist
    assert!(file_content.contains("G.T.2"));
    assert!(file_content.contains("Second Requirement"));
}

/// Test: reqlix_delete_requirement returns error for non-existent requirement
/// Precondition: System has category file without the specified requirement
/// Action: Call handle_delete_requirement with non-existent index
/// Result: Function returns error "Requirement not found"
/// Covers Requirement: G.TOOLREQLIXD.3 step 3, G.TOOLREQLIXD.4
#[test]
fn test_delete_requirement_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Test Chapter

## G.T.1: Test Requirement

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test delete".to_string(),
        index: reqlix::IndexParam::Single("G.T.999".to_string()),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("not found"));
}

/// Test: reqlix_delete_requirement validates parameters
/// Precondition: System has invalid parameters
/// Action: Call handle_delete_requirement with invalid parameters
/// Result: Function returns validation error before processing
/// Covers Requirement: G.TOOLREQLIXD.5, G.P.1, G.P.2
#[test]
fn test_delete_requirement_validation() {
    let params = reqlix::DeleteRequirementParams {
        project_root: "".to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Single("G.T.1".to_string()),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    // Should fail on project_root validation
    assert!(parsed["error"].as_str().unwrap().contains("project_root"));
}

/// Test: reqlix_delete_requirement removes empty chapter
/// Precondition: System has chapter with single requirement
/// Action: Delete the only requirement in chapter
/// Result: Chapter heading is also removed
/// Covers Requirement: G.TOOLREQLIXD.3 step 5
#[test]
fn test_delete_requirement_removes_empty_chapter() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# First Chapter

## G.F.1: Only Requirement

Content.

# Second Chapter

## G.S.1: Another Requirement

More content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test delete".to_string(),
        index: reqlix::IndexParam::Single("G.F.1".to_string()),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);

    // Verify chapter is removed
    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(
        !file_content.contains("# First Chapter"),
        "Empty chapter should be removed"
    );
    assert!(!file_content.contains("G.F.1"));
    // Second chapter should still exist
    assert!(file_content.contains("# Second Chapter"));
    assert!(file_content.contains("G.S.1"));
}

/// Test: reqlix_delete_requirement handles last requirement in file
/// Precondition: System has category file with single requirement
/// Action: Delete the only requirement
/// Result: File becomes empty or contains only chapter heading
/// Covers Requirement: G.TOOLREQLIXD.3 step 4, G.TOOLREQLIXD.3 step 5
#[test]
fn test_delete_requirement_last_in_file() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Only Chapter

## G.O.1: Only Requirement

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test delete".to_string(),
        index: reqlix::IndexParam::Single("G.O.1".to_string()),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);

    // Verify requirement is deleted
    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(!file_content.contains("G.O.1"));
    assert!(!file_content.contains("Only Requirement"));
}

// =============================================================================
// Batch operation tests (G.TOOLREQLIXD.3, G.TOOLREQLIXD.4, G.TOOLREQLIXD.6, G.P.4)
// =============================================================================

/// Test: batch delete_requirement with empty array returns empty result (G.P.4)
#[test]
fn test_batch_delete_requirement_empty_array() {
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
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(vec![]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["data"], serde_json::json!([]));
}

/// Test: batch delete_requirement with single element (G.TOOLREQLIXD.3)
#[test]
fn test_batch_delete_requirement_single_element() {
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
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.1".to_string()]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert!(parsed["data"].is_array());
    assert_eq!(parsed["data"].as_array().unwrap().len(), 1);
    // Each element has success/data structure (G.TOOLREQLIXD.4)
    assert_eq!(parsed["data"][0]["success"], true);
    assert_eq!(parsed["data"][0]["data"]["index"], "G.C.1");
}

/// Test: batch delete_requirement with multiple elements (G.TOOLREQLIXD.3)
#[test]
fn test_batch_delete_requirement_multiple_elements() {
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

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.1".to_string(), "G.C.2".to_string()]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 2);
    // Each element has success/data structure (G.TOOLREQLIXD.4)
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[1]["success"], true);

    // Verify file content
    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(!file_content.contains("G.C.1"));
    assert!(!file_content.contains("G.C.2"));
    assert!(file_content.contains("G.C.3"));
}

/// Test: batch delete_requirement processes all elements (G.TOOLREQLIXD.3)
#[test]
fn test_batch_delete_requirement_processes_all() {
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

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(),
            "G.C.999".to_string(), // Does not exist
            "G.C.2".to_string(),
        ]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Top-level success is true (G.TOOLREQLIXD.4)
    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 3);
    // First element: success
    assert_eq!(data[0]["success"], true);
    // Second element: error
    assert_eq!(data[1]["success"], false);
    assert!(data[1]["error"].as_str().unwrap().contains("not found"));
    // Third element: success (all elements processed)
    assert_eq!(data[2]["success"], true);

    // Both existing requirements should be deleted
    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(!file_content.contains("G.C.1"));
    assert!(!file_content.contains("G.C.2"));
}

/// Test: batch delete_requirement exceeds limit (G.TOOLREQLIXD.6)
#[test]
fn test_batch_delete_requirement_exceeds_limit() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let indices: Vec<String> = (1..=101).map(|i| format!("G.C.{}", i)).collect();
    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(indices),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("100"));
}
