// Tests for Tool: reqlix_get_chapters (T.REQLIXGETCH.*)
// Covers Requirements: T.REQLIXGETCH.1, T.REQLIXGETCH.2, T.REQLIXGETCH.3, T.REQLIXGETCH.4

use reqlix::RequirementsServer;
use tempfile::TempDir;

use super::common::{create_category_file_in_req_dir, create_requirements_dir, create_agents_file_in_req_dir};

// =============================================================================
// Tests for reqlix_get_chapters (T.REQLIXGETCH.*)
// =============================================================================

/// Test: reqlix_get_chapters returns all chapters in category through handler
/// Precondition: System has category file with multiple chapters
/// Action: Call handle_get_chapters
/// Result: Function returns list of chapter names in JSON format
/// Covers Requirement: T.REQLIXGETCH.1, T.REQLIXGETCH.3, T.REQLIXGETCH.4
#[test]
fn test_get_chapters_multiple() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter One

Content of chapter one.

# Chapter Two

Content of chapter two.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::GetChaptersParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test get chapters".to_string(),
        category: "general".to_string(),
    };

    let result = RequirementsServer::handle_get_chapters(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true, "Get chapters should succeed: {}", result);
    assert_eq!(parsed["data"]["category"], "general");
    assert!(parsed["data"]["chapters"].is_array());
    let chapters = parsed["data"]["chapters"].as_array().unwrap();
    assert_eq!(chapters.len(), 2);
    assert_eq!(chapters[0], "Chapter One");
    assert_eq!(chapters[1], "Chapter Two");
}

/// Test: reqlix_get_chapters returns empty array when no chapters
/// Precondition: System has category file with no chapters
/// Action: Call handle_get_chapters
/// Result: Function returns empty array in JSON format
/// Covers Requirement: T.REQLIXGETCH.1, T.REQLIXGETCH.4
#[test]
fn test_get_chapters_empty() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "");

    let params = reqlix::GetChaptersParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test get chapters".to_string(),
        category: "general".to_string(),
    };

    let result = RequirementsServer::handle_get_chapters(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["data"]["category"], "general");
    assert!(parsed["data"]["chapters"].is_array());
    let chapters = parsed["data"]["chapters"].as_array().unwrap();
    assert_eq!(chapters.len(), 0);
}

/// Test: reqlix_get_chapters returns error for non-existent category
/// Precondition: System has no category file with specified name
/// Action: Call handle_get_chapters with non-existent category
/// Result: Function returns error "Category not found" in JSON format
/// Covers Requirement: T.REQLIXGETCH.3
#[test]
fn test_get_chapters_category_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");

    let params = reqlix::GetChaptersParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test get chapters".to_string(),
        category: "nonexistent".to_string(),
    };

    let result = RequirementsServer::handle_get_chapters(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false, "Get chapters should fail for non-existent category");
    assert!(parsed["error"]
        .as_str()
        .unwrap()
        .contains("not found"));
}

// =============================================================================
// Tests for T.REQLIXGETCH.2: Parameters
// =============================================================================

/// Test: reqlix_get_chapters validates project_root parameter (T.REQLIXGETCH.2)
/// Precondition: System has empty project_root
/// Action: Call handle_get_chapters with empty project_root
/// Result: Function returns validation error
/// Covers Requirement: T.REQLIXGETCH.2, G.P.1, G.P.2
#[test]
fn test_get_chapters_validates_project_root() {
    let params = reqlix::GetChaptersParams {
        project_root: "".to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
    };

    let result = RequirementsServer::handle_get_chapters(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("project_root"));
}

/// Test: reqlix_get_chapters validates operation_description parameter (T.REQLIXGETCH.2)
/// Precondition: System has empty operation_description
/// Action: Call handle_get_chapters with empty operation_description
/// Result: Function returns validation error
/// Covers Requirement: T.REQLIXGETCH.2, G.P.1, G.P.2
#[test]
fn test_get_chapters_validates_operation_description() {
    let temp_dir = TempDir::new().unwrap();
    let params = reqlix::GetChaptersParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "".to_string(),
        category: "general".to_string(),
    };

    let result = RequirementsServer::handle_get_chapters(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("operation_description"));
}

/// Test: reqlix_get_chapters validates category parameter (T.REQLIXGETCH.2)
/// Precondition: System has empty category
/// Action: Call handle_get_chapters with empty category
/// Result: Function returns validation error
/// Covers Requirement: T.REQLIXGETCH.2, G.P.1, G.P.2
#[test]
fn test_get_chapters_validates_category() {
    let temp_dir = TempDir::new().unwrap();
    let params = reqlix::GetChaptersParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "".to_string(),
    };

    let result = RequirementsServer::handle_get_chapters(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("category"));
}
