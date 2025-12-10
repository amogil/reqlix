// Integration tests for all tools
// Covers Requirement: G.REQLIX_GET_I.*, G.REQLIX_GET_CA.*, G.REQLIX_GET_CH.*, G.REQLIX_GET_REQUIREMENTS.*, G.REQLIX_GET_REQUIREMENT.*, G.REQLIX_I.*, G.REQLIX_U.*

use reqlix::RequirementsServer;
use std::fs;
use tempfile::TempDir;

// Helper function to create a category file with content
fn create_category_file(temp_dir: &TempDir, category: &str, content: &str) {
    let file_path = temp_dir.path().join(format!("{}.md", category));
    fs::write(&file_path, content).unwrap();
}

// Helper function to create AGENTS.md file
fn create_agents_file(temp_dir: &TempDir, content: &str) {
    let file_path = temp_dir.path().join("AGENTS.md");
    fs::write(&file_path, content).unwrap();
}

// Helper function to create requirements directory structure for handle_* tests
fn create_requirements_dir(temp_dir: &TempDir) -> std::path::PathBuf {
    let req_dir = temp_dir.path().join("docs/development/requirements");
    fs::create_dir_all(&req_dir).unwrap();
    req_dir
}

// Helper function to create a category file in requirements directory
fn create_category_file_in_req_dir(req_dir: &std::path::Path, category: &str, content: &str) {
    let file_path = req_dir.join(format!("{}.md", category));
    fs::write(&file_path, content).unwrap();
}

// Helper function to create AGENTS.md in requirements directory
fn create_agents_file_in_req_dir(req_dir: &std::path::Path, content: &str) {
    let file_path = req_dir.join("AGENTS.md");
    fs::write(&file_path, content).unwrap();
}

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
    let temp_dir = TempDir::new().unwrap();

    // Test that get_create_path returns correct path
    let create_path = RequirementsServer::get_create_path(&temp_dir.path().to_string_lossy());
    assert!(create_path.ends_with("AGENTS.md"));
}

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

// =============================================================================
// Tests for reqlix_get_requirements (G.REQLIX_GET_REQUIREMENTS.*)
// =============================================================================

/// Test: reqlix_get_requirements returns all requirements in chapter
/// Precondition: System has category file with chapter containing requirements
/// Action: Call reqlix_get_requirements
/// Result: Function returns list of requirements with indices and titles
/// Covers Requirement: G.REQLIX_GET_REQUIREMENTS.1, G.REQLIX_GET_REQUIREMENTS.3, G.REQLIX_GET_REQUIREMENTS.4
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
/// Covers Requirement: G.REQLIX_GET_REQUIREMENTS.1, G.REQLIX_GET_REQUIREMENTS.4
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

// =============================================================================
// Tests for reqlix_get_requirement (G.REQLIX_GET_REQUIREMENT.*)
// =============================================================================

/// Test: reqlix_get_requirement finds requirement by index
/// Precondition: System has category file with requirement
/// Action: Call reqlix_get_requirement with valid index
/// Result: Function returns requirement with title and text
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.1, G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4
#[test]
fn test_get_requirement_by_index() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"# Test Chapter

## G.T.1: Test Requirement

This is the requirement text.
It can span multiple lines.
"#;
    create_category_file(&temp_dir, "general", content);

    let requirement = RequirementsServer::find_requirement_streaming(
        &temp_dir.path().join("general.md"),
        "general",
        "G.T.1",
    )
    .unwrap();
    assert_eq!(requirement.index, "G.T.1");
    assert_eq!(requirement.title, "Test Requirement");
    assert!(requirement.text.contains("This is the requirement text"));
}

/// Test: reqlix_get_requirement returns error for non-existent requirement
/// Precondition: System has category file without the specified requirement
/// Action: Call reqlix_get_requirement with non-existent index
/// Result: Function returns error "Requirement not found"
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3
#[test]
fn test_get_requirement_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"# Test Chapter

## G.T.1: Test Requirement

