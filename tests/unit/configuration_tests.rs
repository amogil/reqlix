// Tests for Configuration (G.C.*)
// Covers Requirements: G.C.1, G.C.2, G.C.7

use reqlix::RequirementsServer;
use std::path::PathBuf;
use tempfile::TempDir;

// =============================================================================
// Tests for G.C.1: Requirements directory location
// =============================================================================

// Note: G.C.1 tests are covered in file_system_tests.rs (get_search_paths, get_create_path)
// These tests verify that requirements directory is correctly located

// =============================================================================
// Tests for G.C.2: Directory creation
// =============================================================================

// Note: G.C.2 tests are covered in requirements_storage_format_tests.rs (write_file_utf8_creates_dirs)
// and name_and_file_validation_tests.rs

// =============================================================================
// Tests for G.C.7: Category management
// =============================================================================

/// Test: list_categories with single category
/// Precondition: System has directory with single category file
/// Action: Call list_categories with directory path
/// Result: Function returns Ok with vector containing one category name
/// Covers Requirement: G.C.7
#[test]
fn test_list_categories_single() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("general.md");
    std::fs::write(&file_path, "").unwrap();

    let result = RequirementsServer::list_categories(&temp_dir.path().to_path_buf());
    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 1);
    assert_eq!(categories[0], "general");
}

/// Test: list_categories excluding AGENTS.md
/// Precondition: System has directory with AGENTS.md and category file
/// Action: Call list_categories with directory path
/// Result: Function returns Ok with vector excluding AGENTS (only category files included)
/// Covers Requirement: G.C.7
#[test]
fn test_list_categories_exclude_agents() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::write(temp_dir.path().join("AGENTS.md"), "").unwrap();
    std::fs::write(temp_dir.path().join("general.md"), "").unwrap();

    let result = RequirementsServer::list_categories(&temp_dir.path().to_path_buf());
    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 1);
    assert!(!categories.contains(&"AGENTS".to_string()));
    assert_eq!(categories[0], "general");
}

/// Test: list_categories with multiple categories
/// Precondition: System has directory with multiple category files
/// Action: Call list_categories with directory path
/// Result: Function returns Ok with sorted vector containing all category names
/// Covers Requirement: G.C.7
#[test]
fn test_list_categories_multiple() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::write(temp_dir.path().join("testing.md"), "").unwrap();
    std::fs::write(temp_dir.path().join("general.md"), "").unwrap();
    std::fs::write(temp_dir.path().join("deployment.md"), "").unwrap();

    let result = RequirementsServer::list_categories(&temp_dir.path().to_path_buf());
    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 3);
    // Should be sorted
    assert_eq!(categories[0], "deployment");
    assert_eq!(categories[1], "general");
    assert_eq!(categories[2], "testing");
}

/// Test: list_categories with empty directory
/// Precondition: System has empty directory
/// Action: Call list_categories with empty directory path
/// Result: Function returns Ok with empty vector
/// Covers Requirement: G.C.7
#[test]
fn test_list_categories_empty() {
    let temp_dir = TempDir::new().unwrap();

    let result = RequirementsServer::list_categories(&temp_dir.path().to_path_buf());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Vec::<String>::new());
}

/// Test: list_categories ignoring non-md files
/// Precondition: System has directory with .md and non-.md files
/// Action: Call list_categories with directory path
/// Result: Function returns Ok with vector containing only .md files (non-.md files ignored)
/// Covers Requirement: G.C.7
#[test]
fn test_list_categories_ignore_non_md() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::write(temp_dir.path().join("general.md"), "").unwrap();
    std::fs::write(temp_dir.path().join("readme.txt"), "").unwrap();

    let result = RequirementsServer::list_categories(&temp_dir.path().to_path_buf());
    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 1);
    assert_eq!(categories[0], "general");
}

/// Test: find_category_by_prefix with single category
/// Precondition: System has directory with single category file
/// Action: Call find_category_by_prefix with directory path and prefix
/// Result: Function returns Ok with matching category name
/// Covers Requirement: G.C.7
#[test]
fn test_find_category_by_prefix_single() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::write(temp_dir.path().join("general.md"), "").unwrap();

    let result = RequirementsServer::find_category_by_prefix(&temp_dir.path().to_path_buf(), "G");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "general");
}

