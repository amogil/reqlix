// Tests for Requirements Storage Format (G.R.*)
// Covers Requirements: G.R.1, G.R.2, G.R.3, G.R.4, G.R.5, G.R.8, G.R.9, G.R.10, G.R.11, G.R.12

use super::common::{
    create_agents_file_in_req_dir, create_category_file_in_req_dir, create_requirements_dir,
};
use reqlix::RequirementsServer;
use tempfile::TempDir;

// =============================================================================
// Tests for G.R.2: Chapter definition (level-1 headings)
// =============================================================================

// Tests for parse_level1_heading (G.R.2)

/// Test: parse_level1_heading with valid heading
/// Precondition: System has valid level-1 heading string
/// Action: Call parse_level1_heading with "# Chapter Name"
/// Result: Function returns Some("Chapter Name")
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_valid() {
    let result = RequirementsServer::parse_level1_heading("# Chapter Name");
    assert_eq!(result, Some("Chapter Name".to_string()));
}

/// Test: parse_level1_heading with indented heading (1 space)
/// Precondition: System has level-1 heading with 1 space indent
/// Action: Call parse_level1_heading with " # Chapter Name"
/// Result: Function returns Some("Chapter Name")
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_one_space_indent() {
    let result = RequirementsServer::parse_level1_heading(" # Chapter Name");
    assert_eq!(result, Some("Chapter Name".to_string()));
}

/// Test: parse_level1_heading with indented heading (3 spaces)
/// Precondition: System has level-1 heading with 3 space indent
/// Action: Call parse_level1_heading with "   # Chapter Name"
/// Result: Function returns Some("Chapter Name")
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_three_space_indent() {
    let result = RequirementsServer::parse_level1_heading("   # Chapter Name");
    assert_eq!(result, Some("Chapter Name".to_string()));
}

/// Test: parse_level1_heading with level-2 heading
/// Precondition: System has level-2 heading string
/// Action: Call parse_level1_heading with "## Requirement"
/// Result: Function returns None (not a level-1 heading)
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_level2() {
    let result = RequirementsServer::parse_level1_heading("## Requirement");
    assert_eq!(result, None);
}

/// Test: parse_level1_heading with invalid format (no space after #)
/// Precondition: System has invalid heading format without space after #
/// Action: Call parse_level1_heading with "#Chapter Name"
/// Result: Function returns None (invalid format)
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_no_space() {
    let result = RequirementsServer::parse_level1_heading("#Chapter Name");
    assert_eq!(result, None);
}

// Tests for read_chapters_streaming (G.R.2, G.R.5)

/// Test: read_chapters_streaming with single chapter
/// Precondition: System has markdown file with single level-1 heading
/// Action: Call read_chapters_streaming with file path
/// Result: Function returns Ok with vector containing one chapter name
/// Covers Requirement: G.R.2
#[test]
fn test_read_chapters_streaming_single() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "# Chapter One\n").unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec!["Chapter One"]);
}

/// Test: read_chapters_streaming with multiple chapters
/// Precondition: System has markdown file with multiple level-1 headings
/// Action: Call read_chapters_streaming with file path
/// Result: Function returns Ok with vector containing all chapter names in order
/// Covers Requirement: G.R.2
#[test]
fn test_read_chapters_streaming_multiple() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "# Chapter One\n\nSome text\n\n# Chapter Two\n").unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 2);
    assert_eq!(chapters[0], "Chapter One");
    assert_eq!(chapters[1], "Chapter Two");
}

/// Test: read_chapters_streaming ignoring headings in code blocks
/// Precondition: System has markdown file with headings inside code blocks
/// Action: Call read_chapters_streaming with file path
/// Result: Function returns Ok with vector containing only real chapters (code block headings ignored)
/// Covers Requirement: G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Real Chapter\n\n```\n# Fake Chapter\n```\n\n# Another Chapter\n",
    )
    .unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 2);
    assert_eq!(chapters[0], "Real Chapter");
    assert_eq!(chapters[1], "Another Chapter");
}

// =============================================================================
// Tests for G.R.3: Requirement definition (level-2 headings)
// =============================================================================

// Tests for parse_level2_heading (G.R.3)

/// Test: parse_level2_heading with valid heading
/// Precondition: System has valid level-2 heading string with index and title
/// Action: Call parse_level2_heading with "## G.G.1: Requirement Title"
/// Result: Function returns Some((index, title))
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_valid() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1: Requirement Title");
    assert_eq!(
        result,
        Some(("G.G.1".to_string(), "Requirement Title".to_string()))
    );
}

