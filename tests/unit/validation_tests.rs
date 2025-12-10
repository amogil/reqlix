// Unit tests for parameter validation functions
// Covers Requirement: G.P.1, G.P.2

use reqlix::RequirementsServer;

/// Test: validate_project_root with empty string
/// Precondition: System has no project_root value
/// Action: Call validate_project_root with empty string
/// Result: Function returns error "project_root is required"
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_project_root_empty() {
    let result = RequirementsServer::validate_project_root("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "project_root is required");
}

/// Test: validate_project_root with valid value
/// Precondition: System has a valid project_root value
/// Action: Call validate_project_root with valid path
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_project_root_valid() {
    let result = RequirementsServer::validate_project_root("/valid/path");
    assert!(result.is_ok());
}

/// Test: validate_project_root with value exceeding max length
/// Precondition: System has a project_root value exceeding 1000 characters
/// Action: Call validate_project_root with string longer than 1000 characters
/// Result: Function returns error indicating max length exceeded
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_project_root_too_long() {
    let long_path = "a".repeat(1001);
    let result = RequirementsServer::validate_project_root(&long_path);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("exceeds maximum length"));
}

/// Test: validate_project_root with exactly max length
/// Precondition: System has a project_root value exactly 1000 characters
/// Action: Call validate_project_root with string exactly 1000 characters
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_project_root_max_length() {
    let max_path = "a".repeat(1000);
    let result = RequirementsServer::validate_project_root(&max_path);
    assert!(result.is_ok());
}

/// Test: validate_project_root with one character over max length
/// Precondition: System has a project_root value 1001 characters
/// Action: Call validate_project_root with string 1001 characters
/// Result: Function returns error
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_project_root_one_over_max() {
    let over_max_path = "a".repeat(1001);
    let result = RequirementsServer::validate_project_root(&over_max_path);
    assert!(result.is_err());
}

// Tests for validate_operation_description

/// Test: validate_operation_description with empty string
/// Precondition: System has no operation_description value
/// Action: Call validate_operation_description with empty string
/// Result: Function returns error "operation_description is required"
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_operation_description_empty() {
    let result = RequirementsServer::validate_operation_description("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "operation_description is required");
}

/// Test: validate_operation_description with valid value
/// Precondition: System has a valid operation_description value
/// Action: Call validate_operation_description with valid description
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_operation_description_valid() {
    let result = RequirementsServer::validate_operation_description("Valid operation description");
    assert!(result.is_ok());
}

/// Test: validate_operation_description with value exceeding max length
/// Precondition: System has an operation_description value exceeding 10000 characters
/// Action: Call validate_operation_description with string longer than 10000 characters
/// Result: Function returns error indicating max length exceeded
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_operation_description_too_long() {
    let long_desc = "a".repeat(10001);
    let result = RequirementsServer::validate_operation_description(&long_desc);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("exceeds maximum length"));
}

/// Test: validate_operation_description with exactly max length
/// Precondition: System has an operation_description value exactly 10000 characters
/// Action: Call validate_operation_description with string exactly 10000 characters
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_operation_description_max_length() {
    let max_desc = "a".repeat(10000);
    let result = RequirementsServer::validate_operation_description(&max_desc);
    assert!(result.is_ok());
}

/// Test: validate_operation_description with one character over max length
/// Precondition: System has an operation_description value 10001 characters
/// Action: Call validate_operation_description with string 10001 characters
/// Result: Function returns error
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_operation_description_one_over_max() {
    let over_max_desc = "a".repeat(10001);
    let result = RequirementsServer::validate_operation_description(&over_max_desc);
    assert!(result.is_err());
}

// Tests for validate_category

/// Test: validate_category with empty string
/// Precondition: System has no category value
/// Action: Call validate_category with empty string
/// Result: Function returns error "category is required"
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_category_empty() {
    let result = RequirementsServer::validate_category("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "category is required");
}

/// Test: validate_category with valid value
/// Precondition: System has a valid category value
/// Action: Call validate_category with valid category name
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_category_valid() {
    let result = RequirementsServer::validate_category("general");
    assert!(result.is_ok());
}

/// Test: validate_category with value exceeding max length
/// Precondition: System has a category value exceeding 100 characters
/// Action: Call validate_category with string longer than 100 characters
/// Result: Function returns error indicating max length exceeded
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_category_too_long() {
    let long_category = "a".repeat(101);
    let result = RequirementsServer::validate_category(&long_category);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("exceeds maximum length"));
}

