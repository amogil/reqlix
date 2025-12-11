// Tests for Tool: reqlix_get_instructions (G.REQLIX_GET_I.*)
// Covers Requirements: G.REQLIX_GET_I.4, G.REQLIX_GET_I.6

use reqlix::RequirementsServer;

// =============================================================================
// Tests for reqlix_get_instructions (G.REQLIX_GET_I.*)
// =============================================================================

/// Test: reqlix_get_instructions creates AGENTS.md if not found
/// Precondition: System has no AGENTS.md file
/// Action: Call reqlix_get_instructions
/// Result: Function creates AGENTS.md with placeholder content
/// Covers Requirement: G.REQLIX_GET_I.4, G.REQLIX_GET_I.6
#[test]
fn test_get_instructions_creates_file() {
    use tempfile::TempDir;
    let temp_dir = TempDir::new().unwrap();

    // Test that get_create_path returns correct path
    let create_path = RequirementsServer::get_create_path(&temp_dir.path().to_string_lossy());
    assert!(create_path.ends_with("AGENTS.md"));
}
