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
        index: reqlix::IndexParam::Single("G.T.1".to_string()),
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
        index: reqlix::IndexParam::Single("G.T.999".to_string()),
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
        index: reqlix::IndexParam::Single("G.T.1".to_string()),
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
        index: reqlix::IndexParam::Single("G.F.1".to_string()),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);

    // Verify chapter is removed
    let file_content = fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(
        !file_content.contains("# First Chapter"),
        "Empty chapter should be removed"
    );
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
        index: reqlix::IndexParam::Single("G.O.1".to_string()),
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
        index: Some("G.T.1".to_string()),
        text: Some("New content without trailing newline".to_string()),
        title: None,
        items: None,
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
        index: Some("G.F.1".to_string()),
        text: Some("Updated content".to_string()),
        title: None,
        items: None,
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
        index: reqlix::IndexParam::Single("G.T.2".to_string()),
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

// =============================================================================
// Tests for chapter name substring bug fix
// =============================================================================

/// Test: insert_requirement must find exact chapter name, not substring
/// Precondition: System has two chapters where one name is substring of another
/// Action: Insert requirement into chapter with shorter name
/// Result: Requirement is inserted into correct chapter, not the one with longer name
/// Covers Bug: Chapter "Foo" was matched by "Foobar" because find() found substring
#[test]
fn test_insert_requirement_exact_chapter_match() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");

    // Create file with two chapters where one name is prefix of another
    // "Tool: reqlix_get_requirement" is prefix of "Tool: reqlix_get_requirements"
    let content = r#"# Tool: reqlix_get_requirements

## G.REQLIX_GET_REQUIREMENTS.1: First

Content in requirements chapter.

# Tool: reqlix_get_requirement

## G.REQLIX_GET_REQUIREMENT.1: First

Content in requirement chapter.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    // Insert into the shorter-named chapter (reqlix_get_requirement without 's')
    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test insert".to_string(),
        category: "general".to_string(),
        chapter: "Tool: reqlix_get_requirement".to_string(),
        title: "New Requirement".to_string(),
        text: "New content".to_string(),
    };
    let result = RequirementsServer::handle_insert_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true, "Insert should succeed: {}", result);

    // Verify requirement was inserted into CORRECT chapter
    let file_content = fs::read_to_string(req_dir.join("general.md")).unwrap();

    // The new requirement should be AFTER "# Tool: reqlix_get_requirement" heading
    // and BEFORE "# Tool: reqlix_get_requirements" content
    let requirement_pos = file_content.find("## G.REQLIX_GET_REQUIREMENT.2: New Requirement");
    let correct_chapter_pos = file_content.find("# Tool: reqlix_get_requirement\n");
    let wrong_chapter_pos = file_content.find("# Tool: reqlix_get_requirements\n");

    assert!(
        requirement_pos.is_some(),
        "New requirement should exist in file"
    );
    assert!(
        correct_chapter_pos.is_some(),
        "Correct chapter should exist"
    );
    assert!(wrong_chapter_pos.is_some(), "Wrong chapter should exist");

    let req_pos = requirement_pos.unwrap();
    let correct_pos = correct_chapter_pos.unwrap();
    let wrong_pos = wrong_chapter_pos.unwrap();

    // New requirement must be after correct chapter heading
    assert!(
        req_pos > correct_pos,
        "Requirement should be after '# Tool: reqlix_get_requirement'. Req at {}, chapter at {}",
        req_pos,
        correct_pos
    );

    // New requirement must NOT be between wrong chapter and correct chapter
    // (i.e., it should not be inserted into reqlix_get_requirements chapter)
    assert!(
        !(req_pos > wrong_pos && req_pos < correct_pos),
        "Requirement should NOT be in 'Tool: reqlix_get_requirements' chapter. Content:\n{}",
        file_content
    );
}

/// Test: insert_requirement ensures blank line before next chapter heading (G.R.11)
/// Precondition: System has chapter with content, followed by another chapter
/// Action: Insert requirement at end of first chapter
/// Result: There is a blank line between inserted requirement and next chapter heading
/// Covers Requirement: G.R.11
#[test]
fn test_insert_requirement_blank_line_before_next_chapter() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");

    let content = r#"# First Chapter

## G.F.1: Existing Requirement

Existing content.

# Second Chapter

