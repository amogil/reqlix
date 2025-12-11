// Tests for Tool: reqlix_search_requirements (G.TOOLREQLIXS.*)
// Covers Requirements: G.TOOLREQLIXS.1, G.TOOLREQLIXS.2, G.TOOLREQLIXS.3, G.TOOLREQLIXS.4, G.TOOLREQLIXS.5, G.TOOLREQLIXS.6

use reqlix::{KeywordsParam, RequirementsServer, SearchRequirementsParams};
use serde_json::Value;
use tempfile::TempDir;

use super::common::{
    create_agents_file_in_req_dir, create_category_file_in_req_dir, create_requirements_dir,
    parse_response,
};

// =============================================================================
// Tests for validate_keywords (G.TOOLREQLIXS.5, G.TOOLREQLIXS.6)
// =============================================================================

/// Test: validate_keywords accepts single keyword
/// Precondition: System has KeywordsParam::Single with valid keyword
/// Action: Call validate_keywords with Single("auth")
/// Result: Function returns Ok with vector containing one keyword "auth"
/// Covers Requirement: G.TOOLREQLIXS.2, G.TOOLREQLIXS.5
#[test]
fn test_validate_keywords_single() {
    let keywords = KeywordsParam::Single("auth".to_string());
    let result = RequirementsServer::validate_keywords(&keywords);
    assert!(result.is_ok());
    let filtered = result.unwrap();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0], "auth");
}

/// Test: validate_keywords accepts array of keywords
/// Precondition: System has KeywordsParam::Batch with multiple valid keywords
/// Action: Call validate_keywords with Batch containing 3 keywords
/// Result: Function returns Ok with vector containing all 3 keywords
/// Covers Requirement: G.TOOLREQLIXS.2, G.TOOLREQLIXS.5
#[test]
fn test_validate_keywords_batch() {
    let keywords = KeywordsParam::Batch(vec![
        "auth".to_string(),
        "user".to_string(),
        "login".to_string(),
    ]);
    let result = RequirementsServer::validate_keywords(&keywords);
    assert!(result.is_ok());
    let filtered = result.unwrap();
    assert_eq!(filtered.len(), 3);
}

/// Test: validate_keywords filters out empty strings
/// Precondition: System has KeywordsParam::Batch with some empty strings
/// Action: Call validate_keywords with Batch containing empty strings mixed with valid keywords
/// Result: Function returns Ok with vector containing only non-empty keywords
/// Covers Requirement: G.TOOLREQLIXS.5
#[test]
fn test_validate_keywords_filters_empty_strings() {
    let keywords = KeywordsParam::Batch(vec![
        "auth".to_string(),
        "".to_string(),
        "user".to_string(),
        "".to_string(),
    ]);
    let result = RequirementsServer::validate_keywords(&keywords);
    assert!(result.is_ok());
    let filtered = result.unwrap();
    assert_eq!(filtered.len(), 2);
    assert_eq!(filtered[0], "auth");
    assert_eq!(filtered[1], "user");
}

/// Test: validate_keywords returns empty vec for empty string
/// Precondition: System has KeywordsParam::Single with empty string
/// Action: Call validate_keywords with Single("")
/// Result: Function returns Ok with empty vector
/// Covers Requirement: G.TOOLREQLIXS.5
#[test]
fn test_validate_keywords_empty_string() {
    let keywords = KeywordsParam::Single("".to_string());
    let result = RequirementsServer::validate_keywords(&keywords);
    assert!(result.is_ok());
    let filtered = result.unwrap();
    assert!(filtered.is_empty());
}

/// Test: validate_keywords returns empty vec for empty array
/// Precondition: System has KeywordsParam::Batch with empty vector
/// Action: Call validate_keywords with Batch([])
/// Result: Function returns Ok with empty vector
/// Covers Requirement: G.TOOLREQLIXS.5, G.P.4
#[test]
fn test_validate_keywords_empty_array() {
    let keywords = KeywordsParam::Batch(vec![]);
    let result = RequirementsServer::validate_keywords(&keywords);
    assert!(result.is_ok());
    let filtered = result.unwrap();
    assert!(filtered.is_empty());
}