/// Test: validate_category with exactly max length
/// Precondition: System has a category value exactly 100 characters
/// Action: Call validate_category with string exactly 100 characters
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_category_max_length() {
    let max_category = "a".repeat(100);
    let result = RequirementsServer::validate_category(&max_category);
    assert!(result.is_ok());
}

/// Test: validate_category with one character over max length
/// Precondition: System has a category value 101 characters
/// Action: Call validate_category with string 101 characters
/// Result: Function returns error
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_category_one_over_max() {
    let over_max_category = "a".repeat(101);
    let result = RequirementsServer::validate_category(&over_max_category);
    assert!(result.is_err());
}

// Tests for validate_chapter

/// Test: validate_chapter with empty string
/// Precondition: System has no chapter value
/// Action: Call validate_chapter with empty string
/// Result: Function returns error "chapter is required"
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_chapter_empty() {
    let result = RequirementsServer::validate_chapter("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "chapter is required");
}

/// Test: validate_chapter with valid value
/// Precondition: System has a valid chapter value
/// Action: Call validate_chapter with valid chapter name
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_chapter_valid() {
    let result = RequirementsServer::validate_chapter("General Requirements");
    assert!(result.is_ok());
}

/// Test: validate_chapter with value exceeding max length
/// Precondition: System has a chapter value exceeding 100 characters
/// Action: Call validate_chapter with string longer than 100 characters
/// Result: Function returns error indicating max length exceeded
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_chapter_too_long() {
    let long_chapter = "a".repeat(101);
    let result = RequirementsServer::validate_chapter(&long_chapter);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("exceeds maximum length"));
}

/// Test: validate_chapter with exactly max length
/// Precondition: System has a chapter value exactly 100 characters
/// Action: Call validate_chapter with string exactly 100 characters
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_chapter_max_length() {
    let max_chapter = "a".repeat(100);
    let result = RequirementsServer::validate_chapter(&max_chapter);
    assert!(result.is_ok());
}

/// Test: validate_chapter with one character over max length
/// Precondition: System has a chapter value 101 characters
/// Action: Call validate_chapter with string 101 characters
/// Result: Function returns error
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_chapter_one_over_max() {
    let over_max_chapter = "a".repeat(101);
    let result = RequirementsServer::validate_chapter(&over_max_chapter);
    assert!(result.is_err());
}

// Tests for validate_index

/// Test: validate_index with empty string
/// Precondition: System has no index value
/// Action: Call validate_index with empty string
/// Result: Function returns error "index is required"
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_index_empty() {
    let result = RequirementsServer::validate_index("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "index is required");
}

/// Test: validate_index with valid value
/// Precondition: System has a valid index value
/// Action: Call validate_index with valid index
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_index_valid() {
    let result = RequirementsServer::validate_index("G.G.1");
    assert!(result.is_ok());
}

/// Test: validate_index with value exceeding max length
/// Precondition: System has an index value exceeding 100 characters
/// Action: Call validate_index with string longer than 100 characters
/// Result: Function returns error indicating max length exceeded
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_index_too_long() {
    let long_index = "a".repeat(101);
    let result = RequirementsServer::validate_index(&long_index);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("exceeds maximum length"));
}

/// Test: validate_index with exactly max length
/// Precondition: System has an index value exactly 100 characters
/// Action: Call validate_index with string exactly 100 characters
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_index_max_length() {
    let max_index = "a".repeat(100);
    let result = RequirementsServer::validate_index(&max_index);
    assert!(result.is_ok());
}

/// Test: validate_index with one character over max length
/// Precondition: System has an index value 101 characters
/// Action: Call validate_index with string 101 characters
/// Result: Function returns error
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_index_one_over_max() {
    let over_max_index = "a".repeat(101);
    let result = RequirementsServer::validate_index(&over_max_index);
    assert!(result.is_err());
}

