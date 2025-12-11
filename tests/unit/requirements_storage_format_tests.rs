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

/// Test: read_chapters_streaming with indented headings
/// Precondition: System has a category file with indented chapter headings
/// Action: Call read_chapters_streaming with file containing " # Chapter" (1-3 spaces)
/// Result: Function parses indented headings correctly
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_indented() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, " # Chapter One\n   # Chapter Two\n").unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 2);
}

/// Test: read_chapters_streaming with empty file
/// Precondition: System has an empty category file
/// Action: Call read_chapters_streaming with empty file
/// Result: Function returns empty vec
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_empty() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "").unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Vec::<String>::new());
}

/// Test: read_chapters_streaming with level-2 headings (should ignore)
/// Precondition: System has a category file with level-2 headings
/// Action: Call read_chapters_streaming with file containing "## Requirement"
/// Result: Function ignores level-2 headings
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_ignore_level2() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "# Chapter\n## Requirement\n").unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Chapter");
}

/// Test: read_chapters_streaming with multi-line code block
/// Precondition: System has a category file with multi-line code block
/// Action: Call read_chapters_streaming with file containing code block spanning multiple lines
/// Result: Function correctly tracks code block boundaries
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_multiline_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Real Chapter\n```\nline1\nline2\n# Fake Chapter\nline3\n```\n# Another Chapter\n",
    )
    .unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 2);
    assert_eq!(chapters[0], "Real Chapter");
    assert_eq!(chapters[1], "Another Chapter");
}

/// Test: read_chapters_streaming with code block language identifier
/// Precondition: System has a category file with code block having language identifier
/// Action: Call read_chapters_streaming with file containing "```json" code block
/// Result: Function correctly identifies code block boundaries
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_code_block_language() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "# Real Chapter\n```json\n# Fake Chapter\n```\n").unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming with trailing spaces in chapter name
/// Precondition: System has a category file with chapter heading having trailing spaces
/// Action: Call read_chapters_streaming with file containing "# Chapter   "
/// Result: Function trims trailing spaces from chapter name
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_trailing_spaces() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "# Chapter One   \n").unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Chapter One");
}

/// Test: read_chapters_streaming with unicode chapter names
/// Precondition: System has a category file with unicode chapter names
/// Action: Call read_chapters_streaming with file containing "# Глава"
/// Result: Function correctly parses unicode chapter names
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_unicode() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "# Глава\n").unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Глава");
}

/// Test: read_chapters_streaming ignoring "## Categories" mention in code block
/// Precondition: System has a category file with "## Categories" mentioned in code block
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```\n## Categories\n```"
/// Result: Function ignores "## Categories" inside code block
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_categories_in_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "# Real Chapter\n```\n## Categories\n```\n").unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring "# Categories" mention in code block
/// Precondition: System has a category file with "# Categories" mentioned in code block
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```\n# Categories\n```"
/// Result: Function ignores "# Categories" inside code block
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_categories_level1_in_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "# Real Chapter\n```\n# Categories\n```\n").unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in requirement text
/// Precondition: System has a category file with chapter mention in requirement text (not code block)
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n## G.G.1: Title\nText with # Categories mention\n"
/// Result: Function ignores "# Categories" mention in requirement text (not a real heading)
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_ignore_chapter_mention_in_text() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Real Chapter\n## G.G.1: Title\nText with # Categories mention\n",
    )
    .unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring "## Categories" in requirement text
/// Precondition: System has a category file with "## Categories" in requirement text
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n## G.G.1: Title\nText with ## Categories\n"
/// Result: Function ignores "## Categories" in requirement text (not a real heading)
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_ignore_categories_level2_in_text() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Real Chapter\n## G.G.1: Title\nText with ## Categories\n",
    )
    .unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in json code block
/// Precondition: System has a category file with chapter mention in json code block
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```json\n{\"heading\": \"# Categories\"}\n```"
/// Result: Function ignores chapter mention inside json code block
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_chapter_in_json_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Real Chapter\n```json\n{\"heading\": \"# Categories\"}\n```\n",
    )
    .unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in markdown code block