/// Test: validate_keywords rejects more than 100 keywords
/// Precondition: System has KeywordsParam::Batch with 101 keywords
/// Action: Call validate_keywords with Batch containing 101 keywords
/// Result: Function returns error about exceeding limit of 100
/// Covers Requirement: G.TOOLREQLIXS.5
#[test]
fn test_validate_keywords_exceeds_limit() {
    let keywords = KeywordsParam::Batch((0..101).map(|i| format!("keyword{}", i)).collect());
    let result = RequirementsServer::validate_keywords(&keywords);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("Keywords count exceeds maximum limit of 100"));
}

/// Test: validate_keywords accepts exactly 100 keywords
/// Precondition: System has KeywordsParam::Batch with exactly 100 keywords
/// Action: Call validate_keywords with Batch containing 100 keywords
/// Result: Function returns Ok with vector containing all 100 keywords
/// Covers Requirement: G.TOOLREQLIXS.5
#[test]
fn test_validate_keywords_at_limit() {
    let keywords = KeywordsParam::Batch((0..100).map(|i| format!("keyword{}", i)).collect());
    let result = RequirementsServer::validate_keywords(&keywords);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 100);
}

/// Test: validate_keywords rejects keyword exceeding 200 characters
/// Precondition: System has KeywordsParam::Single with keyword longer than 200 characters
/// Action: Call validate_keywords with Single containing 201-character keyword
/// Result: Function returns error about exceeding maximum length of 200 characters
/// Covers Requirement: G.TOOLREQLIXS.5, G.P.1
#[test]
fn test_validate_keywords_exceeds_length() {
    let long_keyword = "a".repeat(201);
    let keywords = KeywordsParam::Single(long_keyword);
    let result = RequirementsServer::validate_keywords(&keywords);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("Keyword exceeds maximum length of 200 characters"));
}

/// Test: validate_keywords accepts keyword of exactly 200 characters
/// Precondition: System has KeywordsParam::Single with keyword exactly 200 characters
/// Action: Call validate_keywords with Single containing 200-character keyword
/// Result: Function returns Ok with vector containing the keyword
/// Covers Requirement: G.TOOLREQLIXS.5, G.P.1
#[test]
fn test_validate_keywords_at_length_limit() {
    let keyword = "a".repeat(200);
    let keywords = KeywordsParam::Single(keyword.clone());
    let result = RequirementsServer::validate_keywords(&keywords);
    assert!(result.is_ok());
    assert_eq!(result.unwrap()[0], keyword);
}

// =============================================================================
// Tests for handle_search_requirements (G.TOOLREQLIXS.3, G.TOOLREQLIXS.4)
// =============================================================================

/// Test: search finds requirement by title keyword
/// Precondition: System has requirements directory with category file containing requirement with keyword in title
/// Action: Call handle_search_requirements with keyword matching requirement title
/// Result: Function returns success with one result matching the requirement
/// Covers Requirement: G.TOOLREQLIXS.3, G.TOOLREQLIXS.4
#[test]
fn test_search_finds_by_title() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Security\n\n## G.S.1: User authentication\n\nUsers must authenticate.\n",
    );

    let params = SearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "test".to_string(),
        keywords: KeywordsParam::Single("authentication".to_string()),
    };

    let result = RequirementsServer::handle_search_requirements(params);
    let json: Value = parse_response(&result);

    assert!(json["success"].as_bool().unwrap());
    assert_eq!(json["data"]["results"].as_array().unwrap().len(), 1);
    assert_eq!(json["data"]["results"][0]["index"], "G.S.1");
}

/// Test: search finds requirement by text keyword
/// Precondition: System has requirements directory with category file containing requirement with keyword in text
/// Action: Call handle_search_requirements with keyword matching requirement text
/// Result: Function returns success with one result matching the requirement
/// Covers Requirement: G.TOOLREQLIXS.3, G.TOOLREQLIXS.4
#[test]
fn test_search_finds_by_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Security\n\n## G.S.1: Login\n\nUsers must provide valid credentials.\n",
    );

    let params = SearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "test".to_string(),
        keywords: KeywordsParam::Single("credentials".to_string()),
    };

    let result = RequirementsServer::handle_search_requirements(params);
    let json: Value = parse_response(&result);

    assert!(json["success"].as_bool().unwrap());
    assert_eq!(json["data"]["results"].as_array().unwrap().len(), 1);
    assert_eq!(json["data"]["results"][0]["index"], "G.S.1");
}

