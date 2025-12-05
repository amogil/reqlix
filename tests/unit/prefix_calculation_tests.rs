// Unit tests for prefix calculation functions
// Covers Requirement: G.R.4

use reqlix::RequirementsServer;

/// Test: calculate_unique_prefix with single name
/// Precondition: System has a single name in the list
/// Action: Call calculate_unique_prefix with "general" and list containing only "general"
/// Result: Function returns "G"
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_single_name() {
    let names = vec!["general".to_string()];
    let result = RequirementsServer::calculate_unique_prefix("general", &names);
    assert_eq!(result, "G");
}

/// Test: calculate_unique_prefix with unique first letter
/// Precondition: System has multiple names with different first letters
/// Action: Call calculate_unique_prefix with "general" and list ["general", "testing"]
/// Result: Function returns "G"
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_unique_first_letter() {
    let names = vec!["general".to_string(), "testing".to_string()];
    let result = RequirementsServer::calculate_unique_prefix("general", &names);
    assert_eq!(result, "G");
}

/// Test: calculate_unique_prefix with conflicting first letter
/// Precondition: System has multiple names with same first letter
/// Action: Call calculate_unique_prefix with "general" and list ["general", "guidelines"]
/// Result: Function returns longer prefix to make it unique (e.g., "GE" or "GEN")
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_conflicting_first_letter() {
    let names = vec!["general".to_string(), "guidelines".to_string()];
    let result = RequirementsServer::calculate_unique_prefix("general", &names);
    // Should return at least 2 characters since both start with "G"
    assert!(result.len() >= 2);
    assert!(result.starts_with("G"));
}

/// Test: calculate_unique_prefix with reqlix_ prefix
/// Precondition: System has a name with reqlix_ prefix
/// Action: Call calculate_unique_prefix with "reqlix_get_instructions" and list containing it
/// Result: Function uses full name for prefix calculation (reqlix_ is not special)
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_reqlix_prefix() {
    let names = vec!["reqlix_get_instructions".to_string()];
    let result = RequirementsServer::calculate_unique_prefix("reqlix_get_instructions", &names);
    // Should use full name, so prefix should start with "R"
    assert!(result.starts_with("R"));
}

/// Test: calculate_unique_prefix with multiple reqlix_ names
/// Precondition: System has multiple names with reqlix_ prefix
/// Action: Call calculate_unique_prefix with "reqlix_get_categories" and list ["reqlix_get_instructions", "reqlix_get_categories"]
/// Result: Function returns unique prefix based on full name (reqlix_ is not special)
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_multiple_reqlix() {
    let names = vec![
        "reqlix_get_instructions".to_string(),
        "reqlix_get_categories".to_string(),
    ];
    let result = RequirementsServer::calculate_unique_prefix("reqlix_get_categories", &names);
    // Should use full name, prefix should start with "R"
    assert!(result.starts_with("R"));
    // Since both start with "reqlix_get_", should need more characters to distinguish
    // Verify it's unique - calculate prefix for the other name and check they differ
    let other_result = RequirementsServer::calculate_unique_prefix("reqlix_get_instructions", &names);
    assert_ne!(result, other_result);
    // Both should start with "R" since both names start with "r"
    assert!(other_result.starts_with("R"));
}

/// Test: calculate_unique_prefix with empty list
/// Precondition: System has empty list of names
/// Action: Call calculate_unique_prefix with "general" and empty list
/// Result: Function returns "G" (first letter)
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_empty_list() {
    let names = vec![];
    let result = RequirementsServer::calculate_unique_prefix("general", &names);
    assert_eq!(result, "G");
}

/// Test: calculate_unique_prefix with very long name
/// Precondition: System has a very long name
/// Action: Call calculate_unique_prefix with long name
/// Result: Function returns prefix (may be full name if needed)
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_long_name() {
    let long_name = "a".repeat(100);
    let names = vec![long_name.clone()];
    let result = RequirementsServer::calculate_unique_prefix(&long_name, &names);
    assert_eq!(result, "A");
}

/// Test: calculate_unique_prefix case insensitive uniqueness
/// Precondition: System has names that differ only by case
/// Action: Call calculate_unique_prefix with "General" and list ["general", "General"]
/// Result: Function treats them as conflicting and returns longer prefix
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_case_insensitive() {
    let names = vec!["general".to_string(), "General".to_string()];
    let result = RequirementsServer::calculate_unique_prefix("general", &names);
    // Should return longer prefix since uppercase comparison makes them conflict
    assert!(result.len() >= 2);
}

/// Test: calculate_unique_prefix with special characters
/// Precondition: System has name with special characters
/// Action: Call calculate_unique_prefix with name containing special chars
/// Result: Function handles special characters correctly
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_special_chars() {
    let names = vec!["test-category".to_string()];
    let result = RequirementsServer::calculate_unique_prefix("test-category", &names);
    assert_eq!(result, "T");
}

/// Test: calculate_unique_prefix with unicode characters
/// Precondition: System has name with unicode characters
/// Action: Call calculate_unique_prefix with name containing unicode
/// Result: Function handles unicode characters correctly
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_unicode() {
    let names = vec!["тест".to_string()];
    let result = RequirementsServer::calculate_unique_prefix("тест", &names);
    // Should return first character (uppercased if applicable)
    assert!(!result.is_empty());
}

/// Test: calculate_unique_prefix with numbers in name
/// Precondition: System has name with numbers
/// Action: Call calculate_unique_prefix with name containing numbers
/// Result: Function handles numbers correctly
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_with_numbers() {
    let names = vec!["test123".to_string()];
    let result = RequirementsServer::calculate_unique_prefix("test123", &names);
    assert_eq!(result, "T");
}
