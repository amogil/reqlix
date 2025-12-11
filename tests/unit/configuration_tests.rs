// Tests for Configuration (G.C.*)
// Covers Requirements: G.C.1, G.C.2, G.C.7

use reqlix::RequirementsServer;
use tempfile::TempDir;

// =============================================================================
// Tests for G.C.1: Requirements directory location
// =============================================================================

// Note: G.C.1 tests are covered in file_system_tests.rs (get_search_paths, get_create_path)
// These tests verify that requirements directory is correctly located

// =============================================================================
// Tests for G.C.2: Directory creation
// =============================================================================

// Note: G.C.2 tests are covered in requirements_storage_format_tests.rs (write_file_utf8_creates_dirs)
// and name_and_file_validation_tests.rs

// =============================================================================
// Tests for G.C.7: Category management
// =============================================================================

/// Test: list_categories with single category
/// Precondition: System has directory with single category file
/// Action: Call list_categories with directory path
/// Result: Function returns Ok with vector containing one category name
/// Covers Requirement: G.C.7
#[test]
fn test_list_categories_single() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("general.md");
    std::fs::write(&file_path, "").unwrap();

    let result = RequirementsServer::list_categories(&temp_dir.path().to_path_buf());
    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 1);
    assert_eq!(categories[0], "general");
}

/// Test: list_categories excluding AGENTS.md
/// Precondition: System has directory with AGENTS.md and category file
/// Action: Call list_categories with directory path
/// Result: Function returns Ok with vector excluding AGENTS (only category files included)
/// Covers Requirement: G.C.7
#[test]
fn test_list_categories_exclude_agents() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::write(temp_dir.path().join("AGENTS.md"), "").unwrap();
    std::fs::write(temp_dir.path().join("general.md"), "").unwrap();

    let result = RequirementsServer::list_categories(&temp_dir.path().to_path_buf());
    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 1);
    assert!(!categories.contains(&"AGENTS".to_string()));
    assert_eq!(categories[0], "general");
}

/// Test: list_categories with multiple categories
/// Precondition: System has directory with multiple category files
/// Action: Call list_categories with directory path
/// Result: Function returns Ok with sorted vector containing all category names
/// Covers Requirement: G.C.7
#[test]
fn test_list_categories_multiple() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::write(temp_dir.path().join("testing.md"), "").unwrap();
    std::fs::write(temp_dir.path().join("general.md"), "").unwrap();
    std::fs::write(temp_dir.path().join("deployment.md"), "").unwrap();

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
/// Precondition: System has empty directory
/// Action: Call list_categories with empty directory path
/// Result: Function returns Ok with empty vector
/// Covers Requirement: G.C.7
#[test]
fn test_list_categories_empty() {
    let temp_dir = TempDir::new().unwrap();

    let result = RequirementsServer::list_categories(&temp_dir.path().to_path_buf());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Vec::<String>::new());
}

/// Test: list_categories ignoring non-md files
/// Precondition: System has directory with .md and non-.md files
/// Action: Call list_categories with directory path
/// Result: Function returns Ok with vector containing only .md files (non-.md files ignored)
/// Covers Requirement: G.C.7
#[test]
fn test_list_categories_ignore_non_md() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::write(temp_dir.path().join("general.md"), "").unwrap();
    std::fs::write(temp_dir.path().join("readme.txt"), "").unwrap();

    let result = RequirementsServer::list_categories(&temp_dir.path().to_path_buf());
    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 1);
    assert_eq!(categories[0], "general");
}

/// Test: find_category_by_prefix with single category
/// Precondition: System has directory with single category file
/// Action: Call find_category_by_prefix with directory path and prefix
/// Result: Function returns Ok with matching category name
/// Covers Requirement: G.C.7
#[test]
fn test_find_category_by_prefix_single() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::write(temp_dir.path().join("general.md"), "").unwrap();

    let result = RequirementsServer::find_category_by_prefix(&temp_dir.path().to_path_buf(), "G");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "general");
}

/// Test: find_category_by_prefix with multiple categories
/// Precondition: System has directory with multiple category files
/// Action: Call find_category_by_prefix with directory path and prefix matching one category
/// Result: Function returns Ok with matching category name
/// Covers Requirement: G.C.7
#[test]
fn test_find_category_by_prefix_multiple() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::write(temp_dir.path().join("general.md"), "").unwrap();
    std::fs::write(temp_dir.path().join("testing.md"), "").unwrap();

    let result = RequirementsServer::find_category_by_prefix(&temp_dir.path().to_path_buf(), "G");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "general");
}

/// Test: find_category_by_prefix with non-existent prefix
/// Precondition: System has directory with category files but no matching prefix
/// Action: Call find_category_by_prefix with directory path and non-matching prefix
/// Result: Function returns error about category not found
/// Covers Requirement: G.C.7
#[test]
fn test_find_category_by_prefix_not_found() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::write(temp_dir.path().join("general.md"), "").unwrap();

    let result = RequirementsServer::find_category_by_prefix(&temp_dir.path().to_path_buf(), "X");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

/// Test: find_category_by_prefix with conflicting prefixes
/// Precondition: System has directory with multiple categories starting with same letter
/// Action: Call find_category_by_prefix with longer prefixes to disambiguate
/// Result: Function returns Ok for at least one matching prefix (longer prefix resolves conflict)
/// Covers Requirement: G.C.7
#[test]
fn test_find_category_by_prefix_conflicting() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::write(temp_dir.path().join("general.md"), "").unwrap();
    std::fs::write(temp_dir.path().join("guidelines.md"), "").unwrap();

    // Both start with "G", so need longer prefix
    let result_ge =
        RequirementsServer::find_category_by_prefix(&temp_dir.path().to_path_buf(), "GE");
    let result_gu =
        RequirementsServer::find_category_by_prefix(&temp_dir.path().to_path_buf(), "GU");

    // At least one should succeed
    assert!(result_ge.is_ok() || result_gu.is_ok());
}

/// Test: find_category_by_prefix with empty directory
/// Precondition: System has empty directory
/// Action: Call find_category_by_prefix with empty directory path and prefix
/// Result: Function returns error about category not found
/// Covers Requirement: G.C.7
#[test]
fn test_find_category_by_prefix_empty_dir() {
    let temp_dir = TempDir::new().unwrap();

    let result = RequirementsServer::find_category_by_prefix(&temp_dir.path().to_path_buf(), "G");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}
