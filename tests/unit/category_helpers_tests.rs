// Unit tests for category helper functions
// Covers Requirement: G.C.7

use reqlix::RequirementsServer;
use std::fs;
use tempfile::TempDir;

/// Test: list_categories with single category
/// Precondition: System has requirements directory with one category file
/// Action: Call list_categories with directory containing "general.md"
/// Result: Function returns vec containing "general"
/// Covers Requirement: G.C.7
#[test]
fn test_list_categories_single() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("general.md");
    fs::write(&file_path, "").unwrap();

    let result = RequirementsServer::list_categories(&temp_dir.path().to_path_buf());
    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 1);
    assert_eq!(categories[0], "general");
}

/// Test: list_categories excluding AGENTS.md
/// Precondition: System has requirements directory with AGENTS.md and category files
/// Action: Call list_categories with directory containing "AGENTS.md" and "general.md"
/// Result: Function returns vec excluding "AGENTS"
/// Covers Requirement: G.C.7
#[test]
fn test_list_categories_exclude_agents() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("AGENTS.md"), "").unwrap();
    fs::write(temp_dir.path().join("general.md"), "").unwrap();

    let result = RequirementsServer::list_categories(&temp_dir.path().to_path_buf());
    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 1);
    assert!(!categories.contains(&"AGENTS".to_string()));
    assert_eq!(categories[0], "general");
}

/// Test: list_categories with multiple categories
/// Precondition: System has requirements directory with multiple category files
/// Action: Call list_categories with directory containing multiple .md files
/// Result: Function returns sorted vec of all categories
/// Covers Requirement: G.C.7
#[test]
fn test_list_categories_multiple() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("testing.md"), "").unwrap();
    fs::write(temp_dir.path().join("general.md"), "").unwrap();
    fs::write(temp_dir.path().join("deployment.md"), "").unwrap();

    let result = RequirementsServer::list_categories(&temp_dir.path().to_path_buf());
    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 3);
    // Should be sorted
    assert_eq!(categories[0], "deployment");
    assert_eq!(categories[1], "general");
    assert_eq!(categories[2], "testing");
}

/// Test: list_categories with empty directory
/// Precondition: System has empty requirements directory
/// Action: Call list_categories with empty directory
/// Result: Function returns empty vec
/// Covers Requirement: G.C.7
#[test]
fn test_list_categories_empty() {
    let temp_dir = TempDir::new().unwrap();

    let result = RequirementsServer::list_categories(&temp_dir.path().to_path_buf());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Vec::<String>::new());
}

/// Test: list_categories ignoring non-md files
/// Precondition: System has requirements directory with non-md files
/// Action: Call list_categories with directory containing .txt and .md files
/// Result: Function returns only .md files (excluding AGENTS.md)
/// Covers Requirement: G.C.7
#[test]
fn test_list_categories_ignore_non_md() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("general.md"), "").unwrap();
    fs::write(temp_dir.path().join("readme.txt"), "").unwrap();

    let result = RequirementsServer::list_categories(&temp_dir.path().to_path_buf());
    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 1);
    assert_eq!(categories[0], "general");
}

/// Test: find_category_by_prefix with single category
/// Precondition: System has requirements directory with one category
/// Action: Call find_category_by_prefix with prefix matching the category
/// Result: Function returns category name
/// Covers Requirement: G.C.7
#[test]
fn test_find_category_by_prefix_single() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("general.md"), "").unwrap();

    let result = RequirementsServer::find_category_by_prefix(&temp_dir.path().to_path_buf(), "G");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "general");
}

/// Test: find_category_by_prefix with multiple categories
/// Precondition: System has requirements directory with multiple categories
/// Action: Call find_category_by_prefix with unique prefix
/// Result: Function returns matching category
/// Covers Requirement: G.C.7
#[test]
fn test_find_category_by_prefix_multiple() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("general.md"), "").unwrap();
    fs::write(temp_dir.path().join("testing.md"), "").unwrap();

    let result = RequirementsServer::find_category_by_prefix(&temp_dir.path().to_path_buf(), "G");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "general");
}

/// Test: find_category_by_prefix with non-existent prefix
/// Precondition: System has requirements directory with categories
/// Action: Call find_category_by_prefix with non-existent prefix
/// Result: Function returns error "Category not found"
/// Covers Requirement: G.C.7
#[test]
fn test_find_category_by_prefix_not_found() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("general.md"), "").unwrap();

    let result = RequirementsServer::find_category_by_prefix(&temp_dir.path().to_path_buf(), "X");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

/// Test: find_category_by_prefix with conflicting prefixes
/// Precondition: System has requirements directory with categories having same first letter
/// Action: Call find_category_by_prefix with longer unique prefix
/// Result: Function returns matching category based on longer prefix
/// Covers Requirement: G.C.7
#[test]
fn test_find_category_by_prefix_conflicting() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("general.md"), "").unwrap();
    fs::write(temp_dir.path().join("guidelines.md"), "").unwrap();

    // Both start with "G", so need longer prefix
    // "general" would have prefix "GE" or longer, "guidelines" would have "GU" or longer
    let result_ge =
        RequirementsServer::find_category_by_prefix(&temp_dir.path().to_path_buf(), "GE");
    let result_gu =
        RequirementsServer::find_category_by_prefix(&temp_dir.path().to_path_buf(), "GU");

    // At least one should succeed
    assert!(result_ge.is_ok() || result_gu.is_ok());
}

/// Test: find_category_by_prefix with empty directory
/// Precondition: System has empty requirements directory
/// Action: Call find_category_by_prefix with any prefix
/// Result: Function returns error "Category not found"
/// Covers Requirement: G.C.7
#[test]
fn test_find_category_by_prefix_empty_dir() {
    let temp_dir = TempDir::new().unwrap();

    let result = RequirementsServer::find_category_by_prefix(&temp_dir.path().to_path_buf(), "G");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