/// Precondition: System has a category file with chapter mention in markdown code block
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```markdown\n# Categories\n```"
/// Result: Function ignores chapter mention inside markdown code block
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_chapter_in_markdown_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Real Chapter\n```markdown\n# Categories\n```\n",
    )
    .unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring multiple chapter mentions in code block
/// Precondition: System has a category file with multiple chapter mentions in code block
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```\n# Categories\n# Chapters\n# Chapter List\n```"
/// Result: Function ignores all chapter mentions inside code block
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_multiple_chapters_in_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Real Chapter\n```\n# Categories\n# Chapters\n# Chapter List\n```\n",
    )
    .unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in nested code block
/// Precondition: System has a category file with chapter mention in nested code block structure
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```\nouter\n```\n```\n# Categories\n```"
/// Result: Function correctly handles nested code block boundaries
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_chapter_in_nested_code_blocks() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Real Chapter\n```\nouter\n```\n```\n# Categories\n```\n",
    )
    .unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in requirement body text
/// Precondition: System has a category file with chapter mention in requirement body
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n## G.G.1: Title\nThis mentions # Categories in the text\n"
/// Result: Function ignores chapter mention in requirement body text
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_ignore_chapter_mention_in_requirement_body() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Real Chapter\n## G.G.1: Title\nThis mentions # Categories in the text\n",
    )
    .unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring "## Categories" in inline code
/// Precondition: System has a category file with "## Categories" in inline code (not fenced block)
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\nText with `## Categories` inline\n"
/// Result: Function ignores inline code (not fenced block, so not tracked)
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_ignore_chapter_in_inline_code() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Real Chapter\nText with `## Categories` inline\n",
    )
    .unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming with real chapter after code block with mention
/// Precondition: System has a category file with real chapter after code block containing chapter mention
/// Action: Call read_chapters_streaming with file containing "# First\n```\n# Categories\n```\n# Second Chapter\n"
/// Result: Function correctly identifies real chapter after code block
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_real_chapter_after_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# First Chapter\n```\n# Categories\n```\n# Second Chapter\n",
    )
    .unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 2);
    assert_eq!(chapters[0], "First Chapter");
    assert_eq!(chapters[1], "Second Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in requirement text with indentation
/// Precondition: System has a category file with indented chapter mention in requirement text
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n## G.G.1: Title\n  Text with # Categories\n"
/// Result: Function ignores indented chapter mention in requirement text
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_ignore_indented_chapter_mention() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Real Chapter\n## G.G.1: Title\n  Text with # Categories\n",
    )
    .unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in multi-line code block
/// Precondition: System has a category file with chapter mention in multi-line code block
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```\nline1\n# Categories\nline2\n```"
/// Result: Function ignores chapter mention in multi-line code block
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_chapter_in_multiline_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Real Chapter\n```\nline1\n# Categories\nline2\n```\n",
    )
    .unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring "## Categories" in code block with language
/// Precondition: System has a category file with "## Categories" in code block with language identifier
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```python\n## Categories\n```"
/// Result: Function ignores chapter mention in code block with language
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_categories_in_python_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Real Chapter\n```python\n## Categories\n```\n",
    )
    .unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in requirement text with special chars
/// Precondition: System has a category file with chapter mention containing special characters
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n## G.G.1: Title\nText: # Categories (list)\n"
/// Result: Function ignores chapter mention with special characters in requirement text
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_ignore_chapter_with_special_chars() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Real Chapter\n## G.G.1: Title\nText: # Categories (list)\n",
    )
    .unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in code block at file start
/// Precondition: System has a category file starting with code block containing chapter mention
/// Action: Call read_chapters_streaming with file containing "```\n# Categories\n```\n# Real Chapter\n"
/// Result: Function ignores chapter mention in code block at start
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_chapter_at_file_start_in_code() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "```\n# Categories\n```\n# Real Chapter\n").unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in code block at file end
/// Precondition: System has a category file ending with code block containing chapter mention
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```\n# Categories\n```"
/// Result: Function ignores chapter mention in code block at end
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_chapter_at_file_end_in_code() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "# Real Chapter\n```\n# Categories\n```").unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming with "Categories" as real chapter
/// Precondition: System has a category file with actual chapter named "Categories"
/// Action: Call read_chapters_streaming with file containing "# Categories\n"
/// Result: Function returns "Categories" as a valid chapter
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_real_categories_chapter() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "# Categories\n").unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Categories");
}

