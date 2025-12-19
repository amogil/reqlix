// Tests for Tool: reqlix_get_categories (T.REQLIXGETC.*)
// Covers Requirements: T.REQLIXGETC.1, T.REQLIXGETC.2, T.REQLIXGETC.3

use reqlix::RequirementsServer;
use tempfile::TempDir;

use super::common::{create_agents_file_in_req_dir, create_category_file_in_req_dir, create_requirements_dir};

// =============================================================================
// Tests for reqlix_get_categories (T.REQLIXGETC.*)
// =============================================================================

/// Test: reqlix_get_categories returns all categories through handler
/// Precondition: System has multiple category files
/// Action: Call handle_get_categories
/// Result: Function returns sorted list of categories in JSON format
/// Covers Requirement: T.REQLIXGETC.1, T.REQLIXGETC.3
#[test]
fn test_get_categories_multiple() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "testing", "");
    create_category_file_in_req_dir(&req_dir, "general", "");
    create_category_file_in_req_dir(&req_dir, "deployment", "");

    let params = reqlix::GetCategoriesParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test get categories".to_string(),
    };

    let result = RequirementsServer::handle_get_categories(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true, "Get categories should succeed: {}", result);
    assert!(parsed["data"]["categories"].is_array());
    let categories = parsed["data"]["categories"].as_array().unwrap();
    assert_eq!(categories.len(), 3);
    assert_eq!(categories[0], "deployment");
    assert_eq!(categories[1], "general");
    assert_eq!(categories[2], "testing");
}

/// Test: reqlix_get_categories returns empty array when no categories
/// Precondition: System has no category files (only AGENTS.md)
/// Action: Call handle_get_categories
/// Result: Function returns empty array in JSON format
/// Covers Requirement: T.REQLIXGETC.1, T.REQLIXGETC.3
#[test]
fn test_get_categories_empty() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");

    let params = reqlix::GetCategoriesParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test get categories".to_string(),
    };

    let result = RequirementsServer::handle_get_categories(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert!(parsed["data"]["categories"].is_array());
    let categories = parsed["data"]["categories"].as_array().unwrap();
    assert_eq!(categories.len(), 0);
}

// =============================================================================
// Tests for T.REQLIXGETC.2: Parameters
// =============================================================================

/// Test: reqlix_get_categories validates project_root parameter (T.REQLIXGETC.2)
/// Precondition: System has empty project_root
/// Action: Call handle_get_categories with empty project_root
/// Result: Function returns validation error
/// Covers Requirement: T.REQLIXGETC.2, G.P.1, G.P.2
#[test]
fn test_get_categories_validates_project_root() {
    let params = reqlix::GetCategoriesParams {
        project_root: "".to_string(),
        operation_description: "Test".to_string(),
    };

    let result = RequirementsServer::handle_get_categories(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("project_root"));
}

/// Test: reqlix_get_categories validates operation_description parameter (T.REQLIXGETC.2)
/// Precondition: System has empty operation_description
/// Action: Call handle_get_categories with empty operation_description
/// Result: Function returns validation error
/// Covers Requirement: T.REQLIXGETC.2, G.P.1, G.P.2
#[test]
fn test_get_categories_validates_operation_description() {
    let temp_dir = TempDir::new().unwrap();
    let params = reqlix::GetCategoriesParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "".to_string(),
    };

    let result = RequirementsServer::handle_get_categories(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("operation_description"));
}