/// Test: parse_level2_heading with indented heading
/// Precondition: System has level-2 heading with indentation
/// Action: Call parse_level2_heading with indented heading
/// Result: Function returns Some((index, title)) (indentation is allowed)
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_indented() {
    let result = RequirementsServer::parse_level2_heading("  ## G.G.1: Requirement Title");
    assert_eq!(
        result,
        Some(("G.G.1".to_string(), "Requirement Title".to_string()))
    );
}

/// Test: parse_level2_heading with level-3 heading
/// Precondition: System has level-3 heading string
/// Action: Call parse_level2_heading with "### Subsection"
/// Result: Function returns None (not a level-2 heading)
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_level3() {
    let result = RequirementsServer::parse_level2_heading("### Subsection");
    assert_eq!(result, None);
}

/// Test: parse_level2_heading with missing colon
/// Precondition: System has level-2 heading without colon separator
/// Action: Call parse_level2_heading with "## G.G.1 Requirement Title"
/// Result: Function returns None (colon required)
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_missing_colon() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1 Requirement Title");
    assert_eq!(result, None);
}

/// Test: parse_level2_heading with empty index
/// Precondition: System has level-2 heading with empty index
/// Action: Call parse_level2_heading with "## : Requirement Title"
/// Result: Function returns None (index cannot be empty)
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_empty_index() {
    let result = RequirementsServer::parse_level2_heading("## : Requirement Title");
    assert_eq!(result, None);
}

// Tests for read_requirements_streaming (G.R.3, G.R.5)

/// Test: read_requirements_streaming with single requirement
/// Precondition: System has markdown file with one chapter and one requirement
/// Action: Call read_requirements_streaming with file path and chapter name
/// Result: Function returns Ok with vector containing one requirement with correct index and title
/// Covers Requirement: G.R.3, G.R.5
#[test]
fn test_read_requirements_streaming_single() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "# Chapter\n## G.G.1: Title\n").unwrap();

    let result = RequirementsServer::read_requirements_streaming(&file_path, "Chapter");
    assert!(result.is_ok());
    let requirements = result.unwrap();
    assert_eq!(requirements.len(), 1);
    assert_eq!(requirements[0].index, "G.G.1");
    assert_eq!(requirements[0].title, "Title");
}

/// Test: read_requirements_streaming with multiple requirements
/// Precondition: System has markdown file with one chapter and multiple requirements
/// Action: Call read_requirements_streaming with file path and chapter name
/// Result: Function returns Ok with vector containing all requirements with correct indices and titles
/// Covers Requirement: G.R.3, G.R.5
#[test]
fn test_read_requirements_streaming_multiple() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Chapter\n## G.G.1: Title One\n## G.G.2: Title Two\n",
    )
    .unwrap();

    let result = RequirementsServer::read_requirements_streaming(&file_path, "Chapter");
    assert!(result.is_ok());
    let requirements = result.unwrap();
    assert_eq!(requirements.len(), 2);
    assert_eq!(requirements[0].index, "G.G.1");
    assert_eq!(requirements[0].title, "Title One");
    assert_eq!(requirements[1].index, "G.G.2");
    assert_eq!(requirements[1].title, "Title Two");
}

// =============================================================================
// Tests for G.R.4: Index format
// =============================================================================

// Tests for parse_index (G.R.4)

/// Test: parse_index with valid index
/// Precondition: System has valid index string in format CATEGORY.CHAPTER.NUMBER
/// Action: Call parse_index with "G.G.1"
/// Result: Function returns Ok((category, chapter, number))
/// Covers Requirement: G.R.4
#[test]
fn test_parse_index_valid() {
    let result = RequirementsServer::parse_index("G.G.1");
    assert_eq!(
        result,
        Ok(("G".to_string(), "G".to_string(), "1".to_string()))
    );
}

/// Test: parse_index with multi-character parts
/// Precondition: System has index string with multi-character parts
/// Action: Call parse_index with "GET.GET_C.123"
/// Result: Function returns Ok((category, chapter, number)) with multi-character parts
/// Covers Requirement: G.R.4
#[test]
fn test_parse_index_multi_char() {
    let result = RequirementsServer::parse_index("GET.GET_C.123");
    assert_eq!(
        result,
        Ok(("GET".to_string(), "GET_C".to_string(), "123".to_string()))
    );
}

/// Test: parse_index with invalid format (too few parts)
/// Precondition: System has index string with too few parts
/// Action: Call parse_index with "G.G"
/// Result: Function returns error about invalid index format
/// Covers Requirement: G.R.4
#[test]
fn test_parse_index_too_few_parts() {
    let result = RequirementsServer::parse_index("G.G");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid index format"));
}