/// Test: read_chapters_streaming ignoring chapter mention in code block between real chapters
/// Precondition: System has a category file with chapter mention in code block between real chapters
/// Action: Call read_chapters_streaming with file containing "# First\n```\n# Categories\n```\n# Second\n"
/// Result: Function correctly identifies both real chapters, ignoring mention in code block
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_chapter_between_real_chapters() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# First Chapter\n```\n# Categories\n```\n# Second Chapter\n",
    )
    .unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 2);
    assert_eq!(chapters[0], "First Chapter");
    assert_eq!(chapters[1], "Second Chapter");
}

/// Test: read_chapters_streaming ignoring "## Categories" in requirement text with formatting
/// Precondition: System has a category file with formatted "## Categories" mention in requirement text
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n## G.G.1: Title\nText with **## Categories** bold\n"
/// Result: Function ignores formatted chapter mention in requirement text
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_ignore_formatted_chapter_mention() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Real Chapter\n## G.G.1: Title\nText with **## Categories** bold\n",
    )
    .unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming with "Categories" and other chapters
/// Precondition: System has a category file with "# Categories" chapter heading
/// Action: Call read_chapters_streaming with file containing "# Categories\n# Real Chapter\n"
/// Result: Function returns both chapters including "Categories"
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_categories_with_other() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "# Categories\n# Real Chapter\n").unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 2);
    assert_eq!(chapters[0], "Categories");
    assert_eq!(chapters[1], "Real Chapter");
}

/// Test: read_chapters_streaming with "Categories" among other chapters
/// Precondition: System has a category file with "Categories" and other chapters
/// Action: Call read_chapters_streaming with file containing "# Chapter One\n# Categories\n# Chapter Two\n"
/// Result: Function returns all chapters including "Categories"
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_categories_with_others() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "# Chapter One\n# Categories\n# Chapter Two\n").unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 3);
    assert_eq!(chapters[0], "Chapter One");
    assert_eq!(chapters[1], "Categories");
    assert_eq!(chapters[2], "Chapter Two");
}

/// Test: read_chapters_streaming with "Categories" both in code block and as real heading
/// Precondition: System has a category file with "# Categories" in code block and real "# Categories" chapter
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```\n# Categories\n```\n# Categories\n# Another Chapter\n"
/// Result: Function ignores "Categories" in code block and includes real "Categories" chapter
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_categories_in_code_block_and_real() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Real Chapter\n```\n# Categories\n```\n# Categories\n# Another Chapter\n",
    )
    .unwrap();

    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 3);
    assert_eq!(chapters[0], "Real Chapter");
    assert_eq!(chapters[1], "Categories");
    assert_eq!(chapters[2], "Another Chapter");
}

/// Test: parse_level2_heading with missing colon
/// Precondition: System has a markdown level-2 heading without colon separator
/// Action: Call parse_level2_heading with "## G.G.1 Requirement Title"
/// Result: Function returns None
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_no_colon() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1 Requirement Title");
    assert_eq!(result, None);
}

/// Test: parse_level1_heading with trailing spaces
/// Precondition: System has a markdown level-1 heading with trailing spaces
/// Action: Call parse_level1_heading with "# Chapter Name   "
/// Result: Function returns Some("Chapter Name") (trailing spaces trimmed)
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_trailing_spaces() {
    let result = RequirementsServer::parse_level1_heading("# Chapter Name   ");
    assert_eq!(result, Some("Chapter Name".to_string()));
}

