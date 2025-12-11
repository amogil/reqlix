// Tests for new requirements: G.P.3, G.R.8, G.R.9, G.R.10

use reqlix::RequirementsServer;
use tempfile::TempDir;

// =============================================================================
// Tests for G.P.3: Name validation
// =============================================================================

/// Test: validate_category with valid name
/// Precondition: System has a valid category name
/// Action: Call validate_category with valid name
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_valid() {
    let result = RequirementsServer::validate_category("general");
    assert!(result.is_ok());
}

/// Test: validate_category with name containing invalid filename character
/// Precondition: System has a category name with invalid character
/// Action: Call validate_category with name containing '/'
/// Result: Function returns error about invalid character
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_invalid_char_slash() {
    let result = RequirementsServer::validate_category("test/category");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("invalid character"));
}

/// Test: validate_category with name containing backslash
/// Precondition: System has a category name with backslash
/// Action: Call validate_category with name containing '\'
/// Result: Function returns error about invalid character
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_invalid_char_backslash() {
    let result = RequirementsServer::validate_category("test\\category");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("invalid character"));
}

/// Test: validate_category with reserved name AGENTS
/// Precondition: System has category name "AGENTS"
/// Action: Call validate_category with "AGENTS"
/// Result: Function returns error about reserved name
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_reserved_name() {
    let result = RequirementsServer::validate_category("AGENTS");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("reserved"));
}

/// Test: validate_category with name starting with whitespace
/// Precondition: System has category name starting with space
/// Action: Call validate_category with " category"
/// Result: Function returns error about whitespace
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_leading_whitespace() {
    let result = RequirementsServer::validate_category(" category");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("whitespace"));
}

/// Test: validate_category with name ending with whitespace
/// Precondition: System has category name ending with space
/// Action: Call validate_category with "category "
/// Result: Function returns error about whitespace
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_trailing_whitespace() {
    let result = RequirementsServer::validate_category("category ");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("whitespace"));
}

/// Test: validate_category with consecutive dots
/// Precondition: System has category name with ".."
/// Action: Call validate_category with "test..category"
/// Result: Function returns error about consecutive dots
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_consecutive_dots() {
    let result = RequirementsServer::validate_category("test..category");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("consecutive dots"));
}

/// Test: validate_category with single dot
/// Precondition: System has category name "."
/// Action: Call validate_category with "."
/// Result: Function returns error
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_single_dot() {
    let result = RequirementsServer::validate_category(".");
    assert!(result.is_err());
}

/// Test: validate_category with double dot
/// Precondition: System has category name ".."
/// Action: Call validate_category with ".."
/// Result: Function returns error
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_double_dot() {
    let result = RequirementsServer::validate_category("..");
    assert!(result.is_err());
}

/// Test: validate_chapter with valid name
/// Precondition: System has a valid chapter name
/// Action: Call validate_chapter with valid name
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.3
#[test]
fn test_validate_chapter_valid() {
    let result = RequirementsServer::validate_chapter("General Requirements");
    assert!(result.is_ok());
}

/// Test: validate_chapter with name containing newline
/// Precondition: System has chapter name with newline
/// Action: Call validate_chapter with name containing '\n'
/// Result: Function returns error about newline
/// Covers Requirement: G.P.3
#[test]
fn test_validate_chapter_newline() {
    let result = RequirementsServer::validate_chapter("Chapter\nName");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("newline"));
}

/// Test: validate_chapter with name starting with whitespace
/// Precondition: System has chapter name starting with space
/// Action: Call validate_chapter with " Chapter"
/// Result: Function returns error about whitespace
/// Covers Requirement: G.P.3
#[test]
fn test_validate_chapter_leading_whitespace() {
    let result = RequirementsServer::validate_chapter(" Chapter");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("whitespace"));
}

/// Test: validate_chapter with name ending with whitespace
/// Precondition: System has chapter name ending with space
/// Action: Call validate_chapter with "Chapter "
/// Result: Function returns error about whitespace
/// Covers Requirement: G.P.3
#[test]
fn test_validate_chapter_trailing_whitespace() {
    let result = RequirementsServer::validate_chapter("Chapter ");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("whitespace"));
}

// =============================================================================
// Tests for G.R.8: File encoding (UTF-8)
// =============================================================================

/// Test: read_file_utf8 with valid UTF-8 file
/// Precondition: System has a valid UTF-8 file
/// Action: Call read_file_utf8 with valid file
/// Result: Function returns file content
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
/// Precondition: System has UTF-8 file with special characters
/// Action: Call read_file_utf8 with file containing emoji and unicode
/// Result: Function returns file content correctly
/// Covers Requirement: G.R.8
#[test]
fn test_read_file_utf8_unicode() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    let content = "Test with émojis 🎉 and 中文";
    fs::write(&file_path, content).unwrap();
    
    let result = RequirementsServer::read_file_utf8(&file_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), content);
}

/// Test: read_file_utf8 with non-existent file
/// Precondition: System has non-existent file path
/// Action: Call read_file_utf8 with non-existent file
/// Result: Function returns error
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
/// Precondition: System has path with non-existent parent directories
/// Action: Call write_file_utf8 with nested path
/// Result: Function creates directories and writes file
/// Covers Requirement: G.R.9, G.C.2
#[test]
fn test_write_file_utf8_creates_dirs() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("nested").join("path").join("test.md");
    
    let result = RequirementsServer::write_file_utf8(&file_path, "Content");
    assert!(result.is_ok());
    assert!(file_path.exists());
}

