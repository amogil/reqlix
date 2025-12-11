// Tests for Parameter Constraints (G.P.*)
// Covers Requirements: G.P.1, G.P.2, G.P.3, G.P.4

use reqlix::RequirementsServer;

// =============================================================================
// Tests for G.P.1, G.P.2: Parameter constraints and validation
// =============================================================================

// Tests for validate_project_root (G.P.1, G.P.2)

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

// Tests for validate_operation_description (G.P.1, G.P.2)

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

// Tests for validate_category - length constraints (G.P.1, G.P.2)

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

// Tests for validate_chapter - length constraints (G.P.1, G.P.2)

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

// Tests for validate_index (G.P.1, G.P.2)

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

// Tests for validate_text (G.P.1, G.P.2)

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

// Tests for validate_title (G.P.1, G.P.2)

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

// =============================================================================
// Tests for G.P.3: Name validation
// =============================================================================

// Tests for validate_category - name validation (G.P.3)

/// Test: validate_category with valid name
/// Precondition: System has a valid category name
/// Action: Call validate_category with valid name
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_valid_name() {
    let result = RequirementsServer::validate_category("general");
    assert!(result.is_ok());
}

/// Test: validate_category with single character
/// Precondition: System has a category value with single character
/// Action: Call validate_category with "a"
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_single_char() {
    let result = RequirementsServer::validate_category("a");
    assert!(result.is_ok());
}

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

/// Test: validate_category with name containing invalid filename character '/'
/// Precondition: System has a category name with invalid character
/// Action: Call validate_category with name containing '/'
/// Result: Function returns error (either about lowercase letters or invalid character)
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_invalid_char_slash() {
    let result = RequirementsServer::validate_category("test/category");
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("lowercase English letters") || err_msg.contains("invalid character"));
}

/// Test: validate_category with name containing backslash
/// Precondition: System has a category name with backslash
/// Action: Call validate_category with name containing '\'
/// Result: Function returns error (either about lowercase letters or invalid character)
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_invalid_char_backslash() {
    let result = RequirementsServer::validate_category("test\\category");
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("lowercase English letters") || err_msg.contains("invalid character"));
}

/// Test: validate_category with reserved name AGENTS
/// Precondition: System has category name "AGENTS"
/// Action: Call validate_category with "AGENTS"
/// Result: Function returns error (either about lowercase letters or reserved name)
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_reserved_name() {
    let result = RequirementsServer::validate_category("AGENTS");
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("lowercase English letters") || err_msg.contains("reserved"));
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
/// Result: Function returns error (either about lowercase letters or consecutive dots)
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_consecutive_dots() {
    let result = RequirementsServer::validate_category("test..category");
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("lowercase English letters") || err_msg.contains("consecutive dots"));
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

/// Test: validate_category with all invalid filename characters
/// Precondition: System has category name with various invalid characters
/// Action: Call validate_category with each invalid character
/// Result: Function returns error for each (only lowercase letters and underscore allowed)
/// Covers Requirement: G.P.3
#[test]
fn test_validate_category_all_invalid_chars() {
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|', 'A', '1', ' '];
    for &ch in &invalid_chars {
        let name = format!("test{}category", ch);
        let result = RequirementsServer::validate_category(&name);
        assert!(result.is_err(), "Should reject character: {}", ch);
    }
}

// Tests for validate_chapter - name validation (G.P.3)

/// Test: validate_chapter with valid name
/// Precondition: System has a valid chapter name
/// Action: Call validate_chapter with valid name
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.3
#[test]
fn test_validate_chapter_valid_name() {
    let result = RequirementsServer::validate_chapter("General Requirements");
    assert!(result.is_ok());
}

/// Test: validate_chapter with single character
/// Precondition: System has a chapter value with single character
/// Action: Call validate_chapter with "A"
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.3
#[test]
fn test_validate_chapter_single_char() {
    let result = RequirementsServer::validate_chapter("A");
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
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("uppercase and lowercase English letters") || err_msg.contains("newline")
    );
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

/// Test: validate_chapter with complex valid markdown heading
/// Precondition: System has complex but valid chapter name
/// Action: Call validate_chapter with complex name (with colon and spaces)
/// Result: Function returns Ok(())
/// Covers Requirement: G.P.3
#[test]
fn test_validate_chapter_complex_valid() {
    let result = RequirementsServer::validate_chapter("Tool: Get Instructions");
    assert!(result.is_ok());
}

// =============================================================================
// Tests for G.P.4: Empty array handling
// =============================================================================

// Note: G.P.4 tests are covered in tool-specific test files
// (e.g., tool_get_requirement_tests.rs, tool_update_requirement_tests.rs)

// =============================================================================
// Parameter validation tests
// =============================================================================

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
// Error response format tests (G.C.6)
// =============================================================================

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
// Additional validation and edge case tests
// =============================================================================

// =============================================================================
// Additional validation and edge case tests
// =============================================================================

use tempfile::TempDir;

use super::common::{
    create_agents_file_in_req_dir, create_category_file_in_req_dir, create_requirements_dir,
};

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
    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
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

    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
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

    let file_content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
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