/// Test: search is case-insensitive
/// Precondition: System has requirements directory with category file containing requirement
/// Action: Call handle_search_requirements with keyword in different case than requirement text/title
/// Result: Function returns success with matching result (case-insensitive match)
/// Covers Requirement: G.TOOLREQLIXS.3 step 5
#[test]
fn test_search_case_insensitive() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Security\n\n## G.S.1: Authentication\n\nMust use HTTPS protocol.\n",
    );

    let params = SearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "test".to_string(),
        keywords: KeywordsParam::Single("AUTHENTICATION".to_string()),
    };

    let result = RequirementsServer::handle_search_requirements(params);
    let json: Value = parse_response(&result);

    assert!(json["success"].as_bool().unwrap());
    assert_eq!(json["data"]["results"].as_array().unwrap().len(), 1);

    // Also test lowercase search for uppercase content
    let params2 = SearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "test".to_string(),
        keywords: KeywordsParam::Single("https".to_string()),
    };

    let result2 = RequirementsServer::handle_search_requirements(params2);
    let json2: Value = parse_response(&result2);

    assert!(json2["success"].as_bool().unwrap());
    assert_eq!(json2["data"]["results"].as_array().unwrap().len(), 1);
}

/// Test: search with multiple keywords finds requirements matching any
/// Precondition: System has requirements directory with multiple requirements
/// Action: Call handle_search_requirements with Batch containing multiple keywords
/// Result: Function returns success with results matching any of the keywords (OR logic)
/// Covers Requirement: G.TOOLREQLIXS.3 step 6
#[test]
fn test_search_multiple_keywords_or() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Features\n\n## G.F.1: Login feature\n\nUser login.\n\n## G.F.2: Dashboard\n\nMain dashboard view.\n\n## G.F.3: Settings\n\nUser settings page.\n",
    );

    let params = SearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "test".to_string(),
        keywords: KeywordsParam::Batch(vec!["login".to_string(), "dashboard".to_string()]),
    };

    let result = RequirementsServer::handle_search_requirements(params);
    let json: Value = parse_response(&result);

    assert!(json["success"].as_bool().unwrap());
    assert_eq!(json["data"]["results"].as_array().unwrap().len(), 2);
}

/// Test: search returns empty results when no matches
/// Precondition: System has requirements directory with requirements that don't match keyword
/// Action: Call handle_search_requirements with keyword that doesn't match any requirement
/// Result: Function returns success with empty results array
/// Covers Requirement: G.TOOLREQLIXS.4
#[test]
fn test_search_no_matches() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Features\n\n## G.F.1: Login\n\nUser login feature.\n",
    );

    let params = SearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "test".to_string(),
        keywords: KeywordsParam::Single("nonexistent".to_string()),
    };

    let result = RequirementsServer::handle_search_requirements(params);
    let json: Value = parse_response(&result);

    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"]["results"].as_array().unwrap().is_empty());
    assert_eq!(json["data"]["keywords"].as_array().unwrap().len(), 1);
}

/// Test: search with empty keywords returns empty results (G.P.4)
/// Precondition: System has requirements directory with requirements
/// Action: Call handle_search_requirements with Batch([])
/// Result: Function returns success with empty results array
/// Covers Requirement: G.TOOLREQLIXS.5, G.P.4
#[test]
fn test_search_empty_keywords() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Features\n\n## G.F.1: Login\n\nUser login feature.\n",
    );

    let params = SearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "test".to_string(),
        keywords: KeywordsParam::Batch(vec![]),
    };

    let result = RequirementsServer::handle_search_requirements(params);
    let json: Value = parse_response(&result);

    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"]["results"].as_array().unwrap().is_empty());
    assert!(json["data"]["keywords"].as_array().unwrap().is_empty());
}