/// Test: parse_level1_heading with 4 spaces indentation (too many)
/// Precondition: System has a markdown level-1 heading with 4 spaces indentation
/// Action: Call parse_level1_heading with "    # Chapter Name"
/// Result: Function returns None (4 spaces is too many)
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_four_space_indent() {
    let result = RequirementsServer::parse_level1_heading("    # Chapter Name");
    assert_eq!(result, None);
}

/// Test: parse_level1_heading with trailing hash characters
/// Precondition: System has a markdown level-1 heading with trailing hashes
/// Action: Call parse_level1_heading with "# Chapter Name ###"
/// Result: Function returns Some("Chapter Name") (trailing hashes removed per standard markdown)
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_trailing_hashes() {
    let result = RequirementsServer::parse_level1_heading("# Chapter Name ###");
    assert_eq!(result, Some("Chapter Name".to_string()));
}

/// Test: parse_level1_heading with empty chapter name
/// Precondition: System has a markdown level-1 heading with empty name
/// Action: Call parse_level1_heading with "# "
/// Result: Function returns Some("") (empty string)
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_empty_name() {
    let result = RequirementsServer::parse_level1_heading("# ");
    assert_eq!(result, Some("".to_string()));
}

/// Test: parse_level2_heading with empty title
/// Precondition: System has a markdown level-2 heading with empty title
/// Action: Call parse_level2_heading with "## G.G.1: "
/// Result: Function returns None (empty title is invalid)
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_empty_title() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1: ");
    assert_eq!(result, None);
}

/// Test: parse_level2_heading with spaces around colon
/// Precondition: System has a markdown level-2 heading with spaces around colon
/// Action: Call parse_level2_heading with "## G.G.1 : Requirement Title"
/// Result: Function returns Some(("G.G.1", "Requirement Title")) (spaces trimmed)
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_spaces_around_colon() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1 : Requirement Title");
    assert_eq!(
        result,
        Some(("G.G.1".to_string(), "Requirement Title".to_string()))
    );
}

/// Test: parse_level2_heading with trailing hash characters
/// Precondition: System has a markdown level-2 heading with trailing hashes
/// Action: Call parse_level2_heading with "## G.G.1: Title ###"
/// Result: Function returns Some(("G.G.1", "Title")) (trailing hashes removed per standard markdown)
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_trailing_hashes() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1: Title ###");
    assert_eq!(result, Some(("G.G.1".to_string(), "Title".to_string())));
}

/// Test: parse_level2_heading with complex index format
/// Precondition: System has a markdown level-2 heading with complex index
/// Action: Call parse_level2_heading with "## GET.GET_C.123: Complex Requirement Title"
/// Result: Function returns Some(("GET.GET_C.123", "Complex Requirement Title"))
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_complex_index() {
    let result =
        RequirementsServer::parse_level2_heading("## GET.GET_C.123: Complex Requirement Title");
    assert_eq!(
        result,
        Some((
            "GET.GET_C.123".to_string(),
            "Complex Requirement Title".to_string()
        ))
    );
}

/// Test: parse_level2_heading with title containing colon
/// Precondition: System has a markdown level-2 heading with colon in title
/// Action: Call parse_level2_heading with "## G.G.1: Title: Subtitle"
/// Result: Function returns Some(("G.G.1", "Title: Subtitle")) (first colon is separator)
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_colon_in_title() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1: Title: Subtitle");
    assert_eq!(
        result,
        Some(("G.G.1".to_string(), "Title: Subtitle".to_string()))
    );
}

/// Test: parse_level1_heading with only hash and space
/// Precondition: System has a markdown level-1 heading with only "# "
/// Action: Call parse_level1_heading with "# "
/// Result: Function returns Some("") (empty name)
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_only_hash_space() {
    let result = RequirementsServer::parse_level1_heading("# ");
    assert_eq!(result, Some("".to_string()));
}

/// Test: parse_level1_heading with tab character
/// Precondition: System has a markdown level-1 heading with tab character
/// Action: Call parse_level1_heading with "\t# Chapter Name"
/// Result: Function returns None (tabs are not spaces)
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_tab() {
    let result = RequirementsServer::parse_level1_heading("\t# Chapter Name");
    assert_eq!(result, None);
}