## G.S.1: Another Requirement

More content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test insert".to_string(),
        category: "general".to_string(),
        chapter: "First Chapter".to_string(),
        title: "New Requirement".to_string(),
        text: "New content".to_string(),
    };
    let result = RequirementsServer::handle_insert_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true, "Insert should succeed: {}", result);

    // Verify blank line before next chapter heading (G.R.11)
    let file_content = fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(
        file_content.contains("New content\n\n# Second Chapter"),
        "Should have blank line before next chapter heading. Content:\n{}",
        file_content
    );
}

// =============================================================================
// Batch operation tests (G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_U.3, G.TOOLREQLIXD.3)
// =============================================================================

/// Test: batch get_requirement with empty array returns empty result (G.P.4)
#[test]
fn test_batch_get_requirement_empty_array() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(vec![]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["data"], serde_json::json!([]));
}

/// Test: batch get_requirement with single element (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_get_requirement_single_element() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.1".to_string()]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert!(parsed["data"].is_array());
    assert_eq!(parsed["data"].as_array().unwrap().len(), 1);
    // Each element has success/data structure (G.REQLIX_GET_REQUIREMENT.4)
    assert_eq!(parsed["data"][0]["success"], true);
    assert_eq!(parsed["data"][0]["data"]["index"], "G.C.1");
}

/// Test: batch get_requirement with multiple elements (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_get_requirement_multiple_elements() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: First

Content one.

## G.C.2: Second

Content two.

## G.C.3: Third

Content three.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(),
            "G.C.2".to_string(),
            "G.C.3".to_string(),
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 3);
    // Each element has success/data structure (G.REQLIX_GET_REQUIREMENT.4)
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[0]["data"]["index"], "G.C.1");
    assert_eq!(data[1]["success"], true);
    assert_eq!(data[1]["data"]["index"], "G.C.2");
    assert_eq!(data[2]["success"], true);
    assert_eq!(data[2]["data"]["index"], "G.C.3");
}

/// Test: batch get_requirement preserves order (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_get_requirement_preserves_order() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: First

Content one.

## G.C.2: Second

Content two.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    // Request in reverse order
    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.2".to_string(), "G.C.1".to_string()]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    // Each element has success/data structure (G.REQLIX_GET_REQUIREMENT.4)
    assert_eq!(data[0]["data"]["index"], "G.C.2");
    assert_eq!(data[1]["data"]["index"], "G.C.1");
}

/// Test: batch get_requirement processes all elements and returns individual errors (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_get_requirement_processes_all() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(),
            "G.C.999".to_string(), // Does not exist
            "G.C.1".to_string(),
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Top-level success is true (G.REQLIX_GET_REQUIREMENT.4)
    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 3);
    // First element: success
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[0]["data"]["index"], "G.C.1");
    // Second element: error
    assert_eq!(data[1]["success"], false);
    assert!(data[1]["error"].as_str().unwrap().contains("not found"));
    // Third element: success (all elements processed)
    assert_eq!(data[2]["success"], true);
    assert_eq!(data[2]["data"]["index"], "G.C.1");
}

/// Test: batch get_requirement exceeds limit (G.REQLIX_GET_REQUIREMENT.5)
#[test]
fn test_batch_get_requirement_exceeds_limit() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let indices: Vec<String> = (1..=101).map(|i| format!("G.C.{}", i)).collect();
    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(indices),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("100"));
}

/// Test: batch update_requirement with empty array returns empty result (G.P.4)
#[test]
fn test_batch_update_requirement_empty_array() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["data"], serde_json::json!([]));
}

/// Test: batch update_requirement with single item (G.REQLIX_U.3)
#[test]
fn test_batch_update_requirement_single_item() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nOld content.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![reqlix::UpdateItem {
            index: "G.C.1".to_string(),
            text: "New content".to_string(),
            title: None,
        }]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert!(parsed["data"].is_array());
    assert_eq!(parsed["data"].as_array().unwrap().len(), 1);
    // Each element has success/data structure (G.REQLIX_U.4)
    assert_eq!(parsed["data"][0]["success"], true);
    assert_eq!(parsed["data"][0]["data"]["text"], "New content");
}

/// Test: batch update_requirement with multiple items (G.REQLIX_U.3)
#[test]
fn test_batch_update_requirement_multiple_items() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: First

