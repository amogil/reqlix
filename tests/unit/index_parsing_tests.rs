// Unit tests for index parsing functions
// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3

use reqlix::RequirementsServer;

/// Test: parse_index with valid index
/// Precondition: System has a valid requirement index in format "C.C.N"
/// Action: Call parse_index with "G.G.1"
/// Result: Function returns Ok(("G", "G", "1"))
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3
#[test]
fn test_parse_index_valid() {
    let result = RequirementsServer::parse_index("G.G.1");
    assert_eq!(
        result,
        Ok(("G".to_string(), "G".to_string(), "1".to_string()))
    );
}

/// Test: parse_index with multi-character parts
/// Precondition: System has a requirement index with multi-character parts
/// Action: Call parse_index with "GET.GET_C.123"
/// Result: Function returns Ok(("GET", "GET_C", "123"))
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3
#[test]
fn test_parse_index_multi_char() {
    let result = RequirementsServer::parse_index("GET.GET_C.123");
    assert_eq!(
        result,
        Ok(("GET".to_string(), "GET_C".to_string(), "123".to_string()))
    );
}

/// Test: parse_index with invalid format (too few parts)
/// Precondition: System has an index with only 2 parts instead of 3
/// Action: Call parse_index with "G.G"
/// Result: Function returns error indicating invalid format
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3
#[test]
fn test_parse_index_too_few_parts() {
    let result = RequirementsServer::parse_index("G.G");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid index format"));
}

/// Test: parse_index with invalid format (too many parts)
/// Precondition: System has an index with 4 parts instead of 3
/// Action: Call parse_index with "G.G.1.2"
/// Result: Function returns error indicating invalid format
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3
#[test]
fn test_parse_index_too_many_parts() {
    let result = RequirementsServer::parse_index("G.G.1.2");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid index format"));
}

/// Test: parse_index with empty string
/// Precondition: System has an empty index string
/// Action: Call parse_index with ""
/// Result: Function returns error indicating invalid format
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3
#[test]
fn test_parse_index_empty() {
    let result = RequirementsServer::parse_index("");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid index format"));
}

/// Test: parse_index with only dots
/// Precondition: System has an index string with only dots
/// Action: Call parse_index with "..."
/// Result: Function returns error indicating invalid format
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3
#[test]
fn test_parse_index_only_dots() {
    let result = RequirementsServer::parse_index("...");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid index format"));
}

/// Test: parse_index with leading dot
/// Precondition: System has an index string with leading dot
/// Action: Call parse_index with ".G.1"
/// Result: Function returns Ok with empty first part (parsing succeeds, validation should catch this)
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3
#[test]
fn test_parse_index_leading_dot() {
    let result = RequirementsServer::parse_index(".G.1");
    // parse_index only splits by dots, doesn't validate empty parts
    // Empty parts will be caught by validate_index
    assert_eq!(
        result,
        Ok(("".to_string(), "G".to_string(), "1".to_string()))
    );
}

/// Test: parse_index with trailing dot
/// Precondition: System has an index string with trailing dot
/// Action: Call parse_index with "G.G."
/// Result: Function returns Ok with empty third part (parsing succeeds, validation should catch this)
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3
#[test]
fn test_parse_index_trailing_dot() {
    let result = RequirementsServer::parse_index("G.G.");
    // parse_index only splits by dots, doesn't validate empty parts
    // Empty parts will be caught by validate_index
    assert_eq!(
        result,
        Ok(("G".to_string(), "G".to_string(), "".to_string()))
    );
}

/// Test: parse_index with consecutive dots
/// Precondition: System has an index string with consecutive dots
/// Action: Call parse_index with "G..1"
/// Result: Function returns Ok with empty second part (parsing succeeds, validation should catch this)
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3
#[test]
fn test_parse_index_consecutive_dots() {
    let result = RequirementsServer::parse_index("G..1");
    // parse_index only splits by dots, doesn't validate empty parts
    // Empty parts will be caught by validate_index
    assert_eq!(
        result,
        Ok(("G".to_string(), "".to_string(), "1".to_string()))
    );
}

/// Test: parse_index with single character parts
/// Precondition: System has an index with single character parts
/// Action: Call parse_index with "A.B.1"
/// Result: Function returns Ok(("A", "B", "1"))
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3
#[test]
fn test_parse_index_single_chars() {
    let result = RequirementsServer::parse_index("A.B.1");
    assert_eq!(
        result,
        Ok(("A".to_string(), "B".to_string(), "1".to_string()))
    );
}

/// Test: parse_index with large number
/// Precondition: System has an index with large number part
/// Action: Call parse_index with "G.G.999999"
/// Result: Function returns Ok(("G", "G", "999999"))
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3
#[test]
fn test_parse_index_large_number() {
    let result = RequirementsServer::parse_index("G.G.999999");
    assert_eq!(
        result,
        Ok(("G".to_string(), "G".to_string(), "999999".to_string()))
    );
}

/// Test: parse_index with zero as number
/// Precondition: System has an index with zero as number part
/// Action: Call parse_index with "G.G.0"
/// Result: Function returns Ok(("G", "G", "0"))
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3
#[test]
fn test_parse_index_zero_number() {
    let result = RequirementsServer::parse_index("G.G.0");
    assert_eq!(
        result,
        Ok(("G".to_string(), "G".to_string(), "0".to_string()))
    );
}

/// Test: parse_index with underscore in parts
/// Precondition: System has an index with underscores in parts
/// Action: Call parse_index with "GET.GET_C.1"
/// Result: Function returns Ok(("GET", "GET_C", "1"))
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3
#[test]
fn test_parse_index_underscores() {
    let result = RequirementsServer::parse_index("GET.GET_C.1");
    assert_eq!(
        result,
        Ok(("GET".to_string(), "GET_C".to_string(), "1".to_string()))
    );
}

/// Test: parse_index with whitespace (should fail validation before parsing)
/// Precondition: System has an index with whitespace
/// Action: Call parse_index with "G .G.1"
/// Result: Function may return error or parse with whitespace (implementation dependent)
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3
#[test]
fn test_parse_index_with_whitespace() {
    let result = RequirementsServer::parse_index("G .G.1");
    // This might parse or fail depending on implementation
    // The important thing is it doesn't crash
    let _ = result;
}
