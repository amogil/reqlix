// Tests for Tool: reqlix_insert_requirement (T.REQLIXI.*)
// Covers Requirements: T.REQLIXI.1, T.REQLIXI.2, T.REQLIXI.3, T.REQLIXI.4, T.REQLIXI.5, T.REQLIXI.6

use reqlix::RequirementsServer;
use tempfile::TempDir;

use super::common::{
    create_agents_file_in_req_dir, create_category_file_in_req_dir, create_requirements_dir,
};

// =============================================================================
// Tests for reqlix_insert_requirement (G.REQLIX_I.*)
// =============================================================================

/// Test: reqlix_insert_requirement creates new requirement (T.REQLIXI.1, T.REQLIXI.4)
/// Precondition: System has category file with chapter
/// Action: Call reqlix_insert_requirement with valid parameters
/// Result: Function creates requirement and returns full data in correct format
/// Covers Requirement: T.REQLIXI.1, T.REQLIXI.3, T.REQLIXI.4, T.REQLIXI.5
#[test]
fn test_insert_requirement_new() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Test Chapter

## G.T.1: Existing Requirement

Existing content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test insert".to_string(),
        category: "general".to_string(),
        chapter: "Test Chapter".to_string(),
        title: "New Requirement".to_string(),
        text: "New content".to_string(),
    };

    let result = RequirementsServer::handle_insert_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true, "Insert should succeed: {}", result);
    
    // Verify response format (T.REQLIXI.4)
    assert!(parsed["data"].is_object());
    assert!(parsed["data"]["index"].is_string());
    assert_eq!(parsed["data"]["title"], "New Requirement");
    assert_eq!(parsed["data"]["text"], "New content");
    assert_eq!(parsed["data"]["category"], "general");
    assert_eq!(parsed["data"]["chapter"], "Test Chapter");

    // Verify requirement was added to file
    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(file_content.contains("## G.T.2: New Requirement"));
    assert!(file_content.contains("New content"));
}

/// Test: reqlix_insert_requirement validates title uniqueness (T.REQLIXI.3)
/// Precondition: System has category file with requirement having same title
/// Action: Call reqlix_insert_requirement with duplicate title
/// Result: Function returns error "Title already exists in chapter"
/// Covers Requirement: T.REQLIXI.3 step 3
#[test]
fn test_insert_requirement_duplicate_title() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Test Chapter

## G.T.1: Duplicate Title

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test insert".to_string(),
        category: "general".to_string(),
        chapter: "Test Chapter".to_string(),
        title: "Duplicate Title".to_string(), // Same as existing
        text: "New content".to_string(),
    };

    let result = RequirementsServer::handle_insert_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false, "Insert should fail with duplicate title");
    assert!(parsed["error"]
        .as_str()
        .unwrap()
        .contains("already exists"));
}

/// Test: reqlix_insert_requirement validates parameters through handler (T.REQLIXI.2, T.REQLIXI.5)
/// Precondition: System has invalid parameters
/// Action: Call handle_insert_requirement with invalid parameters
/// Result: Function returns validation error before processing
/// Covers Requirement: T.REQLIXI.2, T.REQLIXI.5, G.P.1, G.P.2
#[test]
fn test_insert_requirement_validation() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    // Test empty project_root
    let params = reqlix::InsertRequirementParams {
        project_root: "".to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Test".to_string(),
        text: "Content".to_string(),
    };
    let result = RequirementsServer::handle_insert_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("project_root"));

    // Test empty category
    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "".to_string(),
        chapter: "Chapter".to_string(),
        title: "Test".to_string(),
        text: "Content".to_string(),
    };
    let result = RequirementsServer::handle_insert_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("category"));

    // Test empty text
    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Test".to_string(),
        text: "".to_string(),
    };
    let result = RequirementsServer::handle_insert_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("text"));
}

// =============================================================================
// Tests for T.REQLIXI.3: Algorithm (category and chapter creation)
// =============================================================================

/// Test: reqlix_insert_requirement creates category file if not found (T.REQLIXI.3 step 1)
/// Precondition: System has no category file
/// Action: Call handle_insert_requirement with new category
/// Result: Function creates category file and inserts requirement
/// Covers Requirement: T.REQLIXI.3 step 1
#[test]
fn test_insert_requirement_creates_category() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");

    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test insert".to_string(),
        category: "newcategory".to_string(),
        chapter: "New Chapter".to_string(),
        title: "New Requirement".to_string(),
        text: "New content".to_string(),
    };

    let result = RequirementsServer::handle_insert_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true, "Insert should succeed: {}", result);
    
    // Verify category file was created
    assert!(req_dir.join("newcategory.md").exists(), "Category file should be created");
    
    // Verify requirement was inserted
    let file_content = std::fs::read_to_string(req_dir.join("newcategory.md")).unwrap();
    assert!(file_content.contains("# New Chapter"));
    assert!(file_content.contains("## N.N.1: New Requirement"));
    assert!(file_content.contains("New content"));
}

/// Test: reqlix_insert_requirement creates chapter if not found (T.REQLIXI.3 step 2)
/// Precondition: System has category file without the specified chapter
/// Action: Call handle_insert_requirement with new chapter name
/// Result: Function creates chapter heading and inserts requirement
/// Covers Requirement: T.REQLIXI.3 step 2
#[test]
fn test_insert_requirement_creates_chapter() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Existing Chapter\n\n## G.E.1: Existing\n\nContent.\n");

    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test insert".to_string(),
        category: "general".to_string(),
        chapter: "New Chapter".to_string(),
        title: "New Requirement".to_string(),
        text: "New content".to_string(),
    };

    let result = RequirementsServer::handle_insert_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true, "Insert should succeed: {}", result);
    
    // Verify chapter was created
    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(file_content.contains("# New Chapter"), "New chapter should be created");
    // Verify requirement was inserted in new chapter (index format depends on prefix calculation)
    assert!(file_content.contains("New Requirement"), "Requirement should be in new chapter");
    assert!(file_content.contains("New content"));
    
    // Verify the new chapter appears after existing chapter
    let existing_pos = file_content.find("# Existing Chapter").unwrap();
    let new_chapter_pos = file_content.find("# New Chapter").unwrap();
    assert!(new_chapter_pos > existing_pos, "New chapter should be after existing chapter");
}

// =============================================================================
// Tests for chapter name substring bug fix
// =============================================================================

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
    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();

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
    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(
        file_content.contains("New content\n\n# Second Chapter"),
        "Should have blank line before next chapter heading. Content:\n{}",
        file_content
    );
}

// Note: Embedding creation tests are covered in embedding_integration_tests.rs
// (test_insert_creates_embedding_after_heading, test_insert_embedding_before_text, etc.)