/// Test: validate_index with realistic long index format
/// Precondition: System has an index value with realistic format like "G.REQLIX_U.3"
/// Action: Call validate_index with realistic long index (13 characters)
/// Result: Function returns Ok(()) since it's within 100 character limit
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_index_realistic_long_format() {
    let realistic_index = "G.REQLIX_U.3";
    let result = RequirementsServer::validate_index(realistic_index);
    assert!(result.is_ok());
}

/// Test: validate_index with very long but valid index
/// Precondition: System has an index value at 99 characters (just under limit)
/// Action: Call validate_index with string 99 characters
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_index_just_under_max() {
    let just_under_max_index = "a".repeat(99);
    let result = RequirementsServer::validate_index(&just_under_max_index);
    assert!(result.is_ok());
}

/// Test: validate_index with index containing dots and special characters at max length
/// Precondition: System has an index value exactly 100 characters with realistic format
/// Action: Call validate_index with complex index format at max length
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_index_complex_format_at_max() {
    // Create a realistic index format that's exactly 100 characters
    // Format: CATEGORY_PREFIX.CHAPTER_PREFIX.NUMBER
    let category_part = "VERY_LONG_CATEGORY_NAME";
    let chapter_part = "EXTREMELY_LONG_CHAPTER_NAME_FOR_TESTING";
    let number_part = "12345";
    let separator = ".";
    let total_len = category_part.len()
        + separator.len()
        + chapter_part.len()
        + separator.len()
        + number_part.len();
    // Adjust to exactly 100 characters
    let padding_needed = 100 - total_len;
    let complex_index = format!(
        "{}{}{}{}{}{}",
        category_part,
        separator,
        chapter_part,
        separator,
        number_part,
        "x".repeat(padding_needed.max(0))
    );
    assert_eq!(complex_index.len(), 100);
    let result = RequirementsServer::validate_index(&complex_index);
    assert!(result.is_ok());
}

// Tests for validate_text

/// Test: validate_text with empty string
/// Precondition: System has no text value
/// Action: Call validate_text with empty string
/// Result: Function returns error "text is required"
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_text_empty() {
    let result = RequirementsServer::validate_text("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "text is required");
}

/// Test: validate_text with valid value
/// Precondition: System has a valid text value
/// Action: Call validate_text with valid text
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_text_valid() {
    let result = RequirementsServer::validate_text("Valid requirement text");
    assert!(result.is_ok());
}

/// Test: validate_text with value exceeding max length
/// Precondition: System has a text value exceeding 10000 characters
/// Action: Call validate_text with string longer than 10000 characters
/// Result: Function returns error indicating max length exceeded
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_text_too_long() {
    let long_text = "a".repeat(10001);
    let result = RequirementsServer::validate_text(&long_text);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("exceeds maximum length"));
}

/// Test: validate_text with exactly max length
/// Precondition: System has a text value exactly 10000 characters
/// Action: Call validate_text with string exactly 10000 characters
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_text_max_length() {
    let max_text = "a".repeat(10000);
    let result = RequirementsServer::validate_text(&max_text);
    assert!(result.is_ok());
}

/// Test: validate_text with one character over max length
/// Precondition: System has a text value 10001 characters
/// Action: Call validate_text with string 10001 characters
/// Result: Function returns error
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_text_one_over_max() {
    let over_max_text = "a".repeat(10001);
    let result = RequirementsServer::validate_text(&over_max_text);
    assert!(result.is_err());
}

// Tests for validate_title

/// Test: validate_title with empty string when required=true
/// Precondition: System has no title value and title is required
/// Action: Call validate_title with empty string and required=true
/// Result: Function returns error "title is required"
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_title_empty_required() {
    let result = RequirementsServer::validate_title("", true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "title is required");
}

/// Test: validate_title with empty string when required=false
/// Precondition: System has no title value and title is optional
/// Action: Call validate_title with empty string and required=false
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_title_empty_optional() {
    let result = RequirementsServer::validate_title("", false);
    assert!(result.is_ok());
}