/// Test: find_category_by_prefix with multiple categories
/// Precondition: System has directory with multiple category files
/// Action: Call find_category_by_prefix with directory path and prefix matching one category
/// Result: Function returns Ok with matching category name
/// Covers Requirement: G.C.7
#[test]
fn test_find_category_by_prefix_multiple() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::write(temp_dir.path().join("general.md"), "").unwrap();
    std::fs::write(temp_dir.path().join("testing.md"), "").unwrap();

    let result = RequirementsServer::find_category_by_prefix(&temp_dir.path().to_path_buf(), "G");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "general");
}

/// Test: find_category_by_prefix with non-existent prefix
/// Precondition: System has directory with category files but no matching prefix
/// Action: Call find_category_by_prefix with directory path and non-matching prefix
/// Result: Function returns error about category not found
/// Covers Requirement: G.C.7
#[test]
fn test_find_category_by_prefix_not_found() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::write(temp_dir.path().join("general.md"), "").unwrap();

    let result = RequirementsServer::find_category_by_prefix(&temp_dir.path().to_path_buf(), "X");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

/// Test: find_category_by_prefix with conflicting prefixes
/// Precondition: System has directory with multiple categories starting with same letter
/// Action: Call find_category_by_prefix with longer prefixes to disambiguate
/// Result: Function returns Ok for at least one matching prefix (longer prefix resolves conflict)
/// Covers Requirement: G.C.7
#[test]
fn test_find_category_by_prefix_conflicting() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::write(temp_dir.path().join("general.md"), "").unwrap();
    std::fs::write(temp_dir.path().join("guidelines.md"), "").unwrap();

    // Both start with "G", so need longer prefix
    let result_ge =
        RequirementsServer::find_category_by_prefix(&temp_dir.path().to_path_buf(), "GE");
    let result_gu =
        RequirementsServer::find_category_by_prefix(&temp_dir.path().to_path_buf(), "GU");

    // At least one should succeed
    assert!(result_ge.is_ok() || result_gu.is_ok());
}

/// Test: find_category_by_prefix with empty directory
/// Precondition: System has empty directory
/// Action: Call find_category_by_prefix with empty directory path and prefix
/// Result: Function returns error about category not found
/// Covers Requirement: G.C.7
#[test]
fn test_find_category_by_prefix_empty_dir() {
    let temp_dir = TempDir::new().unwrap();

    let result = RequirementsServer::find_category_by_prefix(&temp_dir.path().to_path_buf(), "G");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}
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
    let other_result =
        RequirementsServer::calculate_unique_prefix("reqlix_get_instructions", &names);
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
/// Result: Function ignores special characters and uses only letters
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_special_chars() {
    let names = vec!["test-category".to_string()];
    let result = RequirementsServer::calculate_unique_prefix("test-category", &names);
    // Should return "T" (first letter, ignoring hyphen)
    assert_eq!(result, "T");
}

