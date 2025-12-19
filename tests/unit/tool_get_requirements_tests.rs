// Tests for Tool: reqlix_get_requirements (T.REQLIXGETR.*)
// Covers Requirements: T.REQLIXGETR.1, T.REQLIXGETR.2, T.REQLIXGETR.3, T.REQLIXGETR.4

use reqlix::RequirementsServer;
use tempfile::TempDir;

use super::common::{create_category_file_in_req_dir, create_requirements_dir, create_agents_file_in_req_dir};

// =============================================================================
// Tests for reqlix_get_requirements (T.REQLIXGETR.*)
// =============================================================================

/// Test: reqlix_get_requirements returns all requirements in chapter through handler
/// Precondition: System has category file with chapter containing requirements
/// Action: Call handle_get_requirements
/// Result: Function returns list of requirements with indices and titles in JSON format
/// Covers Requirement: T.REQLIXGETR.1, T.REQLIXGETR.3, T.REQLIXGETR.4
#[test]
fn test_get_requirements_multiple() {
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

    let params = reqlix::GetRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test get requirements".to_string(),
        category: "general".to_string(),
        chapter: "Test Chapter".to_string(),
    };

    let result = RequirementsServer::handle_get_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true, "Get requirements should succeed: {}", result);
    assert_eq!(parsed["data"]["category"], "general");
    assert_eq!(parsed["data"]["chapter"], "Test Chapter");
    assert!(parsed["data"]["requirements"].is_array());
    let requirements = parsed["data"]["requirements"].as_array().unwrap();
    assert_eq!(requirements.len(), 2);
    assert_eq!(requirements[0]["index"], "G.T.1");
    assert_eq!(requirements[0]["title"], "First Requirement");
    assert_eq!(requirements[1]["index"], "G.T.2");
    assert_eq!(requirements[1]["title"], "Second Requirement");
}

/// Test: reqlix_get_requirements returns empty array when no requirements
/// Precondition: System has chapter with no requirements
/// Action: Call handle_get_requirements
/// Result: Function returns empty array in JSON format
/// Covers Requirement: T.REQLIXGETR.1, T.REQLIXGETR.4
#[test]
fn test_get_requirements_empty() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Test Chapter

No requirements here.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::GetRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test get requirements".to_string(),
        category: "general".to_string(),
        chapter: "Test Chapter".to_string(),
    };

    let result = RequirementsServer::handle_get_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["data"]["category"], "general");
    assert_eq!(parsed["data"]["chapter"], "Test Chapter");
    assert!(parsed["data"]["requirements"].is_array());
    let requirements = parsed["data"]["requirements"].as_array().unwrap();
    assert_eq!(requirements.len(), 0);
}

/// Test: reqlix_get_requirements returns error for non-existent category
/// Precondition: System has no category file with specified name
/// Action: Call handle_get_requirements with non-existent category
/// Result: Function returns error "Category not found" in JSON format
/// Covers Requirement: T.REQLIXGETR.3
#[test]
fn test_get_requirements_category_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");

    let params = reqlix::GetRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test get requirements".to_string(),
        category: "nonexistent".to_string(),
        chapter: "Chapter".to_string(),
    };

    let result = RequirementsServer::handle_get_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false, "Get requirements should fail for non-existent category");
    assert!(parsed["error"]
        .as_str()
        .unwrap()
        .contains("not found"));
}

/// Test: reqlix_get_requirements returns error for non-existent chapter
/// Precondition: System has category file but without specified chapter
/// Action: Call handle_get_requirements with non-existent chapter
/// Result: Function returns error "Chapter not found" in JSON format
/// Covers Requirement: T.REQLIXGETR.3
#[test]
fn test_get_requirements_chapter_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Existing Chapter

## G.E.1: Requirement

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::GetRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test get requirements".to_string(),
        category: "general".to_string(),
        chapter: "Nonexistent Chapter".to_string(),
    };

    let result = RequirementsServer::handle_get_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false, "Get requirements should fail for non-existent chapter");
    assert!(parsed["error"]
        .as_str()
        .unwrap()
        .contains("not found"));
}
