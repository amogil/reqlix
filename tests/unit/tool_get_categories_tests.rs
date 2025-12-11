// Tests for Tool: reqlix_get_categories (G.REQLIX_GET_CA.*)
// Covers Requirements: G.REQLIX_GET_CA.1, G.REQLIX_GET_CA.3

use reqlix::RequirementsServer;
use tempfile::TempDir;

use super::common::{create_agents_file, create_category_file};

// =============================================================================
// Tests for reqlix_get_categories (G.REQLIX_GET_CA.*)
// =============================================================================

/// Test: reqlix_get_categories returns all categories
/// Precondition: System has multiple category files
/// Action: Call reqlix_get_categories
/// Result: Function returns sorted list of categories
/// Covers Requirement: G.REQLIX_GET_CA.1, G.REQLIX_GET_CA.3
#[test]
fn test_get_categories_multiple() {
    let temp_dir = TempDir::new().unwrap();
    create_category_file(&temp_dir, "testing", "");
    create_category_file(&temp_dir, "general", "");
    create_category_file(&temp_dir, "deployment", "");
    create_agents_file(&temp_dir, "");

    let categories = RequirementsServer::list_categories(&temp_dir.path().to_path_buf()).unwrap();
    assert_eq!(categories.len(), 3);
    assert_eq!(categories[0], "deployment");
    assert_eq!(categories[1], "general");
    assert_eq!(categories[2], "testing");
}

/// Test: reqlix_get_categories returns empty array when no categories
/// Precondition: System has no category files (only AGENTS.md)
/// Action: Call reqlix_get_categories
/// Result: Function returns empty array
/// Covers Requirement: G.REQLIX_GET_CA.1, G.REQLIX_GET_CA.3
#[test]
fn test_get_categories_empty() {
    let temp_dir = TempDir::new().unwrap();
    create_agents_file(&temp_dir, "");

    let categories = RequirementsServer::list_categories(&temp_dir.path().to_path_buf()).unwrap();
    assert_eq!(categories.len(), 0);
}