/// Test: calculate_unique_prefix with unicode characters
/// Precondition: System has name with unicode characters (no ASCII letters)
/// Action: Call calculate_unique_prefix with name containing only unicode
/// Result: Function returns empty string (only ASCII letters are considered)
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_unicode() {
    let names = vec!["тест".to_string()];
    let result = RequirementsServer::calculate_unique_prefix("тест", &names);
    // Should return empty string since there are no ASCII letters
    assert_eq!(result, "");
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

/// Test: calculate_chapter_prefix with single chapter
/// Precondition: System has a single chapter in the list
/// Action: Call calculate_chapter_prefix with "General Requirements" and list containing only "General Requirements"
/// Result: Function returns "G"
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_chapter_prefix_single_chapter() {
    let chapters = vec!["General Requirements".to_string()];
    let result = RequirementsServer::calculate_chapter_prefix("General Requirements", &chapters);
    assert_eq!(result, "G");
}

/// Test: calculate_chapter_prefix with unique first letter
/// Precondition: System has multiple chapters with different first letters
/// Action: Call calculate_chapter_prefix with "General Requirements" and list ["General Requirements", "Testing"]
/// Result: Function returns "G"
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_chapter_prefix_unique_first_letter() {
    let chapters = vec!["General Requirements".to_string(), "Testing".to_string()];
    let result = RequirementsServer::calculate_chapter_prefix("General Requirements", &chapters);
    assert_eq!(result, "G");
}

/// Test: calculate_chapter_prefix with conflicting first letter
/// Precondition: System has multiple chapters with same first letter
/// Action: Call calculate_chapter_prefix with "General Requirements" and list ["General Requirements", "Guidelines"]
/// Result: Function returns longer prefix to make it unique (e.g., "GE" or "GEN")
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_chapter_prefix_conflicting_first_letter() {
    let chapters = vec!["General Requirements".to_string(), "Guidelines".to_string()];
    let result = RequirementsServer::calculate_chapter_prefix("General Requirements", &chapters);
    // Should return at least 2 characters since both start with "G"
    assert!(result.len() >= 2);
    assert!(result.starts_with("G"));
    // Verify it's unique - calculate prefix for the other chapter and check they differ
    let other_result = RequirementsServer::calculate_chapter_prefix("Guidelines", &chapters);
    assert_ne!(result, other_result);
}

/// Test: calculate_chapter_prefix with reqlix_ prefix chapters
/// Precondition: System has multiple chapters with reqlix_ prefix
/// Action: Call calculate_chapter_prefix with "reqlix_get_instructions" and list ["reqlix_get_instructions", "reqlix_get_categories"]
/// Result: Function returns unique prefix based on full name
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_chapter_prefix_multiple_reqlix() {
    let chapters = vec![
        "reqlix_get_instructions".to_string(),
        "reqlix_get_categories".to_string(),
    ];
    let result = RequirementsServer::calculate_chapter_prefix("reqlix_get_instructions", &chapters);
    // Should use full name, prefix should start with "R"
    assert!(result.starts_with("R"));
    // Since both start with "reqlix_get_", should need more characters to distinguish
    let other_result =
        RequirementsServer::calculate_chapter_prefix("reqlix_get_categories", &chapters);
    assert_ne!(result, other_result);
    // Both should start with "R" since both names start with "r"
    assert!(other_result.starts_with("R"));
}

/// Test: calculate_chapter_prefix with empty list
/// Precondition: System has empty list of chapters
/// Action: Call calculate_chapter_prefix with "General Requirements" and empty list
/// Result: Function returns "G" (first letter)
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_chapter_prefix_empty_list() {
    let chapters = vec![];
    let result = RequirementsServer::calculate_chapter_prefix("General Requirements", &chapters);
    assert_eq!(result, "G");
}

/// Test: calculate_chapter_prefix case insensitive uniqueness
/// Precondition: System has chapters that differ only by case
/// Action: Call calculate_chapter_prefix with "General" and list ["general", "General"]
/// Result: Function treats them as conflicting and returns longer prefix
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_chapter_prefix_case_insensitive() {
    let chapters = vec!["general".to_string(), "General".to_string()];
    let result = RequirementsServer::calculate_chapter_prefix("general", &chapters);
    // Should return longer prefix since uppercase comparison makes them conflict
    assert!(result.len() >= 2);
    assert!(result.starts_with("G"));
}

/// Test: calculate_chapter_prefix with three conflicting chapters
/// Precondition: System has three chapters starting with same letters
/// Action: Call calculate_chapter_prefix with "Get Instructions" and list ["Get Instructions", "Get Categories", "Get Chapters"]
/// Result: Function returns unique prefix that distinguishes all three
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_chapter_prefix_three_conflicts() {
    let chapters = vec![
        "Get Instructions".to_string(),
        "Get Categories".to_string(),
        "Get Chapters".to_string(),
    ];
    let result1 = RequirementsServer::calculate_chapter_prefix("Get Instructions", &chapters);
    let result2 = RequirementsServer::calculate_chapter_prefix("Get Categories", &chapters);
    let result3 = RequirementsServer::calculate_chapter_prefix("Get Chapters", &chapters);

    // All should start with "G"
    assert!(result1.starts_with("G"));
    assert!(result2.starts_with("G"));
    assert!(result3.starts_with("G"));

    // All should be unique
    assert_ne!(result1, result2);
    assert_ne!(result1, result3);
    assert_ne!(result2, result3);
}

/// Test: calculate_chapter_prefix with very long name
/// Precondition: System has a very long chapter name
/// Action: Call calculate_chapter_prefix with long name
/// Result: Function returns prefix (may be full name if needed for uniqueness)
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_chapter_prefix_long_name() {
    let long_name = "A".repeat(100);
    let chapters = vec![long_name.clone()];
    let result = RequirementsServer::calculate_chapter_prefix(&long_name, &chapters);
    assert_eq!(result, "A");
}

/// Test: calculate_chapter_prefix with special characters
/// Precondition: System has chapter name with special characters
/// Action: Call calculate_chapter_prefix with name containing special chars
/// Result: Function ignores special characters and uses only letters
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_chapter_prefix_special_chars() {
    let chapters = vec!["test-chapter".to_string()];
    let result = RequirementsServer::calculate_chapter_prefix("test-chapter", &chapters);
    // Should return "T" (first letter, ignoring hyphen)
    assert_eq!(result, "T");
}

/// Test: calculate_chapter_prefix returns full name when needed for uniqueness
/// Precondition: System has chapters that require full name to be unique
/// Action: Call calculate_chapter_prefix with "ABC" and list ["ABC", "ABD"]
/// Result: Function returns full name "ABC" when needed
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_chapter_prefix_full_name_when_needed() {
    let chapters = vec!["ABC".to_string(), "ABD".to_string()];
    let result = RequirementsServer::calculate_chapter_prefix("ABC", &chapters);
    // Should return "ABC" since "A" and "AB" conflict, but "ABC" is unique
    assert_eq!(result, "ABC");
    let other_result = RequirementsServer::calculate_chapter_prefix("ABD", &chapters);
    assert_eq!(other_result, "ABD");
    assert_ne!(result, other_result);
}

/// Test: calculate_unique_prefix with spaces in name
/// Precondition: System has name with spaces
/// Action: Call calculate_unique_prefix with "test category"
/// Result: Function ignores spaces and uses only letters, returns "T"
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_with_spaces() {
    let names = vec!["test category".to_string()];
    let result = RequirementsServer::calculate_unique_prefix("test category", &names);
    // Should return "T" (first letter, ignoring space)
    assert_eq!(result, "T");
}

/// Test: calculate_chapter_prefix with spaces and colons
/// Precondition: System has chapter name with spaces and colons
/// Action: Call calculate_chapter_prefix with "Chapter: Subchapter"
/// Result: Function ignores spaces and colons, uses only letters
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_chapter_prefix_with_spaces_and_colons() {
    let chapters = vec!["Chapter: Subchapter".to_string()];
    let result = RequirementsServer::calculate_chapter_prefix("Chapter: Subchapter", &chapters);
    // Should return "C" (first letter, ignoring space and colon)
    assert_eq!(result, "C");
}

/// Test: calculate_chapter_prefix with hyphens
/// Precondition: System has chapter name with hyphens
/// Action: Call calculate_chapter_prefix with "Chapter-Subchapter"
/// Result: Function ignores hyphens, uses only letters
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_chapter_prefix_with_hyphens() {
    let chapters = vec!["Chapter-Subchapter".to_string()];
    let result = RequirementsServer::calculate_chapter_prefix("Chapter-Subchapter", &chapters);
    // Should return "C" (first letter, ignoring hyphen)
    assert_eq!(result, "C");
}

/// Test: calculate_unique_prefix with mixed characters
/// Precondition: System has name with letters, numbers, and special chars
/// Action: Call calculate_unique_prefix with "test123-category_456"
/// Result: Function uses only letters, returns "T"
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_mixed_characters() {
    let names = vec!["test123-category_456".to_string()];
    let result = RequirementsServer::calculate_unique_prefix("test123-category_456", &names);
    // Should return "T" (first letter, ignoring numbers, hyphens, underscores)
    assert_eq!(result, "T");
}

/// Test: calculate_chapter_prefix with conflicting names that differ only by non-letters
/// Precondition: System has chapters that differ only by spaces/colons/hyphens
/// Action: Call calculate_chapter_prefix with "Chapter Name" and list ["Chapter-Name", "Chapter:Name"]
/// Result: Function treats them as having same letters and returns same prefix (full name)
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_chapter_prefix_conflicting_non_letters() {
    let chapters = vec![
        "Chapter Name".to_string(),
        "Chapter-Name".to_string(),
        "Chapter:Name".to_string(),
    ];
    let result1 = RequirementsServer::calculate_chapter_prefix("Chapter Name", &chapters);
    let result2 = RequirementsServer::calculate_chapter_prefix("Chapter-Name", &chapters);
    let result3 = RequirementsServer::calculate_chapter_prefix("Chapter:Name", &chapters);

    // All should start with "C" since they all start with "Chapter"
    assert!(result1.starts_with("C"));
    assert!(result2.starts_with("C"));
    assert!(result3.starts_with("C"));

    // Since they all have the same letters (ChapterName), they should get the same prefix
    // which is the full uppercase version of all letters
    assert_eq!(result1, "CHAPTERNAME");
    assert_eq!(result2, "CHAPTERNAME");
    assert_eq!(result3, "CHAPTERNAME");
}

/// Test: calculate_unique_prefix ignores underscores in category name
/// Precondition: System has category name with underscores
/// Action: Call calculate_unique_prefix with "test_category_name"
/// Result: Function ignores underscores, uses only letters "testcategoryname", returns "T"
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_ignores_underscores() {
    let names = vec!["test_category_name".to_string()];
    let result = RequirementsServer::calculate_unique_prefix("test_category_name", &names);
    // Should return "T" (first letter, ignoring underscores)
    assert_eq!(result, "T");
}

/// Test: calculate_chapter_prefix with multiple spaces between words
/// Precondition: System has chapter name with multiple spaces
/// Action: Call calculate_chapter_prefix with "Chapter    Name"
/// Result: Function ignores all spaces, uses only letters
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_chapter_prefix_multiple_spaces() {
    let chapters = vec!["Chapter    Name".to_string()];
    let result = RequirementsServer::calculate_chapter_prefix("Chapter    Name", &chapters);
    // Should return "C" (first letter, ignoring all spaces)
    assert_eq!(result, "C");
}

/// Test: calculate_unique_prefix with name starting with non-letter
/// Precondition: System has category name starting with number
/// Action: Call calculate_unique_prefix with "123test"
/// Result: Function skips numbers, uses first letter "T"
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_starts_with_number() {
    let names = vec!["123test".to_string()];
    let result = RequirementsServer::calculate_unique_prefix("123test", &names);
    // Should return "T" (first letter, ignoring leading numbers)
    assert_eq!(result, "T");
}

/// Test: calculate_chapter_prefix with complex name containing all allowed non-letter characters
/// Precondition: System has chapter name with spaces, colons, hyphens, and numbers
/// Action: Call calculate_chapter_prefix with "Chapter: Sub-Chapter 123 Name"
/// Result: Function uses only letters "ChapterSubChapterName", returns appropriate prefix
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_chapter_prefix_complex_non_letters() {
    let chapters = vec!["Chapter: Sub-Chapter 123 Name".to_string()];
    let result =
        RequirementsServer::calculate_chapter_prefix("Chapter: Sub-Chapter 123 Name", &chapters);
    // Should return "C" (first letter, ignoring spaces, colons, hyphens, numbers)
    assert_eq!(result, "C");
}

/// Test: calculate_unique_prefix with names that have same letters but different non-letters
/// Precondition: System has categories "test-category" and "test_category"
/// Action: Call calculate_unique_prefix for both
/// Result: Function treats them as having same letters and returns same prefix
/// Covers Requirement: G.R.4
#[test]
fn test_calculate_unique_prefix_same_letters_different_non_letters() {
    let names = vec!["test-category".to_string(), "test_category".to_string()];
    let result1 = RequirementsServer::calculate_unique_prefix("test-category", &names);
    let result2 = RequirementsServer::calculate_unique_prefix("test_category", &names);

    // Both should start with "T"
    assert!(result1.starts_with("T"));
    assert!(result2.starts_with("T"));

    // Since they have the same letters (testcategory), they should get the same prefix
    // which is the full uppercase version of all letters
    assert_eq!(result1, "TESTCATEGORY");
    assert_eq!(result2, "TESTCATEGORY");
}

/// Test: get_search_paths with default path
/// Precondition: System has no REQLIX_REQ_REL_PATH environment variable set
/// Action: Call get_search_paths with project_root "/test/project"
/// Result: Function returns paths including default "docs/development/requirements/AGENTS.md"
/// Covers Requirement: G.REQLIX_GET_INST.3
#[test]
fn test_get_search_paths_default() {
    // Clear environment variable if set
    std::env::remove_var("REQLIX_REQ_REL_PATH");

    let result = RequirementsServer::get_search_paths("/test/project");
    assert!(result.len() >= 2);
    assert!(result.contains(&PathBuf::from(
        "/test/project/docs/development/requirements/AGENTS.md"
    )));
    assert!(result.contains(&PathBuf::from("/test/project/docs/dev/req/AGENTS.md")));
}

/// Test: get_search_paths with custom environment variable
/// Precondition: System has REQLIX_REQ_REL_PATH environment variable set
/// Action: Call get_search_paths with project_root "/test/project" and REQLIX_REQ_REL_PATH="custom/path"
/// Result: Function returns paths including custom path first
/// Covers Requirement: G.REQLIX_GET_INST.3
#[test]
fn test_get_search_paths_custom_env() {
    // Save original value if exists
    let original = std::env::var("REQLIX_REQ_REL_PATH").ok();

    std::env::set_var("REQLIX_REQ_REL_PATH", "custom/path");

    let result = RequirementsServer::get_search_paths("/test/project");
    assert!(!result.is_empty());
    assert_eq!(
        result[0],
        PathBuf::from("/test/project/custom/path/AGENTS.md")
    );

    // Restore original value
    match original {
        Some(val) => std::env::set_var("REQLIX_REQ_REL_PATH", val),
        None => std::env::remove_var("REQLIX_REQ_REL_PATH"),
    }
}

/// Test: get_search_paths with empty project root
/// Precondition: System has empty project_root
/// Action: Call get_search_paths with ""
/// Result: Function returns paths with empty root
/// Covers Requirement: G.REQLIX_GET_INST.3
#[test]
fn test_get_search_paths_empty_root() {
    std::env::remove_var("REQLIX_REQ_REL_PATH");

    let result = RequirementsServer::get_search_paths("");
    assert!(!result.is_empty());
    assert!(result.contains(&PathBuf::from("docs/development/requirements/AGENTS.md")));
}

/// Test: get_search_paths with relative project root
/// Precondition: System has relative project_root
/// Action: Call get_search_paths with "project"
/// Result: Function returns paths with relative root
/// Covers Requirement: G.REQLIX_GET_INST.3
#[test]
fn test_get_search_paths_relative_root() {
    std::env::remove_var("REQLIX_REQ_REL_PATH");

    let result = RequirementsServer::get_search_paths("project");
    assert!(!result.is_empty());
    assert!(result.contains(&PathBuf::from(
        "project/docs/development/requirements/AGENTS.md"
    )));
}

/// Test: get_search_paths order
/// Precondition: System has REQLIX_REQ_REL_PATH set
/// Action: Call get_search_paths and check order
/// Result: Custom path comes first, then defaults
/// Covers Requirement: G.REQLIX_GET_INST.3
#[test]
fn test_get_search_paths_order() {
    // Save original value if exists
    let original = std::env::var("REQLIX_REQ_REL_PATH").ok();

    std::env::set_var("REQLIX_REQ_REL_PATH", "custom");

    let result = RequirementsServer::get_search_paths("/test");
    assert!(result.len() >= 3);
    // First should be custom path
    assert!(result[0].to_string_lossy().contains("custom"));

    // Restore original value
    match original {
        Some(val) => std::env::set_var("REQLIX_REQ_REL_PATH", val),
        None => std::env::remove_var("REQLIX_REQ_REL_PATH"),
    }
}

/// Test: get_create_path with default path
/// Precondition: System has no REQLIX_REQ_REL_PATH environment variable set
/// Action: Call get_create_path with project_root "/test/project"
/// Result: Function returns default path "docs/development/requirements/AGENTS.md"
/// Covers Requirement: G.REQLIX_GET_INST.4
#[test]
fn test_get_create_path_default() {
    std::env::remove_var("REQLIX_REQ_REL_PATH");

    let result = RequirementsServer::get_create_path("/test/project");
    assert_eq!(
        result,
        PathBuf::from("/test/project/docs/development/requirements/AGENTS.md")
    );
}

/// Test: get_create_path with custom environment variable
/// Precondition: System has REQLIX_REQ_REL_PATH environment variable set
/// Action: Call get_create_path with project_root "/test/project" and REQLIX_REQ_REL_PATH="custom/path"
/// Result: Function returns custom path
/// Covers Requirement: G.REQLIX_GET_INST.4
#[test]
fn test_get_create_path_custom_env() {
    // Save original value if exists
    let original = std::env::var("REQLIX_REQ_REL_PATH").ok();

    std::env::set_var("REQLIX_REQ_REL_PATH", "custom/path");

    let result = RequirementsServer::get_create_path("/test/project");
    assert_eq!(result, PathBuf::from("/test/project/custom/path/AGENTS.md"));

    // Restore original value
    match original {
        Some(val) => std::env::set_var("REQLIX_REQ_REL_PATH", val),
        None => std::env::remove_var("REQLIX_REQ_REL_PATH"),
    }
}

/// Test: get_create_path with empty project root
/// Precondition: System has empty project_root
/// Action: Call get_create_path with ""
/// Result: Function returns path with empty root
/// Covers Requirement: G.REQLIX_GET_INST.4
#[test]
fn test_get_create_path_empty_root() {
    std::env::remove_var("REQLIX_REQ_REL_PATH");

    let result = RequirementsServer::get_create_path("");
    assert_eq!(
        result,
        PathBuf::from("docs/development/requirements/AGENTS.md")
    );
}

/// Test: get_create_path with relative project root
/// Precondition: System has relative project_root
/// Action: Call get_create_path with "project"
/// Result: Function returns path with relative root
/// Covers Requirement: G.REQLIX_GET_INST.4
#[test]
fn test_get_create_path_relative_root() {
    std::env::remove_var("REQLIX_REQ_REL_PATH");

    let result = RequirementsServer::get_create_path("project");
    assert_eq!(
        result,
        PathBuf::from("project/docs/development/requirements/AGENTS.md")
    );
}

/// Test: get_create_path with nested custom path
/// Precondition: System has nested REQLIX_REQ_REL_PATH
/// Action: Call get_create_path with nested custom path
/// Result: Function returns correct nested path
/// Covers Requirement: G.REQLIX_GET_INST.4
#[test]
fn test_get_create_path_nested_custom() {
    // Save original value if exists
    let original = std::env::var("REQLIX_REQ_REL_PATH").ok();

    std::env::set_var("REQLIX_REQ_REL_PATH", "a/b/c");

    let result = RequirementsServer::get_create_path("/root");
    assert_eq!(result, PathBuf::from("/root/a/b/c/AGENTS.md"));

    // Restore original value
    match original {
        Some(val) => std::env::set_var("REQLIX_REQ_REL_PATH", val),
        None => std::env::remove_var("REQLIX_REQ_REL_PATH"),
    }
}
