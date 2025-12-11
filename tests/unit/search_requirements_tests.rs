// Unit tests for reqlix_search_requirements tool
// Covers Requirements: G.TOOLREQLIXS.1, G.TOOLREQLIXS.2, G.TOOLREQLIXS.3, G.TOOLREQLIXS.4, G.TOOLREQLIXS.5, G.TOOLREQLIXS.6

use reqlix::{KeywordsParam, RequirementsServer, SearchRequirementsParams};
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

// Helper function to create requirements directory structure
fn create_requirements_dir(temp_dir: &TempDir) -> std::path::PathBuf {
    let req_dir = temp_dir.path().join("docs/development/requirements");
    fs::create_dir_all(&req_dir).unwrap();
    req_dir
}

// Helper function to create a category file in requirements directory
fn create_category_file(req_dir: &std::path::Path, category: &str, content: &str) {
    let file_path = req_dir.join(format!("{}.md", category));
    fs::write(&file_path, content).unwrap();
}

// Helper function to create AGENTS.md in requirements directory
fn create_agents_file(req_dir: &std::path::Path, content: &str) {
    let file_path = req_dir.join("AGENTS.md");
    fs::write(&file_path, content).unwrap();
}

// Helper to parse JSON response
fn parse_response(response: &str) -> Value {
    serde_json::from_str(response).unwrap()
}

// =============================================================================
// Tests for validate_keywords (G.TOOLREQLIXS.5, G.TOOLREQLIXS.6)
// =============================================================================

/// Test 1: validate_keywords accepts single keyword
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

/// Test 2: validate_keywords accepts array of keywords
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

/// Test 3: validate_keywords filters out empty strings
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

/// Test 4: validate_keywords returns empty vec for empty string
/// Covers Requirement: G.TOOLREQLIXS.5
#[test]
fn test_validate_keywords_empty_string() {
    let keywords = KeywordsParam::Single("".to_string());
    let result = RequirementsServer::validate_keywords(&keywords);
    assert!(result.is_ok());
    let filtered = result.unwrap();
    assert!(filtered.is_empty());
}

/// Test 5: validate_keywords returns empty vec for empty array
/// Covers Requirement: G.TOOLREQLIXS.5, G.P.4
#[test]
fn test_validate_keywords_empty_array() {
    let keywords = KeywordsParam::Batch(vec![]);
    let result = RequirementsServer::validate_keywords(&keywords);
    assert!(result.is_ok());
    let filtered = result.unwrap();
    assert!(filtered.is_empty());
}

/// Test 6: validate_keywords rejects more than 100 keywords
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

/// Test 7: validate_keywords accepts exactly 100 keywords
/// Covers Requirement: G.TOOLREQLIXS.5
#[test]
fn test_validate_keywords_at_limit() {
    let keywords = KeywordsParam::Batch((0..100).map(|i| format!("keyword{}", i)).collect());
    let result = RequirementsServer::validate_keywords(&keywords);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 100);
}

/// Test 8: validate_keywords rejects keyword exceeding 200 characters
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

/// Test 9: validate_keywords accepts keyword of exactly 200 characters
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

/// Test 10: search finds requirement by title keyword
/// Covers Requirement: G.TOOLREQLIXS.3, G.TOOLREQLIXS.4
#[test]
fn test_search_finds_by_title() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file(&req_dir, "# Instructions\n");
    create_category_file(
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

/// Test 11: search finds requirement by text keyword
/// Covers Requirement: G.TOOLREQLIXS.3, G.TOOLREQLIXS.4
#[test]
fn test_search_finds_by_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file(&req_dir, "# Instructions\n");
    create_category_file(
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

/// Test 12: search is case-insensitive
/// Covers Requirement: G.TOOLREQLIXS.3 step 5
#[test]
fn test_search_case_insensitive() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file(&req_dir, "# Instructions\n");
    create_category_file(
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

/// Test 13: search with multiple keywords finds requirements matching any
/// Covers Requirement: G.TOOLREQLIXS.3 step 6
#[test]
fn test_search_multiple_keywords_or() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file(&req_dir, "# Instructions\n");
    create_category_file(
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

/// Test 14: search returns empty results when no matches
/// Covers Requirement: G.TOOLREQLIXS.4
#[test]
fn test_search_no_matches() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file(&req_dir, "# Instructions\n");
    create_category_file(
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

/// Test 15: search with empty keywords returns empty results (G.P.4)
/// Covers Requirement: G.TOOLREQLIXS.5, G.P.4
#[test]
fn test_search_empty_keywords() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file(&req_dir, "# Instructions\n");
    create_category_file(
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

/// Test 16: search across multiple categories
/// Covers Requirement: G.TOOLREQLIXS.3 step 1
#[test]
fn test_search_across_categories() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file(&req_dir, "# Instructions\n");
    create_category_file(
        &req_dir,
        "general",
        "# Features\n\n## G.F.1: Security login\n\nUser login with security.\n",
    );
    create_category_file(
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

/// Test 17: search returns correct response structure
/// Covers Requirement: G.TOOLREQLIXS.4
#[test]
fn test_search_response_structure() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file(&req_dir, "# Instructions\n");
    create_category_file(
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

/// Test 18: search validates project_root parameter
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

/// Test 19: search validates operation_description parameter
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

/// Test 20: search with all empty strings in array returns empty results
/// Covers Requirement: G.TOOLREQLIXS.5
#[test]
fn test_search_all_empty_strings_filtered() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file(&req_dir, "# Instructions\n");
    create_category_file(
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