/// Test: parse_level2_heading with tab character
/// Precondition: System has a markdown level-2 heading with tab character
/// Action: Call parse_level2_heading with "\t## G.G.1: Title"
/// Result: Function returns None (tabs are not spaces)
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_tab() {
    let result = RequirementsServer::parse_level2_heading("\t## G.G.1: Title");
    assert_eq!(result, None);
}

/// Test: parse_level1_heading with unicode characters
/// Precondition: System has a markdown level-1 heading with unicode characters
/// Action: Call parse_level1_heading with "# Глава"
/// Result: Function returns Some("Глава")
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_unicode() {
    let result = RequirementsServer::parse_level1_heading("# Глава");
    assert_eq!(result, Some("Глава".to_string()));
}

/// Test: parse_level2_heading with unicode characters
/// Precondition: System has a markdown level-2 heading with unicode characters
/// Action: Call parse_level2_heading with "## G.G.1: Требование"
/// Result: Function returns Some(("G.G.1", "Требование"))
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_unicode() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1: Требование");
    assert_eq!(
        result,
        Some(("G.G.1".to_string(), "Требование".to_string()))
    );
}

/// Test: parse_level1_heading with special characters
/// Precondition: System has a markdown level-1 heading with special characters
/// Action: Call parse_level1_heading with "# Chapter-Name_123"
/// Result: Function returns Some("Chapter-Name_123")
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_special_chars() {
    let result = RequirementsServer::parse_level1_heading("# Chapter-Name_123");
    assert_eq!(result, Some("Chapter-Name_123".to_string()));
}

/// Test: parse_level2_heading with special characters in title
/// Precondition: System has a markdown level-2 heading with special characters in title
/// Action: Call parse_level2_heading with "## G.G.1: Title-Name_123"
/// Result: Function returns Some(("G.G.1", "Title-Name_123"))
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_special_chars_title() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1: Title-Name_123");
    assert_eq!(
        result,
        Some(("G.G.1".to_string(), "Title-Name_123".to_string()))
    );
}

/// Test: parse_level2_heading with multiple spaces in title
/// Precondition: System has a markdown level-2 heading with multiple spaces in title
/// Action: Call parse_level2_heading with "## G.G.1: Title   With   Spaces"
/// Result: Function returns Some(("G.G.1", "Title   With   Spaces")) (spaces preserved)
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_multiple_spaces() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1: Title   With   Spaces");
    assert_eq!(
        result,
        Some(("G.G.1".to_string(), "Title   With   Spaces".to_string()))
    );
}

/// Test: parse_level1_heading with multiple spaces in name
/// Precondition: System has a markdown level-1 heading with multiple spaces in name
/// Action: Call parse_level1_heading with "# Chapter   Name"
/// Result: Function returns Some("Chapter   Name") (spaces preserved)
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_multiple_spaces() {
    let result = RequirementsServer::parse_level1_heading("# Chapter   Name");
    assert_eq!(result, Some("Chapter   Name".to_string()));
}

/// Test: read_requirements_streaming ignoring requirements in code blocks
/// Precondition: System has a category file with requirement heading inside code block
/// Action: Call read_requirements_streaming with file containing "## G.G.1: Title" inside ```
/// Result: Function ignores requirement heading inside code block
/// Covers Requirement: G.REQLIX_GET_REQ.3, G.R.3, G.R.5
#[test]
fn test_read_requirements_streaming_ignore_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Chapter\n## G.G.1: Real Title\n```\n## G.G.2: Fake Title\n```\n",
    )
    .unwrap();

    let result = RequirementsServer::read_requirements_streaming(&file_path, "Chapter");
    assert!(result.is_ok());
    let requirements = result.unwrap();
    assert_eq!(requirements.len(), 1);
    assert_eq!(requirements[0].index, "G.G.1");
    assert_eq!(requirements[0].title, "Real Title");
}