/// Test: parse_index with invalid format (too many parts)
/// Precondition: System has index string with too many parts
/// Action: Call parse_index with "G.G.1.2"
/// Result: Function returns error about invalid index format
/// Covers Requirement: G.R.4
#[test]
fn test_parse_index_too_many_parts() {
    let result = RequirementsServer::parse_index("G.G.1.2");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid index format"));
}

// Tests for calculate_unique_prefix (G.R.4)

/// Test: calculate_unique_prefix with single name
/// Precondition: System has single name in list
/// Action: Call calculate_unique_prefix with single name
/// Result: Function returns single character prefix
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_single_name() {
    let names = vec!["general".to_string()];
    let result = RequirementsServer::calculate_unique_prefix("general", &names);
    assert_eq!(result, "G");
}

/// Test: calculate_unique_prefix with unique first letter
/// Precondition: System has multiple names with unique first letters
/// Action: Call calculate_unique_prefix with name having unique first letter
/// Result: Function returns single character prefix
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_unique_first_letter() {
    let names = vec!["general".to_string(), "testing".to_string()];
    let result = RequirementsServer::calculate_unique_prefix("general", &names);
    assert_eq!(result, "G");
}

/// Test: calculate_unique_prefix with conflicting first letter
/// Precondition: System has multiple names with same first letter
/// Action: Call calculate_unique_prefix with name having conflicting first letter
/// Result: Function returns prefix with length >= 2 to ensure uniqueness
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_conflicting_first_letter() {
    let names = vec!["general".to_string(), "guidelines".to_string()];
    let result = RequirementsServer::calculate_unique_prefix("general", &names);
    assert!(result.len() >= 2);
    assert!(result.starts_with("G"));
}

// Tests for calculate_chapter_prefix (G.R.4)

/// Test: calculate_chapter_prefix with single name
/// Precondition: System has single chapter name in list
/// Action: Call calculate_chapter_prefix with single name
/// Result: Function returns single character prefix
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_chapter_prefix_single_name() {
    let names = vec!["General Requirements".to_string()];
    let result = RequirementsServer::calculate_chapter_prefix("General Requirements", &names);
    assert_eq!(result, "G");
}

/// Test: calculate_chapter_prefix with unique first letter
/// Precondition: System has multiple chapter names with unique first letters
/// Action: Call calculate_chapter_prefix with name having unique first letter
/// Result: Function returns single character prefix
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_chapter_prefix_unique_first_letter() {
    let names = vec![
        "General Requirements".to_string(),
        "Testing Requirements".to_string(),
    ];
    let result = RequirementsServer::calculate_chapter_prefix("General Requirements", &names);
    assert_eq!(result, "G");
}

// =============================================================================
// Tests for G.R.5: Requirement parsing boundaries
// =============================================================================

// Note: G.R.5 tests are covered in chapter_parsing_tests.rs and requirement_parsing_tests.rs
// These tests verify that requirements are correctly parsed with proper boundaries

// =============================================================================
// Tests for G.R.8: File encoding (UTF-8)
// =============================================================================

/// Test: read_file_utf8 with valid UTF-8 file
/// Precondition: System has valid UTF-8 file
/// Action: Call read_file_utf8 with file path
/// Result: Function returns Ok with file content
/// Covers Requirement: G.R.8
#[test]
fn test_read_file_utf8_valid() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "Test content").unwrap();

    let result = RequirementsServer::read_file_utf8(&file_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Test content");
}

/// Test: read_file_utf8 with UTF-8 special characters
/// Precondition: System has UTF-8 file with special characters (emojis, non-ASCII)
/// Action: Call read_file_utf8 with file path
/// Result: Function returns Ok with file content including special characters
/// Covers Requirement: G.R.8
#[test]
fn test_read_file_utf8_unicode() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    let content = "Test with émojis 🎉 and 中文";
    std::fs::write(&file_path, content).unwrap();

    let result = RequirementsServer::read_file_utf8(&file_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), content);
}

/// Test: read_file_utf8 with non-existent file
/// Precondition: System has non-existent file path
/// Action: Call read_file_utf8 with non-existent file path
/// Result: Function returns error about file not found
/// Covers Requirement: G.R.8, G.R.9
#[test]
fn test_read_file_utf8_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("nonexistent.md");

    let result = RequirementsServer::read_file_utf8(&file_path);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("File not found"));
}

// =============================================================================
// Tests for G.R.9: File system error handling
// =============================================================================

/// Test: write_file_utf8 creates parent directories
/// Precondition: System has file path with non-existent parent directories
/// Action: Call write_file_utf8 with nested file path
/// Result: Function returns Ok and creates parent directories, file exists
/// Covers Requirement: G.R.9
#[test]
fn test_write_file_utf8_creates_dirs() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("nested").join("path").join("test.md");

    let result = RequirementsServer::write_file_utf8(&file_path, "Content");
    assert!(result.is_ok());
    assert!(file_path.exists());
}