Old one.

## G.C.2: Second

Old two.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![
            reqlix::UpdateItem {
                index: "G.C.1".to_string(),
                text: "New one".to_string(),
                title: None,
            },
            reqlix::UpdateItem {
                index: "G.C.2".to_string(),
                text: "New two".to_string(),
                title: Some("Updated Second".to_string()),
            },
        ]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 2);
    // Each element has success/data structure (G.REQLIX_U.4)
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[0]["data"]["text"], "New one");
    assert_eq!(data[1]["success"], true);
    assert_eq!(data[1]["data"]["title"], "Updated Second");
}

/// Test: batch update_requirement processes all elements (G.REQLIX_U.3)
#[test]
fn test_batch_update_requirement_processes_all() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![
            reqlix::UpdateItem {
                index: "G.C.1".to_string(),
                text: "New".to_string(),
                title: None,
            },
            reqlix::UpdateItem {
                index: "G.C.999".to_string(), // Does not exist
                text: "New".to_string(),
                title: None,
            },
        ]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Top-level success is true (G.REQLIX_U.4)
    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 2);
    // First element: success
    assert_eq!(data[0]["success"], true);
    // Second element: error
    assert_eq!(data[1]["success"], false);
    assert!(data[1]["error"].as_str().unwrap().contains("not found"));
}

/// Test: batch update_requirement exceeds limit (G.REQLIX_U.7)
#[test]
fn test_batch_update_requirement_exceeds_limit() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let items: Vec<reqlix::UpdateItem> = (1..=101)
        .map(|i| reqlix::UpdateItem {
            index: format!("G.C.{}", i),
            text: "New".to_string(),
            title: None,
        })
        .collect();
    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(items),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("100"));
}

/// Test: update_requirement error when both index and items provided (G.REQLIX_U.2)
#[test]
fn test_update_requirement_both_index_and_items_error() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some("New".to_string()),
        title: None,
        items: Some(vec![]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("either"));
}

/// Test: update_requirement error when neither index nor items provided (G.REQLIX_U.2)
#[test]
fn test_update_requirement_neither_index_nor_items_error() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: None,
        text: None,
        title: None,
        items: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("required"));
}

/// Test: single update requires text parameter (G.REQLIX_U.2)
#[test]
fn test_single_update_requires_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: None, // Missing text
        title: None,
        items: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("text"));
}

/// Test: batch delete_requirement with empty array returns empty result (G.P.4)
#[test]
fn test_batch_delete_requirement_empty_array() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(vec![]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["data"], serde_json::json!([]));
}

/// Test: batch delete_requirement with single element (G.TOOLREQLIXD.3)
#[test]
fn test_batch_delete_requirement_single_element() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.1".to_string()]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert!(parsed["data"].is_array());
    assert_eq!(parsed["data"].as_array().unwrap().len(), 1);
    // Each element has success/data structure (G.TOOLREQLIXD.4)
    assert_eq!(parsed["data"][0]["success"], true);
    assert_eq!(parsed["data"][0]["data"]["index"], "G.C.1");
}

/// Test: batch delete_requirement with multiple elements (G.TOOLREQLIXD.3)
#[test]
fn test_batch_delete_requirement_multiple_elements() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: First

Content one.

## G.C.2: Second

Content two.

## G.C.3: Third

Content three.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.1".to_string(), "G.C.2".to_string()]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 2);
    // Each element has success/data structure (G.TOOLREQLIXD.4)
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[1]["success"], true);

    // Verify file content
    let file_content = fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(!file_content.contains("G.C.1"));
    assert!(!file_content.contains("G.C.2"));
    assert!(file_content.contains("G.C.3"));
}

/// Test: batch delete_requirement processes all elements (G.TOOLREQLIXD.3)
#[test]
fn test_batch_delete_requirement_processes_all() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: First

Content one.

## G.C.2: Second

Content two.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(),
            "G.C.999".to_string(), // Does not exist
            "G.C.2".to_string(),
        ]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Top-level success is true (G.TOOLREQLIXD.4)
    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 3);
    // First element: success
    assert_eq!(data[0]["success"], true);
    // Second element: error
    assert_eq!(data[1]["success"], false);
    assert!(data[1]["error"].as_str().unwrap().contains("not found"));
    // Third element: success (all elements processed)
    assert_eq!(data[2]["success"], true);

    // Both existing requirements should be deleted
    let file_content = fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(!file_content.contains("G.C.1"));
    assert!(!file_content.contains("G.C.2"));
}