/// Test: validate_title with valid value
/// Precondition: System has a valid title value
/// Action: Call validate_title with valid title
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_title_valid() {
    let result = RequirementsServer::validate_title("Valid Title", true);
    assert!(result.is_ok());
}

/// Test: validate_title with value exceeding max length
/// Precondition: System has a title value exceeding 100 characters
/// Action: Call validate_title with string longer than 100 characters
/// Result: Function returns error indicating max length exceeded
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_title_too_long() {
    let long_title = "a".repeat(101);
    let result = RequirementsServer::validate_title(&long_title, true);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("exceeds maximum length"));
}

/// Test: validate_title with exactly max length
/// Precondition: System has a title value exactly 100 characters
/// Action: Call validate_title with string exactly 100 characters
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_title_max_length() {
    let max_title = "a".repeat(100);
    let result = RequirementsServer::validate_title(&max_title, true);
    assert!(result.is_ok());
}

/// Test: validate_title with one character over max length
/// Precondition: System has a title value 101 characters
/// Action: Call validate_title with string 101 characters
/// Result: Function returns error
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_title_one_over_max() {
    let over_max_title = "a".repeat(101);
    let result = RequirementsServer::validate_title(&over_max_title, true);
    assert!(result.is_err());
}

// Edge case tests for validation functions

/// Test: validate_project_root with whitespace-only string
/// Precondition: System has a project_root value containing only whitespace
/// Action: Call validate_project_root with string containing only spaces
/// Result: Function returns Ok(()) (whitespace is not empty)
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_project_root_whitespace() {
    let result = RequirementsServer::validate_project_root("   ");
    assert!(result.is_ok());
}

/// Test: validate_category with single character
/// Precondition: System has a category value with single character
/// Action: Call validate_category with "a"
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_category_single_char() {
    let result = RequirementsServer::validate_category("a");
    assert!(result.is_ok());
}

/// Test: validate_chapter with single character
/// Precondition: System has a chapter value with single character
/// Action: Call validate_chapter with "A"
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_chapter_single_char() {
    let result = RequirementsServer::validate_chapter("A");
    assert!(result.is_ok());
}

/// Test: validate_index with single character parts
/// Precondition: System has an index value with single character parts
/// Action: Call validate_index with "A.B.1"
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_index_single_chars() {
    let result = RequirementsServer::validate_index("A.B.1");
    assert!(result.is_ok());
}

/// Test: validate_text with single character
/// Precondition: System has a text value with single character
/// Action: Call validate_text with "a"
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_text_single_char() {
    let result = RequirementsServer::validate_text("a");
    assert!(result.is_ok());
}

/// Test: validate_title with single character when required
/// Precondition: System has a title value with single character and title is required
/// Action: Call validate_title with "a" and required=true
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_title_single_char_required() {
    let result = RequirementsServer::validate_title("a", true);
    assert!(result.is_ok());
}

/// Test: validate_title with single character when optional
/// Precondition: System has a title value with single character and title is optional
/// Action: Call validate_title with "a" and required=false
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_title_single_char_optional() {
    let result = RequirementsServer::validate_title("a", false);
    assert!(result.is_ok());
}

/// Test: validate_operation_description with newlines
/// Precondition: System has an operation_description value containing newlines
/// Action: Call validate_operation_description with string containing newlines
/// Result: Function returns Ok(()) (newlines are valid characters)
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_operation_description_newlines() {
    let result = RequirementsServer::validate_operation_description("Line 1\nLine 2");
    assert!(result.is_ok());
}

/// Test: validate_text with newlines
/// Precondition: System has a text value containing newlines
/// Action: Call validate_text with string containing newlines
/// Result: Function returns Ok(()) (newlines are valid characters)
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_text_newlines() {
    let result = RequirementsServer::validate_text("Line 1\nLine 2\nLine 3");
    assert!(result.is_ok());
}

/// Test: validate_category with unicode characters
/// Precondition: System has a category value containing unicode characters
/// Action: Call validate_category with string containing unicode
/// Result: Function returns error (only lowercase English letters and underscore allowed)
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_unicode() {
    let result = RequirementsServer::validate_category("тест");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("lowercase English letters"));
}

