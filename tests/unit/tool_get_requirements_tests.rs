// Tests for Tool: reqlix_get_requirements (T.REQLIXGETR.*)
// Covers Requirements: T.REQLIXGETR.1, T.REQLIXGETR.3, T.REQLIXGETR.4

use reqlix::RequirementsServer;
use tempfile::TempDir;

use super::common::create_category_file;

// =============================================================================
// Tests for reqlix_get_requirements (T.REQLIXGETR.*)
// =============================================================================

/// Test: reqlix_get_requirements returns all requirements in chapter
/// Precondition: System has category file with chapter containing requirements
/// Action: Call reqlix_get_requirements
/// Result: Function returns list of requirements with indices and titles
/// Covers Requirement: T.REQLIXGETR.1, T.REQLIXGETR.3, T.REQLIXGETR.4
#[test]
fn test_get_requirements_multiple() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"# Test Chapter

## G.T.1: First Requirement

Content of first requirement.

## G.T.2: Second Requirement

Content of second requirement.
"#;
    create_category_file(&temp_dir, "general", content);

    let requirements = RequirementsServer::read_requirements_streaming(
        &temp_dir.path().join("general.md"),
        "Test Chapter",
    )
    .unwrap();
    assert_eq!(requirements.len(), 2);
    assert_eq!(requirements[0].index, "G.T.1");
    assert_eq!(requirements[0].title, "First Requirement");
    assert_eq!(requirements[1].index, "G.T.2");
    assert_eq!(requirements[1].title, "Second Requirement");
}

/// Test: reqlix_get_requirements returns empty array when no requirements
/// Precondition: System has chapter with no requirements
/// Action: Call reqlix_get_requirements
/// Result: Function returns empty array
/// Covers Requirement: T.REQLIXGETR.1, T.REQLIXGETR.4
#[test]
fn test_get_requirements_empty() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"# Test Chapter

No requirements here.
"#;
    create_category_file(&temp_dir, "general", content);

    let requirements = RequirementsServer::read_requirements_streaming(
        &temp_dir.path().join("general.md"),
        "Test Chapter",
    )
    .unwrap();
    assert_eq!(requirements.len(), 0);
}