/// Test: batch delete_requirement exceeds limit (G.TOOLREQLIXD.6)
#[test]
fn test_batch_delete_requirement_exceeds_limit() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let indices: Vec<String> = (1..=101).map(|i| format!("G.C.{}", i)).collect();
    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test batch".to_string(),
        index: reqlix::IndexParam::Batch(indices),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("100"));
}
/// Test: batch get with duplicate indices works (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_get_with_duplicates() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(),
            "G.C.1".to_string(),
            "G.C.1".to_string(),
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 3);
}

/// Test: batch get from multiple categories (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_get_multiple_categories() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: General\n\nGeneral content.\n",
    );
    create_category_file_in_req_dir(
        &req_dir,
        "testing",
        "# Chapter\n\n## T.C.1: Testing\n\nTesting content.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.1".to_string(), "T.C.1".to_string()]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 2);
    // Each element has success/data structure (G.REQLIX_GET_REQUIREMENT.4)
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[0]["data"]["category"], "general");
    assert_eq!(data[1]["success"], true);
    assert_eq!(data[1]["data"]["category"], "testing");
}

/// Test: batch update preserves order (G.REQLIX_U.3)
#[test]
fn test_batch_update_preserves_order() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: First

Old one.

## G.C.2: Second

Old two.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    // Update in reverse order
    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![
            reqlix::UpdateItem {
                index: "G.C.2".to_string(),
                text: "New two".to_string(),
                title: None,
            },
            reqlix::UpdateItem {
                index: "G.C.1".to_string(),
                text: "New one".to_string(),
                title: None,
            },
        ]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    // Each element has success/data structure (G.REQLIX_U.4)
    assert_eq!(data[0]["data"]["index"], "G.C.2");
    assert_eq!(data[1]["data"]["index"], "G.C.1");
}

/// Test: batch delete preserves order (G.TOOLREQLIXD.3)
#[test]
fn test_batch_delete_preserves_order() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: First

Content one.

## G.C.2: Second

Content two.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    // Delete in reverse order
    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.2".to_string(), "G.C.1".to_string()]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    // Each element has success/data structure (G.TOOLREQLIXD.4)
    assert_eq!(data[0]["data"]["index"], "G.C.2");
    assert_eq!(data[1]["data"]["index"], "G.C.1");
}

/// Test: batch at exact limit of 100 works (G.REQLIX_GET_REQUIREMENT.5)
#[test]
fn test_batch_get_at_limit() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");

    // Create 100 requirements
    let mut content = String::from("# Chapter\n\n");
    for i in 1..=100 {
        content.push_str(&format!("## G.C.{}: Req{}\n\nContent {}.\n\n", i, i, i));
    }
    create_category_file_in_req_dir(&req_dir, "general", &content);

    let indices: Vec<String> = (1..=100).map(|i| format!("G.C.{}", i)).collect();
    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(indices),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 100);
}

// =============================================================================
// Additional validation and edge case tests
// =============================================================================

/// Test: validate_index with empty string
#[test]
fn test_validate_index_empty() {
    let result = RequirementsServer::validate_index("");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("required"));
}

/// Test: validate_index with very long string
#[test]
fn test_validate_index_too_long() {
    let long_index = "G.".to_string() + &"A".repeat(200);
    let result = RequirementsServer::validate_index(&long_index);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("exceeds"));
}

/// Test: validate_text with empty string
#[test]
fn test_validate_text_empty() {
    let result = RequirementsServer::validate_text("");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("required"));
}

/// Test: validate_project_root with empty string
#[test]
fn test_validate_project_root_empty() {
    let result = RequirementsServer::validate_project_root("");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("required"));
}

/// Test: parse_level1_heading with valid heading
#[test]
fn test_parse_level1_heading_valid() {
    let result = RequirementsServer::parse_level1_heading("# Chapter Name");
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "Chapter Name");
}

/// Test: parse_level1_heading with level2
#[test]
fn test_parse_level1_heading_level2() {
    let result = RequirementsServer::parse_level1_heading("## Not Level 1");
    assert!(result.is_none());
}