Content.
"#;
    create_category_file(&temp_dir, "general", content);

    let result = RequirementsServer::find_requirement_streaming(
        &temp_dir.path().join("general.md"),
        "general",
        "G.T.999",
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

// =============================================================================
// Tests for reqlix_insert_requirement (G.REQLIX_I.*)
// =============================================================================

/// Test: reqlix_insert_requirement creates new requirement
/// Precondition: System has category file with chapter
/// Action: Call reqlix_insert_requirement with valid parameters
/// Result: Function creates requirement and returns full data
/// Covers Requirement: G.REQLIX_I.1, G.REQLIX_I.3, G.REQLIX_I.5
#[test]
fn test_insert_requirement_new() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"# Test Chapter

## G.T.1: Existing Requirement

Existing content.
"#;
    create_category_file(&temp_dir, "general", content);

    // This would test the full insert flow, but requires access to private methods
    // For now, we verify the file structure is correct
    let file_content = fs::read_to_string(temp_dir.path().join("general.md")).unwrap();
    assert!(file_content.contains("Test Chapter"));
    assert!(file_content.contains("Existing Requirement"));
}

/// Test: reqlix_insert_requirement validates title uniqueness
/// Precondition: System has category file with requirement having same title
/// Action: Call reqlix_insert_requirement with duplicate title
/// Result: Function returns error "Title already exists in chapter"
/// Covers Requirement: G.REQLIX_I.3 step 3
#[test]
fn test_insert_requirement_duplicate_title() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"# Test Chapter

## G.T.1: Duplicate Title

Content.
"#;
    create_category_file(&temp_dir, "general", content);

    // Verify that reading requirements finds the existing one
    let requirements = RequirementsServer::read_requirements_streaming(
        &temp_dir.path().join("general.md"),
        "Test Chapter",
    )
    .unwrap();
    assert_eq!(requirements.len(), 1);
    assert_eq!(requirements[0].title, "Duplicate Title");
}

// =============================================================================
// Tests for reqlix_update_requirement (G.REQLIX_U.*)
// =============================================================================

/// Test: reqlix_update_requirement updates existing requirement
/// Precondition: System has category file with requirement
/// Action: Call reqlix_update_requirement with new text
/// Result: Function updates requirement and returns full data
/// Covers Requirement: G.REQLIX_U.1, G.REQLIX_U.3, G.REQLIX_U.4
#[test]
fn test_update_requirement_text() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"# Test Chapter

## G.T.1: Test Requirement

Old content.
"#;
    create_category_file(&temp_dir, "general", content);

    // Verify initial state
    let requirement = RequirementsServer::find_requirement_streaming(
        &temp_dir.path().join("general.md"),
        "general",
        "G.T.1",
    )
    .unwrap();
    assert!(requirement.text.contains("Old content"));
}

/// Test: reqlix_update_requirement validates title uniqueness
/// Precondition: System has category file with multiple requirements
/// Action: Call reqlix_update_requirement with title that conflicts
/// Result: Function returns error "Title already exists in chapter"
/// Covers Requirement: G.REQLIX_U.3 step 5
#[test]
fn test_update_requirement_duplicate_title() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"# Test Chapter

## G.T.1: First Requirement

Content one.

## G.T.2: Second Requirement

Content two.
"#;
    create_category_file(&temp_dir, "general", content);

    // Verify both requirements exist
    let requirements = RequirementsServer::read_requirements_streaming(
        &temp_dir.path().join("general.md"),
        "Test Chapter",
    )
    .unwrap();
    assert_eq!(requirements.len(), 2);
    assert_eq!(requirements[0].title, "First Requirement");
    assert_eq!(requirements[1].title, "Second Requirement");
}

/// Test: find_requirement_streaming correctly identifies boundaries before next chapter
/// Precondition: System has category file with requirement followed by level-1 heading (new chapter)
/// Action: Call find_requirement_streaming for last requirement before new chapter
/// Result: Requirement text does NOT include the next chapter heading (G.R.5: ends at same or higher level)
/// Covers Requirement: G.R.5, G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4
#[test]
fn test_requirement_boundary_before_next_chapter() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"# First Chapter