/// Test: write_file_utf8 with valid content
/// Precondition: System has valid file path
/// Action: Call write_file_utf8 with content
/// Result: Function writes file successfully
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
/// Action: Call is_file_empty_or_whitespace with ""
/// Result: Function returns true
/// Covers Requirement: G.R.10
#[test]
fn test_is_file_empty_or_whitespace_empty() {
    assert!(RequirementsServer::is_file_empty_or_whitespace(""));
}

/// Test: is_file_empty_or_whitespace with only spaces
/// Precondition: System has string with only spaces
/// Action: Call is_file_empty_or_whitespace with "   "
/// Result: Function returns true
/// Covers Requirement: G.R.10
#[test]
fn test_is_file_empty_or_whitespace_spaces() {
    assert!(RequirementsServer::is_file_empty_or_whitespace("   "));
}

/// Test: is_file_empty_or_whitespace with only newlines
/// Precondition: System has string with only newlines
/// Action: Call is_file_empty_or_whitespace with "\n\n"
/// Result: Function returns true
/// Covers Requirement: G.R.10
#[test]
fn test_is_file_empty_or_whitespace_newlines() {
    assert!(RequirementsServer::is_file_empty_or_whitespace("\n\n"));
}

/// Test: is_file_empty_or_whitespace with tabs
/// Precondition: System has string with only tabs
/// Action: Call is_file_empty_or_whitespace with "\t\t"
/// Result: Function returns true
/// Covers Requirement: G.R.10
#[test]
fn test_is_file_empty_or_whitespace_tabs() {
    assert!(RequirementsServer::is_file_empty_or_whitespace("\t\t"));
}

/// Test: is_file_empty_or_whitespace with mixed whitespace
/// Precondition: System has string with mixed whitespace
/// Action: Call is_file_empty_or_whitespace with " \n\t "
/// Result: Function returns true
/// Covers Requirement: G.R.10
#[test]
fn test_is_file_empty_or_whitespace_mixed() {
    assert!(RequirementsServer::is_file_empty_or_whitespace(" \n\t "));
}

/// Test: is_file_empty_or_whitespace with content
/// Precondition: System has string with content
/// Action: Call is_file_empty_or_whitespace with "content"
/// Result: Function returns false
/// Covers Requirement: G.R.10
#[test]
fn test_is_file_empty_or_whitespace_with_content() {
    assert!(!RequirementsServer::is_file_empty_or_whitespace("content"));
}

/// Test: read_chapters_streaming with empty file
/// Precondition: System has empty category file
/// Action: Call read_chapters_streaming with empty file
/// Result: Function returns empty vector
/// Covers Requirement: G.R.10
#[test]
fn test_read_chapters_streaming_empty_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("empty.md");
    fs::write(&file_path, "").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

/// Test: read_chapters_streaming with whitespace-only file
/// Precondition: System has file with only whitespace
/// Action: Call read_chapters_streaming with whitespace file
/// Result: Function returns empty vector
/// Covers Requirement: G.R.10
#[test]
fn test_read_chapters_streaming_whitespace_only() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("whitespace.md");
    fs::write(&file_path, "   \n\t\n  ").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

/// Test: read_requirements_streaming with empty file
/// Precondition: System has empty category file
/// Action: Call read_requirements_streaming with empty file
/// Result: Function returns empty vector
/// Covers Requirement: G.R.10
#[test]
fn test_read_requirements_streaming_empty_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("empty.md");
    fs::write(&file_path, "").unwrap();
    
    let result = RequirementsServer::read_requirements_streaming(&file_path, "Chapter");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

/// Test: read_requirements_streaming with chapter having no requirements
/// Precondition: System has file with chapter but no requirements
/// Action: Call read_requirements_streaming with chapter
/// Result: Function returns empty vector
/// Covers Requirement: G.R.10
#[test]
fn test_read_requirements_streaming_empty_chapter() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Test Chapter\n\nNo requirements here.").unwrap();
    
    let result = RequirementsServer::read_requirements_streaming(&file_path, "Test Chapter");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

/// Test: validate_category with all invalid filename characters
/// Precondition: System has category name with various invalid characters
/// Action: Call validate_category with each invalid character
/// Result: Function returns error for each
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_all_invalid_chars() {
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    for &ch in &invalid_chars {
        let name = format!("test{}category", ch);
        let result = RequirementsServer::validate_category(&name);
        assert!(result.is_err(), "Should reject character: {}", ch);
        assert!(result.unwrap_err().contains("invalid character"));
    }
}

/// Test: validate_chapter with complex valid markdown heading
/// Precondition: System has complex but valid chapter name
/// Action: Call validate_chapter with complex name
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.3
#[test]
fn test_validate_chapter_complex_valid() {
    let result = RequirementsServer::validate_chapter("Tool: reqlix_get_instructions");
    assert!(result.is_ok());
}

/// Test: validate_category with edge case - single character
/// Precondition: System has single character category name
/// Action: Call validate_category with "a"
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_single_char() {
    let result = RequirementsServer::validate_category("a");
    assert!(result.is_ok());
}

/// Test: validate_chapter with edge case - single character
/// Precondition: System has single character chapter name
/// Action: Call validate_chapter with "A"
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.3
#[test]
fn test_validate_chapter_single_char() {
    let result = RequirementsServer::validate_chapter("A");
    assert!(result.is_ok());
}