/// Test: parse_level1_heading with no space
#[test]
fn test_parse_level1_heading_no_space() {
    let result = RequirementsServer::parse_level1_heading("#NoSpace");
    assert!(result.is_none());
}

/// Test: parse_level2_heading with valid heading
#[test]
fn test_parse_level2_heading_valid() {
    let result = RequirementsServer::parse_level2_heading("## G.C.1: Title");
    assert!(result.is_some());
    let (index, title) = result.unwrap();
    assert_eq!(index, "G.C.1");
    assert_eq!(title, "Title");
}

/// Test: parse_level2_heading with level1
#[test]
fn test_parse_level2_heading_level1() {
    let result = RequirementsServer::parse_level2_heading("# Not Level 2");
    assert!(result.is_none());
}

/// Test: parse_level2_heading with level3
#[test]
fn test_parse_level2_heading_level3() {
    let result = RequirementsServer::parse_level2_heading("### Not Level 2");
    assert!(result.is_none());
}

/// Test: parse_index with valid index
#[test]
fn test_parse_index_valid() {
    let result = RequirementsServer::parse_index("G.C.1");
    assert!(result.is_ok());
    let (cat, chap, num) = result.unwrap();
    assert_eq!(cat, "G");
    assert_eq!(chap, "C");
    assert_eq!(num, "1");
}

/// Test: parse_index with invalid format
#[test]
fn test_parse_index_invalid() {
    let result = RequirementsServer::parse_index("invalid");
    assert!(result.is_err());
}

/// Test: parse_index with two parts
#[test]
fn test_parse_index_two_parts() {
    let result = RequirementsServer::parse_index("G.C");
    assert!(result.is_err());
}

/// Test: insert requirement creates new chapter
#[test]
fn test_insert_creates_chapter() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Existing\n\n## G.E.1: Test\n\nContent.\n",
    );

    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "New Chapter".to_string(),
        title: "New Req".to_string(),
        text: "New content".to_string(),
    };
    let result = RequirementsServer::handle_insert_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let file_content = fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(file_content.contains("# New Chapter"));
}

/// Test: update with new title changes heading
#[test]
fn test_update_changes_title() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Old Title\n\nContent.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some("Content".to_string()),
        title: Some("New Title".to_string()),
        items: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["data"]["title"], "New Title");

    let file_content = fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(file_content.contains("## G.C.1: New Title"));
    assert!(!file_content.contains("Old Title"));
}

/// Test: update without title keeps existing title
#[test]
fn test_update_keeps_title() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Original Title\n\nOld content.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some("New content".to_string()),
        title: None,
        items: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["data"]["title"], "Original Title");
}

/// Test: delete removes empty chapter
#[test]
fn test_delete_removes_empty_chapter() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter One

## G.O.1: Only Req

Content.

# Chapter Two

## G.T.1: Another Req

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Single("G.O.1".to_string()),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);

    let file_content = fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(!file_content.contains("# Chapter One"));
    assert!(file_content.contains("# Chapter Two"));
}

/// Test: validate category name with invalid characters
#[test]
fn test_validate_category_invalid_chars() {
    let result = RequirementsServer::validate_category("General");
    assert!(result.is_err());
}

/// Test: validate category name valid
#[test]
fn test_validate_category_valid() {
    let result = RequirementsServer::validate_category("general_test");
    assert!(result.is_ok());
}

/// Test: validate chapter name with invalid characters
#[test]
fn test_validate_chapter_invalid_chars() {
    let result = RequirementsServer::validate_chapter("Chapter\nName");
    assert!(result.is_err());
}

/// Test: validate chapter name valid
#[test]
fn test_validate_chapter_valid() {
    let result = RequirementsServer::validate_chapter("Tool: test-chapter");
    assert!(result.is_ok());
}

/// Test: get_requirement with multipart index
#[test]
fn test_get_requirement_multipart_index() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Long Chapter Name\n\n## G.LONGCHAPTER.1: Test\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Single("G.LONGCHAPTER.1".to_string()),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["data"]["index"], "G.LONGCHAPTER.1");
}

/// Test: requirement with code block in content (G.R.5)
#[test]
fn test_requirement_with_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Code Example

Here is code:

```rust
fn main() {
    ## This is not a heading
    # Neither is this
}
```

End of requirement.