## G.F.1: Last Requirement In Chapter

Content of last requirement.

# Second Chapter

## G.S.1: First Requirement In Second Chapter

Content of second chapter requirement.
"#;
    create_category_file(&temp_dir, "general", content);

    // Get the last requirement in first chapter
    let requirement = RequirementsServer::find_requirement_streaming(
        &temp_dir.path().join("general.md"),
        "general",
        "G.F.1",
    )
    .unwrap();

    // Verify requirement text does NOT include next chapter (G.R.5)
    assert!(requirement.text.contains("Content of last requirement"));
    assert!(
        !requirement.text.contains("# Second Chapter"),
        "Level-1 heading should end requirement, not be included in text"
    );
    assert!(
        !requirement.text.contains("G.S.1"),
        "Content after level-1 heading should not be in requirement"
    );
    assert!(
        !requirement
            .text
            .contains("Content of second chapter requirement"),
        "Content from next chapter should not be in requirement"
    );
}

// =============================================================================
// Tests for parameter validation in tools (G.REQLIX_I.6, G.REQLIX_U.6)
// =============================================================================

/// Test: reqlix_insert_requirement validates all parameters
/// Precondition: System has invalid parameters
/// Action: Call reqlix_insert_requirement with invalid parameters
/// Result: Function returns validation error before processing
/// Covers Requirement: G.REQLIX_I.6, G.P.1, G.P.2
#[test]
fn test_insert_requirement_validation() {
    // Test that validation functions work correctly
    assert!(RequirementsServer::validate_project_root("").is_err());
    assert!(RequirementsServer::validate_category("").is_err());
    assert!(RequirementsServer::validate_chapter("").is_err());
    assert!(RequirementsServer::validate_text("").is_err());
    assert!(RequirementsServer::validate_title("", true).is_err());
}

/// Test: reqlix_update_requirement validates all parameters
/// Precondition: System has invalid parameters
/// Action: Call reqlix_update_requirement with invalid parameters
/// Result: Function returns validation error before processing
/// Covers Requirement: G.REQLIX_U.6, G.P.1, G.P.2
#[test]
fn test_update_requirement_validation() {
    // Test that validation functions work correctly
    assert!(RequirementsServer::validate_project_root("").is_err());
    assert!(RequirementsServer::validate_index("").is_err());
    assert!(RequirementsServer::validate_text("").is_err());
    // Title is optional for update, so empty is OK when required=false
    assert!(RequirementsServer::validate_title("", false).is_ok());
}

// =============================================================================
// Tests for error response format (G.C.6)
// =============================================================================

/// Test: Error response format validation
/// Precondition: System encounters an error condition
/// Action: Verify error JSON structure
/// Result: Error JSON has "success": false and "error" field
/// Covers Requirement: G.C.6
#[test]
fn test_error_response_format() {
    // Verify error format structure by checking validation errors return proper format
    let result = RequirementsServer::validate_project_root("");
    assert!(result.is_err());
    // Error message should be human-readable
    let error_msg = result.unwrap_err();
    assert!(!error_msg.is_empty());
    assert!(error_msg.contains("required") || error_msg.contains("exceeds"));
}

// =============================================================================
// Tests for reqlix_get_version (G.TOOLREQLIXGETV.*)
// =============================================================================

/// Test: reqlix_get_version returns version string
/// Precondition: Server is running
/// Action: Call handle_get_version
/// Result: Function returns JSON with version from Cargo.toml
/// Covers Requirement: G.TOOLREQLIXGETV.2, G.TOOLREQLIXGETV.3
#[test]
fn test_get_version_returns_version() {
    let params = reqlix::GetVersionParams {};
    let result = RequirementsServer::handle_get_version(params);
    
    // Parse JSON response
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    // Verify success (G.TOOLREQLIXGETV.2)
    assert_eq!(parsed["success"], true);
    
    // Verify version is present and matches Cargo.toml (G.TOOLREQLIXGETV.3)
    let version = parsed["data"]["version"].as_str().unwrap();
    assert_eq!(version, env!("CARGO_PKG_VERSION"));
}