/// Test: search across multiple categories
/// Precondition: System has requirements directory with multiple category files
/// Action: Call handle_search_requirements with keyword matching requirements in different categories
/// Result: Function returns success with results from all matching categories
/// Covers Requirement: G.TOOLREQLIXS.3 step 1
#[test]
fn test_search_across_categories() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Features\n\n## G.F.1: Security login\n\nUser login with security.\n",
    );
    create_category_file_in_req_dir(
        &req_dir,
        "testing",
        "# Tests\n\n## T.T.1: Security test\n\nTest security features.\n",
    );

    let params = SearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "test".to_string(),
        keywords: KeywordsParam::Single("security".to_string()),
    };

    let result = RequirementsServer::handle_search_requirements(params);
    let json: Value = parse_response(&result);

    assert!(json["success"].as_bool().unwrap());
    assert_eq!(json["data"]["results"].as_array().unwrap().len(), 2);
}

/// Test: search returns correct response structure
/// Precondition: System has requirements directory with matching requirement
/// Action: Call handle_search_requirements with matching keyword
/// Result: Function returns success with correct JSON structure (success, data.keywords, data.results with index, title, text, category, chapter)
/// Covers Requirement: G.TOOLREQLIXS.4
#[test]
fn test_search_response_structure() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test requirement\n\nRequirement body text.\n",
    );

    let params = SearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "test".to_string(),
        keywords: KeywordsParam::Single("test".to_string()),
    };

    let result = RequirementsServer::handle_search_requirements(params);
    let json: Value = parse_response(&result);

    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"]["keywords"].is_array());
    assert!(json["data"]["results"].is_array());

    let req = &json["data"]["results"][0];
    assert!(req["index"].is_string());
    assert!(req["title"].is_string());
    assert!(req["text"].is_string());
    assert!(req["category"].is_string());
    assert!(req["chapter"].is_string());
}

/// Test: search validates project_root parameter
/// Precondition: System has invalid project_root (empty string)
/// Action: Call handle_search_requirements with empty project_root
/// Result: Function returns error about invalid project_root
/// Covers Requirement: G.TOOLREQLIXS.6
#[test]
fn test_search_validates_project_root() {
    let params = SearchRequirementsParams {
        project_root: "".to_string(),
        operation_description: "test".to_string(),
        keywords: KeywordsParam::Single("test".to_string()),
    };

    let result = RequirementsServer::handle_search_requirements(params);
    let json: Value = parse_response(&result);

    assert!(!json["success"].as_bool().unwrap());
    assert!(json["error"].as_str().unwrap().contains("project_root"));
}

/// Test: search validates operation_description parameter
/// Precondition: System has invalid operation_description (empty string)
/// Action: Call handle_search_requirements with empty operation_description
/// Result: Function returns error about invalid operation_description
/// Covers Requirement: G.TOOLREQLIXS.6
#[test]
fn test_search_validates_operation_description() {
    let temp_dir = TempDir::new().unwrap();

    let params = SearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "".to_string(),
        keywords: KeywordsParam::Single("test".to_string()),
    };

    let result = RequirementsServer::handle_search_requirements(params);
    let json: Value = parse_response(&result);

    assert!(!json["success"].as_bool().unwrap());
    assert!(json["error"]
        .as_str()
        .unwrap()
        .contains("operation_description"));
}

/// Test: search with all empty strings in array returns empty results
/// Precondition: System has requirements directory with requirements
/// Action: Call handle_search_requirements with Batch containing only empty strings
/// Result: Function returns success with empty results (empty strings filtered out, leaving no keywords)
/// Covers Requirement: G.TOOLREQLIXS.5
#[test]
fn test_search_all_empty_strings_filtered() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Features\n\n## G.F.1: Login\n\nUser login feature.\n",
    );

    let params = SearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "test".to_string(),
        keywords: KeywordsParam::Batch(vec!["".to_string(), "".to_string(), "".to_string()]),
    };

    let result = RequirementsServer::handle_search_requirements(params);
    let json: Value = parse_response(&result);

    // After filtering empty strings, keywords array is empty -> return empty results
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"]["results"].as_array().unwrap().is_empty());
    assert!(json["data"]["keywords"].as_array().unwrap().is_empty());
}