/// Test: validate_chapter with unicode characters
/// Precondition: System has a chapter value containing unicode characters
/// Action: Call validate_chapter with string containing unicode
/// Result: Function returns error (only A-Z, a-z, spaces, colons, and hyphens allowed)
/// Covers Requirement: G.P.3
#[test]
fn test_validate_chapter_unicode() {
    let result = RequirementsServer::validate_chapter("Глава");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("uppercase and lowercase English letters"));
}

// Tests for category validation: only lowercase English letters and underscore (G.P.3)

/// Test: validate_category with uppercase letters
/// Precondition: System has a category value containing uppercase letters
/// Action: Call validate_category with "General"
/// Result: Function returns error (only lowercase allowed)
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_uppercase() {
    let result = RequirementsServer::validate_category("General");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("lowercase English letters"));
}

/// Test: validate_category with numbers
/// Precondition: System has a category value containing numbers
/// Action: Call validate_category with "test123"
/// Result: Function returns error (only lowercase letters and underscore allowed)
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_with_numbers() {
    let result = RequirementsServer::validate_category("test123");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("lowercase English letters"));
}

/// Test: validate_category with spaces
/// Precondition: System has a category value containing spaces
/// Action: Call validate_category with "test category"
/// Result: Function returns error (spaces not allowed)
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_with_spaces() {
    let result = RequirementsServer::validate_category("test category");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("lowercase English letters"));
}

/// Test: validate_category with valid underscore
/// Precondition: System has a category value with underscore
/// Action: Call validate_category with "test_category"
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_with_underscore() {
    let result = RequirementsServer::validate_category("test_category");
    assert!(result.is_ok());
}

/// Test: validate_category with mixed case
/// Precondition: System has a category value with mixed case
/// Action: Call validate_category with "TestCategory"
/// Result: Function returns error (only lowercase allowed)
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_mixed_case() {
    let result = RequirementsServer::validate_category("TestCategory");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("lowercase English letters"));
}

// Tests for chapter validation: only A-Z, a-z, spaces, colons, and hyphens (G.P.3)

/// Test: validate_chapter with numbers
/// Precondition: System has a chapter value containing numbers
/// Action: Call validate_chapter with "Chapter 123"
/// Result: Function returns error (numbers not allowed)
/// Covers Requirement: G.P.3
#[test]
fn test_validate_chapter_with_numbers() {
    let result = RequirementsServer::validate_chapter("Chapter 123");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("uppercase and lowercase English letters"));
}

/// Test: validate_chapter with underscore
/// Precondition: System has a chapter value containing underscore
/// Action: Call validate_chapter with "Chapter_Name"
/// Result: Function returns Ok (underscore is allowed per G.P.3)
/// Covers Requirement: G.P.3
#[test]
fn test_validate_chapter_with_underscore() {
    let result = RequirementsServer::validate_chapter("Chapter_Name");
    assert!(result.is_ok());
}

/// Test: validate_chapter with valid colon
/// Precondition: System has a chapter value containing colon
/// Action: Call validate_chapter with "Chapter: Subchapter"
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.3
#[test]
fn test_validate_chapter_with_colon() {
    let result = RequirementsServer::validate_chapter("Chapter: Subchapter");
    assert!(result.is_ok());
}

/// Test: validate_chapter with valid hyphen
/// Precondition: System has a chapter value containing hyphen
/// Action: Call validate_chapter with "Chapter-Subchapter"
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.3
#[test]
fn test_validate_chapter_with_hyphen() {
    let result = RequirementsServer::validate_chapter("Chapter-Subchapter");
    assert!(result.is_ok());
}

/// Test: validate_chapter with valid combination of allowed characters
/// Precondition: System has a chapter value with spaces, colons, and hyphens
/// Action: Call validate_chapter with "Chapter: Sub-Chapter Name"
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.3
#[test]
fn test_validate_chapter_with_all_allowed_chars() {
    let result = RequirementsServer::validate_chapter("Chapter: Sub-Chapter Name");
    assert!(result.is_ok());
}