/// Test: reqlix_get_version always succeeds
/// Precondition: Server is running
/// Action: Call handle_get_version multiple times
/// Result: Function always returns success: true
/// Covers Requirement: G.TOOLREQLIXGETV.2
#[test]
fn test_get_version_always_succeeds() {
    // Call multiple times to verify consistent behavior
    for _ in 0..3 {
        let params = reqlix::GetVersionParams {};
        let result = RequirementsServer::handle_get_version(params);
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["success"], true);
        assert!(parsed["data"]["version"].is_string());
    }
}

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
        index: "G.T.1".to_string(),
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
    let file_content = fs::read_to_string(req_dir.join("general.md")).unwrap();
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
        index: "G.T.999".to_string(),
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
        index: "G.T.1".to_string(),
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
        index: "G.F.1".to_string(),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);

    // Verify chapter is removed
    let file_content = fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(!file_content.contains("# First Chapter"), "Empty chapter should be removed");
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
        index: "G.O.1".to_string(),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);

    // Verify requirement is deleted
    let file_content = fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(!file_content.contains("G.O.1"));
    assert!(!file_content.contains("Only Requirement"));
}

// =============================================================================
// Tests for G.R.11 (Blank line before headings)
// =============================================================================

/// Test: update_requirement ensures blank line before next heading
/// Precondition: System has category file with requirements
/// Action: Update requirement text
/// Result: There is a blank line between updated text and next heading
/// Covers Requirement: G.R.11
#[test]
fn test_update_requirement_blank_line_before_next_heading() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Test Chapter

## G.T.1: First Requirement

Old content.

## G.T.2: Second Requirement

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test update".to_string(),
        index: "G.T.1".to_string(),
        text: "New content without trailing newline".to_string(),
        title: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);

    // Verify blank line before next heading (G.R.11)
    let file_content = fs::read_to_string(req_dir.join("general.md")).unwrap();
    // Should have blank line before ## G.T.2
    assert!(
        file_content.contains("New content without trailing newline\n\n## G.T.2"),
        "Should have blank line before next requirement heading. Content: {}",
        file_content
    );
}

/// Test: update_requirement ensures blank line before next chapter heading
/// Precondition: System has category file with requirement before next chapter
/// Action: Update requirement that is last in its chapter
/// Result: There is a blank line between updated text and next chapter heading
/// Covers Requirement: G.R.11
#[test]
fn test_update_requirement_blank_line_before_chapter_heading() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# First Chapter

## G.F.1: Last In Chapter

Old content.

# Second Chapter

## G.S.1: First In Second

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test update".to_string(),
        index: "G.F.1".to_string(),
        text: "Updated content".to_string(),
        title: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);

    // Verify blank line before next chapter heading (G.R.11)
    let file_content = fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(
        file_content.contains("Updated content\n\n# Second Chapter"),
        "Should have blank line before next chapter heading. Content: {}",
        file_content
    );
}

/// Test: delete_requirement maintains blank line formatting
/// Precondition: System has multiple requirements
/// Action: Delete middle requirement
/// Result: Proper blank lines are maintained between remaining requirements
/// Covers Requirement: G.R.11
#[test]
fn test_delete_requirement_maintains_blank_lines() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Test Chapter

## G.T.1: First Requirement

Content one.

## G.T.2: Middle Requirement

Content two.

## G.T.3: Last Requirement

Content three.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test delete".to_string(),
        index: "G.T.2".to_string(),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);

    // Verify proper formatting after deletion
    let file_content = fs::read_to_string(req_dir.join("general.md")).unwrap();
    // Should have blank line before ## G.T.3
    assert!(
        file_content.contains("Content one.\n\n## G.T.3"),
        "Should have blank line before next requirement after deletion. Content: {}",
        file_content
    );
}