/// Test: read_requirements_streaming with requirements in different chapters
/// Precondition: System has a category file with requirements in multiple chapters
/// Action: Call read_requirements_streaming for specific chapter
/// Result: Function returns only requirements from specified chapter
/// Covers Requirement: G.REQLIX_GET_REQ.3, G.R.3, G.R.5
#[test]
fn test_read_requirements_streaming_different_chapters() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Chapter One\n## G.G.1: Title One\n# Chapter Two\n## G.G.2: Title Two\n",
    )
    .unwrap();

    let result = RequirementsServer::read_requirements_streaming(&file_path, "Chapter One");
    assert!(result.is_ok());
    let requirements = result.unwrap();
    assert_eq!(requirements.len(), 1);
    assert_eq!(requirements[0].index, "G.G.1");
    assert_eq!(requirements[0].title, "Title One");
}

/// Test: read_requirements_streaming with indented headings
/// Precondition: System has a category file with indented requirement headings
/// Action: Call read_requirements_streaming with file containing "  ## G.G.1: Title" (1-3 spaces)
/// Result: Function parses indented headings correctly
/// Covers Requirement: G.REQLIX_GET_REQ.3, G.R.3, G.R.5
#[test]
fn test_read_requirements_streaming_indented() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "# Chapter\n  ## G.G.1: Title\n").unwrap();

    let result = RequirementsServer::read_requirements_streaming(&file_path, "Chapter");
    assert!(result.is_ok());
    let requirements = result.unwrap();
    assert_eq!(requirements.len(), 1);
    assert_eq!(requirements[0].index, "G.G.1");
}

/// Test: read_requirements_streaming with level-1 headings (should ignore)
/// Precondition: System has a category file with level-1 headings in chapter
/// Action: Call read_requirements_streaming with file containing "# Subchapter"
/// Result: Function ignores level-1 headings when collecting requirements
/// Covers Requirement: G.REQLIX_GET_REQ.3, G.R.3, G.R.5
#[test]
fn test_read_requirements_streaming_ignore_level1() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Chapter\n## G.G.1: Title\n# Subchapter\n## G.G.2: Title Two\n",
    )
    .unwrap();

    let result = RequirementsServer::read_requirements_streaming(&file_path, "Chapter");
    assert!(result.is_ok());
    let requirements = result.unwrap();
    // Should include both requirements since Subchapter is treated as part of Chapter
    assert!(!requirements.is_empty());
}

/// Test: find_requirement_streaming with simple requirement
/// Precondition: System has a category file with a requirement
/// Action: Call find_requirement_streaming with index "G.G.1"
/// Result: Function returns RequirementFull with correct index, title, and text
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4, G.R.5
#[test]
fn test_find_requirement_streaming_simple() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "# Chapter\n## G.G.1: Title\nText content\n").unwrap();

    let result = RequirementsServer::find_requirement_streaming(&file_path, "test", "G.G.1");
    assert!(result.is_ok());
    let req = result.unwrap();
    assert_eq!(req.index, "G.G.1");
    assert_eq!(req.title, "Title");
    assert_eq!(req.text, "Text content");
    assert_eq!(req.category, "test");
    assert_eq!(req.chapter, "Chapter");
}

/// Test: find_requirement_streaming with requirement at end of file
/// Precondition: System has a category file with requirement at end
/// Action: Call find_requirement_streaming for last requirement
/// Result: Function correctly includes all text until EOF
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4, G.R.5
#[test]
fn test_find_requirement_streaming_end_of_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "# Chapter\n## G.G.1: Title\nLine 1\nLine 2\n").unwrap();

    let result = RequirementsServer::find_requirement_streaming(&file_path, "test", "G.G.1");
    assert!(result.is_ok());
    let req = result.unwrap();
    assert_eq!(req.index, "G.G.1");
    assert!(req.text.contains("Line 1"));
    assert!(req.text.contains("Line 2"));
}

/// Test: find_requirement_streaming with code block in requirement
/// Precondition: System has a category file with requirement containing code block
/// Action: Call find_requirement_streaming for requirement with code block
/// Result: Function includes code block content in requirement text
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4, G.R.5
#[test]
fn test_find_requirement_streaming_with_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Chapter\n## G.G.1: Title\n```\ncode line\n```\n",
    )
    .unwrap();

    let result = RequirementsServer::find_requirement_streaming(&file_path, "test", "G.G.1");
    assert!(result.is_ok());
    let req = result.unwrap();
    assert!(req.text.contains("```"));
    assert!(req.text.contains("code line"));
}