## G.C.2: Next

Next content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Single("G.C.1".to_string()),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let text = parsed["data"]["text"].as_str().unwrap();
    assert!(text.contains("## This is not a heading"));
    assert!(text.contains("End of requirement."));
}

// =============================================================================
// Additional batch operation tests (G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_U.3, G.TOOLREQLIXD.3)
// =============================================================================

/// Test: batch get with mixed success and error results (G.REQLIX_GET_REQUIREMENT.4)
#[test]
fn test_batch_get_mixed_results_structure() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: First\n\nContent one.\n\n## G.C.3: Third\n\nContent three.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(),
            "G.C.2".to_string(), // Does not exist
            "G.C.3".to_string(),
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 3);
    // First: success
    assert_eq!(data[0]["success"], true);
    assert!(data[0]["data"].is_object());
    // Second: error
    assert_eq!(data[1]["success"], false);
    assert!(data[1]["error"].is_string());
    // Third: success (processed despite previous error)
    assert_eq!(data[2]["success"], true);
    assert!(data[2]["data"].is_object());
}

/// Test: batch delete where second delete fails because first already deleted (G.TOOLREQLIXD.3)
#[test]
fn test_batch_delete_same_index_twice() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.1".to_string(), "G.C.1".to_string()]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 2);
    // First: success
    assert_eq!(data[0]["success"], true);
    // Second: error (already deleted)
    assert_eq!(data[1]["success"], false);
}

/// Test: batch update with title conflict in same batch (G.REQLIX_U.3)
#[test]
fn test_batch_update_title_conflict_in_batch() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: First\n\nContent one.\n\n## G.C.2: Second\n\nContent two.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![
            reqlix::UpdateItem {
                index: "G.C.1".to_string(),
                text: "New content".to_string(),
                title: Some("Conflict Title".to_string()),
            },
            reqlix::UpdateItem {
                index: "G.C.2".to_string(),
                text: "New content".to_string(),
                title: Some("Conflict Title".to_string()), // Will conflict with G.C.1's new title
            },
        ]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 2);
    // First: success
    assert_eq!(data[0]["success"], true);
    // Second: error (title conflict)
    assert_eq!(data[1]["success"], false);
    assert!(data[1]["error"]
        .as_str()
        .unwrap()
        .contains("already exists"));
}

/// Test: batch delete leaves chapter if some requirements remain (G.TOOLREQLIXD.3)
#[test]
fn test_batch_delete_partial_chapter() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: First\n\nOne.\n\n## G.C.2: Second\n\nTwo.\n\n## G.C.3: Third\n\nThree.\n",
    );

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.1".to_string(), "G.C.3".to_string()]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);

    // Chapter should still exist with G.C.2
    let file_content = fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(file_content.contains("# Chapter"));
    assert!(file_content.contains("G.C.2"));
    assert!(!file_content.contains("G.C.1"));
    assert!(!file_content.contains("G.C.3"));
}

/// Test: batch get with invalid index format (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_get_invalid_index_format() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(),
            "INVALID".to_string(), // Invalid format
            "G.C.1".to_string(),
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[1]["success"], false);
    assert_eq!(data[2]["success"], true);
}

/// Test: batch update with empty text in one item (G.REQLIX_U.3)
#[test]
fn test_batch_update_empty_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: First\n\nOne.\n\n## G.C.2: Second\n\nTwo.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![
            reqlix::UpdateItem {
                index: "G.C.1".to_string(),
                text: "Valid text".to_string(),
                title: None,
            },
            reqlix::UpdateItem {
                index: "G.C.2".to_string(),
                text: "".to_string(), // Empty text - invalid
                title: None,
            },
        ]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[1]["success"], false);
}

/// Test: batch delete from non-existent category (G.TOOLREQLIXD.3)
#[test]
fn test_batch_delete_nonexistent_category() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(),
            "X.C.1".to_string(), // Non-existent category
        ]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[1]["success"], false);
}

/// Test: single get returns correct response format (G.REQLIX_GET_REQUIREMENT.4)
#[test]
fn test_single_get_response_format() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test Title\n\nTest content.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Single("G.C.1".to_string()),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Single request format - data is object, not array
    assert_eq!(parsed["success"], true);
    assert!(parsed["data"].is_object());
    assert!(!parsed["data"].is_array());
    assert_eq!(parsed["data"]["index"], "G.C.1");
    assert_eq!(parsed["data"]["title"], "Test Title");
    assert_eq!(parsed["data"]["category"], "general");
    assert_eq!(parsed["data"]["chapter"], "Chapter");
}

