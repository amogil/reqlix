// Tests for Tool: reqlix_get_chapters (G.REQLIX_GET_CH.*)
// Covers Requirements: G.REQLIX_GET_CH.1, G.REQLIX_GET_CH.3, G.REQLIX_GET_CH.4

use reqlix::RequirementsServer;
use tempfile::TempDir;

use super::common::create_category_file;

// =============================================================================
// Tests for reqlix_get_chapters (G.REQLIX_GET_CH.*)
// =============================================================================

/// Test: reqlix_get_chapters returns all chapters in category
/// Precondition: System has category file with multiple chapters
/// Action: Call reqlix_get_chapters
/// Result: Function returns list of chapter names
/// Covers Requirement: G.REQLIX_GET_CH.1, G.REQLIX_GET_CH.3, G.REQLIX_GET_CH.4
#[test]
fn test_get_chapters_multiple() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"# Chapter One

Content of chapter one.

# Chapter Two

Content of chapter two.
"#;
    create_category_file(&temp_dir, "general", content);

    let chapters =
        RequirementsServer::read_chapters_streaming(&temp_dir.path().join("general.md")).unwrap();
    assert_eq!(chapters.len(), 2);
    assert_eq!(chapters[0], "Chapter One");
    assert_eq!(chapters[1], "Chapter Two");
}

/// Test: reqlix_get_chapters returns empty array when no chapters
/// Precondition: System has category file with no chapters
/// Action: Call reqlix_get_chapters
/// Result: Function returns empty array
/// Covers Requirement: G.REQLIX_GET_CH.1, G.REQLIX_GET_CH.4
#[test]
fn test_get_chapters_empty() {
    let temp_dir = TempDir::new().unwrap();
    create_category_file(&temp_dir, "general", "");

    let chapters =
        RequirementsServer::read_chapters_streaming(&temp_dir.path().join("general.md")).unwrap();
    assert_eq!(chapters.len(), 0);
}