/// Test: write_file_utf8 with valid content
/// Precondition: System has file path
/// Action: Call write_file_utf8 with file path and content
/// Result: Function returns Ok and file contains written content
/// Covers Requirement: G.R.9
#[test]
fn test_write_file_utf8_valid() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");

    let result = RequirementsServer::write_file_utf8(&file_path, "Test content");
    assert!(result.is_ok());

    let content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "Test content");
}

// =============================================================================
// Tests for G.R.10: Empty file handling
// =============================================================================

/// Test: is_file_empty_or_whitespace with empty string
/// Precondition: System has empty string
/// Action: Call is_file_empty_or_whitespace with empty string
/// Result: Function returns true
/// Covers Requirement: G.R.10
#[test]
fn test_is_file_empty_or_whitespace_empty() {
    assert!(RequirementsServer::is_file_empty_or_whitespace(""));
}

/// Test: is_file_empty_or_whitespace with only spaces
/// Precondition: System has string containing only spaces
/// Action: Call is_file_empty_or_whitespace with "   "
/// Result: Function returns true
/// Covers Requirement: G.R.10
#[test]
fn test_is_file_empty_or_whitespace_spaces() {
    assert!(RequirementsServer::is_file_empty_or_whitespace("   "));
}

/// Test: is_file_empty_or_whitespace with only newlines
/// Precondition: System has string containing only newlines
/// Action: Call is_file_empty_or_whitespace with "\n\n"
/// Result: Function returns true
/// Covers Requirement: G.R.10
#[test]
fn test_is_file_empty_or_whitespace_newlines() {
    assert!(RequirementsServer::is_file_empty_or_whitespace("\n\n"));
}

/// Test: is_file_empty_or_whitespace with content
/// Precondition: System has string with actual content
/// Action: Call is_file_empty_or_whitespace with "content"
/// Result: Function returns false
/// Covers Requirement: G.R.10
#[test]
fn test_is_file_empty_or_whitespace_with_content() {
    assert!(!RequirementsServer::is_file_empty_or_whitespace("content"));
}

/// Test: read_chapters_streaming with empty file
/// Precondition: System has empty markdown file
/// Action: Call read_chapters_streaming with empty file path
/// Result: Function returns Ok with empty vector
/// Covers Requirement: G.R.10
#[test]
fn test_read_chapters_streaming_empty_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("empty.md");
    std::fs::write(&file_path, "").unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

/// Test: read_chapters_streaming with whitespace-only file
/// Precondition: System has markdown file containing only whitespace
/// Action: Call read_chapters_streaming with whitespace-only file path
/// Result: Function returns Ok with empty vector
/// Covers Requirement: G.R.10
#[test]
fn test_read_chapters_streaming_whitespace_only() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("whitespace.md");
    std::fs::write(&file_path, "   \n\t\n  ").unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

/// Test: read_requirements_streaming with empty file
/// Precondition: System has empty markdown file
/// Action: Call read_requirements_streaming with empty file path and chapter name
/// Result: Function returns Ok with empty vector
/// Covers Requirement: G.R.10
#[test]
fn test_read_requirements_streaming_empty_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("empty.md");
    std::fs::write(&file_path, "").unwrap();

    let result = RequirementsServer::read_requirements_streaming(&file_path, "Chapter");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

/// Test: read_requirements_streaming with chapter having no requirements
/// Precondition: System has markdown file with chapter but no requirements
/// Action: Call read_requirements_streaming with file path and chapter name
/// Result: Function returns Ok with empty vector
/// Covers Requirement: G.R.10
#[test]
fn test_read_requirements_streaming_empty_chapter() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "# Test Chapter\n\nNo requirements here.").unwrap();

    let result = RequirementsServer::read_requirements_streaming(&file_path, "Test Chapter");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

// =============================================================================
// Tests for G.R.11: Blank line before headings
// =============================================================================

// Note: G.R.11 tests are covered in integration_tests.rs (tool_update_requirement_tests.rs)

// =============================================================================
// Tests for G.R.12: Exact heading match
// =============================================================================

// Note: G.R.12 tests are covered in integration_tests.rs

// =============================================================================
// Tests for G.R.11 (Blank line before headings)
// =============================================================================

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
    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
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
    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
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
    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    // Should have blank line before ## G.T.3
    assert!(
        file_content.contains("Content one.\n\n## G.T.3"),
        "Should have blank line before next requirement after deletion. Content: {}",
        file_content
    );
}