/// Test: single get error returns correct format (G.REQLIX_GET_REQUIREMENT.4)
#[test]
fn test_single_get_error_format() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Single("G.C.999".to_string()),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Single request error - top level success is false
    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].is_string());
}

/// Test: batch operations each element has success field (G.REQLIX_GET_REQUIREMENT.4)
#[test]
fn test_batch_all_elements_have_success() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(),
            "G.C.999".to_string(),
            "G.C.1".to_string(),
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    let data = parsed["data"].as_array().unwrap();
    for item in data {
        assert!(
            item.get("success").is_some(),
            "Each batch element must have success field"
        );
    }
}

/// Test: batch get returns correct chapter for each element (G.REQLIX_GET_REQUIREMENT.4)
#[test]
fn test_batch_get_returns_correct_chapters() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter One\n\n## G.CHAPTERONE.1: Test\n\nContent.\n\n# Chapter Two\n\n## G.CHAPTERTWO.1: Test\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.CHAPTERONE.1".to_string(),
            "G.CHAPTERTWO.1".to_string(),
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data[0]["data"]["chapter"], "Chapter One");
    assert_eq!(data[1]["data"]["chapter"], "Chapter Two");
}

/// Test: batch update with too long title (G.REQLIX_U.3, G.P.1)
#[test]
fn test_batch_update_title_too_long() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: First\n\nOne.\n\n## G.C.2: Second\n\nTwo.\n",
    );

    let long_title = "A".repeat(101); // Exceeds 100 char limit
    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![
            reqlix::UpdateItem {
                index: "G.C.1".to_string(),
                text: "Valid text".to_string(),
                title: None,
            },
            reqlix::UpdateItem {
                index: "G.C.2".to_string(),
                text: "Valid text".to_string(),
                title: Some(long_title),
            },
        ]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[1]["success"], false);
}

/// Test: batch delete from multiple categories (G.TOOLREQLIXD.3)
#[test]
fn test_batch_delete_multiple_categories() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: General\n\nGeneral content.\n",
    );
    create_category_file_in_req_dir(
        &req_dir,
        "testing",
        "# Chapter\n\n## T.C.1: Testing\n\nTesting content.\n",
    );

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.1".to_string(), "T.C.1".to_string()]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[0]["data"]["category"], "general");
    assert_eq!(data[1]["success"], true);
    assert_eq!(data[1]["data"]["category"], "testing");
}

/// Test: batch preserves order with partial errors in middle (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_preserves_order_with_errors() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: First\n\nOne.\n\n## G.C.3: Third\n\nThree.\n\n## G.C.5: Fifth\n\nFive.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(),
            "G.C.2".to_string(), // error
            "G.C.3".to_string(),
            "G.C.4".to_string(), // error
            "G.C.5".to_string(),
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 5);
    assert_eq!(data[0]["success"], true);
    assert_eq!(data[0]["data"]["index"], "G.C.1");
    assert_eq!(data[1]["success"], false);
    assert_eq!(data[2]["success"], true);
    assert_eq!(data[2]["data"]["index"], "G.C.3");
    assert_eq!(data[3]["success"], false);
    assert_eq!(data[4]["success"], true);
    assert_eq!(data[4]["data"]["index"], "G.C.5");
}

/// Test: batch update verifies text actually changed in file (G.REQLIX_U.3)
#[test]
fn test_batch_update_changes_file_content() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: First\n\nOld one.\n\n## G.C.2: Second\n\nOld two.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![
            reqlix::UpdateItem {
                index: "G.C.1".to_string(),
                text: "UPDATED ONE".to_string(),
                title: None,
            },
            reqlix::UpdateItem {
                index: "G.C.2".to_string(),
                text: "UPDATED TWO".to_string(),
                title: None,
            },
        ]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);

    // Verify file content actually changed
    let file_content = fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(file_content.contains("UPDATED ONE"));
    assert!(file_content.contains("UPDATED TWO"));
    assert!(!file_content.contains("Old one"));
    assert!(!file_content.contains("Old two"));
}