/// Test: find_requirement_streaming with requirement boundary at next requirement
/// Precondition: System has a category file with multiple requirements
/// Action: Call find_requirement_streaming for first requirement
/// Result: Function stops at next requirement heading
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4, G.R.5
#[test]
fn test_find_requirement_streaming_boundary_next_requirement() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Chapter\n## G.G.1: Title One\nText one\n## G.G.2: Title Two\nText two\n",
    )
    .unwrap();

    let result = RequirementsServer::find_requirement_streaming(&file_path, "test", "G.G.1");
    assert!(result.is_ok());
    let req = result.unwrap();
    assert_eq!(req.index, "G.G.1");
    assert!(req.text.contains("Text one"));
    assert!(!req.text.contains("Title Two"));
}

/// Test: find_requirement_streaming with level-1 heading ends requirement
/// Precondition: System has a category file with level-1 heading after requirement
/// Action: Call find_requirement_streaming for requirement before "# NextChapter"
/// Result: Function does NOT include level-1 heading in requirement text (G.R.5: ends at same or higher level)
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4, G.R.5
#[test]
fn test_find_requirement_streaming_level1_ends_requirement() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Chapter\n## G.G.1: Title\nText before\n# NextChapter\nText after\n",
    )
    .unwrap();

    let result = RequirementsServer::find_requirement_streaming(&file_path, "test", "G.G.1");
    assert!(result.is_ok());
    let req = result.unwrap();
    assert!(req.text.contains("Text before"));
    // Level-1 heading should NOT be part of requirement text (G.R.5)
    assert!(
        !req.text.contains("# NextChapter"),
        "Level-1 heading should end requirement, not be included in text"
    );
    assert!(
        !req.text.contains("Text after"),
        "Content after level-1 heading should not be in requirement"
    );
}

/// Test: find_requirement_streaming with requirement not found
/// Precondition: System has a category file without specified requirement
/// Action: Call find_requirement_streaming with non-existent index
/// Result: Function returns error "Requirement not found"
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4
#[test]
fn test_find_requirement_streaming_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(&file_path, "# Chapter\n## G.G.1: Title\n").unwrap();

    let result = RequirementsServer::find_requirement_streaming(&file_path, "test", "G.G.999");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

/// Test: find_requirement_streaming with multi-line code block
/// Precondition: System has a category file with multi-line code block in requirement
/// Action: Call find_requirement_streaming for requirement with multi-line code
/// Result: Function correctly tracks code block boundaries and includes all content
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4, G.R.5
#[test]
fn test_find_requirement_streaming_multiline_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Chapter\n## G.G.1: Title\n```\nline1\nline2\nline3\n```\nMore text\n",
    )
    .unwrap();

    let result = RequirementsServer::find_requirement_streaming(&file_path, "test", "G.G.1");
    assert!(result.is_ok());
    let req = result.unwrap();
    assert!(req.text.contains("line1"));
    assert!(req.text.contains("line2"));
    assert!(req.text.contains("line3"));
    assert!(req.text.contains("More text"));
}

/// Test: find_requirement_streaming with code block language identifier
/// Precondition: System has a category file with code block having language identifier
/// Action: Call find_requirement_streaming for requirement with "```json" code block
/// Result: Function correctly identifies code block boundaries
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4, G.R.5
#[test]
fn test_find_requirement_streaming_code_block_language() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Chapter\n## G.G.1: Title\n```json\n{\"key\": \"value\"}\n```\n",
    )
    .unwrap();

    let result = RequirementsServer::find_requirement_streaming(&file_path, "test", "G.G.1");
    assert!(result.is_ok());
    let req = result.unwrap();
    assert!(req.text.contains("```json"));
    assert!(req.text.contains("{\"key\": \"value\"}"));
}
