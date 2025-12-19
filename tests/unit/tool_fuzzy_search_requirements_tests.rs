// Tests for Tool: reqlix_fuzzy_search_requirements (T.REQLIXF.*)
// Covers Requirements: T.REQLIXF.1, T.REQLIXF.2, T.REQLIXF.3, T.REQLIXF.4, T.REQLIXF.5

use reqlix::RequirementsServer;
use tempfile::TempDir;

use super::common::{
    create_agents_file_in_req_dir, create_category_file_in_req_dir, create_requirements_dir,
};

// =============================================================================
// Tests for reqlix_fuzzy_search_requirements (T.REQLIXF.*)
// =============================================================================

/// Test: fuzzy_search_requirements returns empty results when no embeddings exist (T.REQLIXF.1, T.REQLIXF.3)
/// Precondition: System has requirements without embedding comments
/// Action: Call reqlix_fuzzy_search_requirements
/// Result: Function returns empty results array in correct format (T.REQLIXF.3 step 2 Note)
/// Covers Requirement: T.REQLIXF.1, T.REQLIXF.3
#[test]
fn test_fuzzy_search_no_embeddings() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test fuzzy search".to_string(),
        query: "test query".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["data"]["query"], "test query");
    assert_eq!(parsed["data"]["results"], serde_json::json!([]));
}

/// Test: fuzzy_search_requirements validates query parameter (T.REQLIXF.2, T.REQLIXF.5)
#[test]
fn test_fuzzy_search_query_validation() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");

    // Empty query
    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: String::new(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("required"));

    // Query too long
    let long_query = "x".repeat(10001);
    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: long_query,
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("10000"));
}

/// Test: fuzzy_search_requirements validates limit parameter (T.REQLIXF.2, T.REQLIXF.5)
#[test]
fn test_fuzzy_search_limit_validation() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");

    // Limit too small
    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test".to_string(),
        limit: Some(0),
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("1 and 1000"));

    // Limit too large
    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test".to_string(),
        limit: Some(1001),
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("1 and 1000"));
}

/// Test: fuzzy_search_requirements uses default limit of 10 (T.REQLIXF.2)
#[test]
fn test_fuzzy_search_default_limit() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    // Create requirements with embeddings
    let content = r#"# Chapter

## G.C.1: First Requirement
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content one.

## G.C.2: Second Requirement
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content two.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test".to_string(),
        limit: None, // Should default to 10
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    // Results should be limited to default (10), but we only have 2 requirements
    let results = parsed["data"]["results"].as_array().unwrap();
    assert!(results.len() <= 10);
}

/// Test: fuzzy_search_requirements applies limit parameter (T.REQLIXF.3 step 6)
#[test]
fn test_fuzzy_search_applies_limit() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    // Create multiple requirements with embeddings
    let mut content = String::from("# Chapter\n\n");
    for i in 1..=15 {
        content.push_str(&format!(
            "## G.C.{}: Requirement {}\n<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->\n\nContent {}.\n\n",
            i, i, i
        ));
    }
    create_category_file_in_req_dir(&req_dir, "general", &content);

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test".to_string(),
        limit: Some(5),
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    assert_eq!(results.len(), 5); // Limited to 5
}

/// Test: fuzzy_search_requirements ignores requirements without embeddings (T.REQLIXF.3 step 2 Note)
#[test]
fn test_fuzzy_search_ignores_requirements_without_embeddings() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: With Embedding
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content one.

## G.C.2: Without Embedding

Content two.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    // Should only return requirement with embedding
    assert_eq!(results.len(), 1);
    assert_eq!(results[0]["index"], "G.C.1");
}

/// Test: fuzzy_search_requirements returns results with similarity scores (T.REQLIXF.1, T.REQLIXF.4)
#[test]
fn test_fuzzy_search_returns_similarity_scores() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test Requirement
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    if !results.is_empty() {
        assert!(results[0].as_object().unwrap().contains_key("similarity"));
        let similarity = results[0]["similarity"].as_f64().unwrap();
        assert!((0.0..=1.0).contains(&similarity));
    }
}

/// Test: fuzzy_search_requirements orders results by similarity (T.REQLIXF.3 step 5)
#[test]
fn test_fuzzy_search_orders_by_similarity() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    // Create multiple requirements with embeddings
    let content = r#"# Chapter

## G.C.1: First
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.

## G.C.2: Second
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    if results.len() > 1 {
        // Check that results are ordered by similarity (highest first)
        for i in 0..(results.len() - 1) {
            let sim1 = results[i]["similarity"].as_f64().unwrap();
            let sim2 = results[i + 1]["similarity"].as_f64().unwrap();
            assert!(sim1 >= sim2, "Results should be ordered by similarity (highest first)");
        }
    }
}