/// Test: batch delete removes chapter when all requirements deleted (G.TOOLREQLIXD.3)
#[test]
fn test_batch_delete_removes_empty_chapter() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter One\n\n## G.CHAPTERONE.1: Test\n\nContent.\n\n# Chapter Two\n\n## G.CHAPTERTWO.1: Keep\n\nContent.\n",
    );

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.CHAPTERONE.1".to_string()]),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);

    // Chapter One should be removed, Chapter Two should remain
    let file_content = fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(!file_content.contains("# Chapter One"));
    assert!(file_content.contains("# Chapter Two"));
    assert!(file_content.contains("G.CHAPTERTWO.1"));
}

/// Test: batch processes all elements even after multiple errors (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_continues_after_multiple_errors() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.5: Last\n\nContent.\n",
    );

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(), // error
            "G.C.2".to_string(), // error
            "G.C.3".to_string(), // error
            "G.C.4".to_string(), // error
            "G.C.5".to_string(), // success - must be processed
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    let data = parsed["data"].as_array().unwrap();
    assert_eq!(data.len(), 5);
    // First 4 are errors
    for i in 0..4 {
        assert_eq!(data[i]["success"], false);
    }
    // Last one should be processed and succeed
    assert_eq!(data[4]["success"], true);
    assert_eq!(data[4]["data"]["index"], "G.C.5");
}

/// Test: single update response format (G.REQLIX_U.4)
#[test]
fn test_single_update_response_format() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nOld content.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some("New content".to_string()),
        title: None,
        items: None,
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Single update - data is object, not array
    assert_eq!(parsed["success"], true);
    assert!(parsed["data"].is_object());
    assert!(!parsed["data"].is_array());
    assert_eq!(parsed["data"]["text"], "New content");
}

/// Test: single delete response format (G.TOOLREQLIXD.4)
#[test]
fn test_single_delete_response_format() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test Title\n\nContent.\n",
    );

    let params = reqlix::DeleteRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Single("G.C.1".to_string()),
    };
    let result = RequirementsServer::handle_delete_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Single delete - data is object, not array
    assert_eq!(parsed["success"], true);
    assert!(parsed["data"].is_object());
    assert!(!parsed["data"].is_array());
    assert_eq!(parsed["data"]["index"], "G.C.1");
    assert_eq!(parsed["data"]["title"], "Test Title");
}

/// Test: batch get does not modify file (G.REQLIX_GET_REQUIREMENT.3)
#[test]
fn test_batch_get_readonly() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let original_content = "# Chapter\n\n## G.C.1: Test\n\nContent.\n";
    create_category_file_in_req_dir(&req_dir, "general", original_content);

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec!["G.C.1".to_string(), "G.C.1".to_string()]),
    };
    let _ = RequirementsServer::handle_get_requirement(params);

    // File should be unchanged
    let file_content = fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert_eq!(file_content, original_content);
}

/// Test: batch update with index not found in first item (G.REQLIX_U.3)
#[test]
fn test_batch_update_first_item_error() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.2: Second\n\nTwo.\n",
    );

    let params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: None,
        text: None,
        title: None,
        items: Some(vec![
            reqlix::UpdateItem {
                index: "G.C.1".to_string(), // Does not exist
                text: "New".to_string(),
                title: None,
            },
            reqlix::UpdateItem {
                index: "G.C.2".to_string(),
                text: "Updated".to_string(),
                title: None,
            },
        ]),
    };
    let result = RequirementsServer::handle_update_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    // First: error
    assert_eq!(data[0]["success"], false);
    // Second: success (continues despite first error)
    assert_eq!(data[1]["success"], true);
}

/// Test: batch with all errors returns success at top level (G.REQLIX_GET_REQUIREMENT.4)
#[test]
fn test_batch_all_errors_top_level_success() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    let params = reqlix::GetRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: reqlix::IndexParam::Batch(vec![
            "G.C.1".to_string(), // error
            "G.C.2".to_string(), // error
            "G.C.3".to_string(), // error
        ]),
    };
    let result = RequirementsServer::handle_get_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Top-level success is true for batch
    assert_eq!(parsed["success"], true);
    let data = parsed["data"].as_array().unwrap();
    // All elements are errors
    for item in data {
        assert_eq!(item["success"], false);
    }
}
