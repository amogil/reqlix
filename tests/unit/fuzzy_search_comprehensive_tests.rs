// Comprehensive tests for fuzzy search covering various scenarios
// Covers Requirements: T.REQLIXF.*

use reqlix::RequirementsServer;
use tempfile::TempDir;

use super::common::{
    create_agents_file_in_req_dir, create_category_file_in_req_dir, create_requirements_dir,
};

// =============================================================================
// Comprehensive fuzzy search tests
// =============================================================================

#[test]
fn test_fuzzy_search_empty_query_returns_empty() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], false); // Empty query should fail validation
}

#[test]
fn test_fuzzy_search_limit_boundary_values() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let mut content = String::from("# Chapter\n\n");
    for i in 1..=5 {
        content.push_str(&format!(
            "## G.C.{}: Requirement {}\n<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->\n\nContent {}.\n\n",
            i, i, i
        ));
    }
    create_category_file_in_req_dir(&req_dir, "general", &content);

    // Test limit = 1
    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test".to_string(),
        limit: Some(1),
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["data"]["results"].as_array().unwrap().len(), 1);

    // Test limit = 1000
    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test".to_string(),
        limit: Some(1000),
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    assert!(results.len() <= 1000);
}

#[test]
fn test_fuzzy_search_query_whitespace_only() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "   ".to_string(), // Only whitespace
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    // Should fail validation or return empty results
    assert_eq!(parsed["success"], false);
}

#[test]
fn test_fuzzy_search_similarity_decreasing_order() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    // Create requirements with different embeddings
    let content = r#"# Chapter

## G.C.1: First
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content1.

## G.C.2: Second
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content2.

## G.C.3: Third
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content3.
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
    
    // Verify ordering: each result should have similarity >= next result
    for i in 0..(results.len().saturating_sub(1)) {
        let sim1 = results[i]["similarity"].as_f64().unwrap();
        let sim2 = results[i + 1]["similarity"].as_f64().unwrap();
        assert!(sim1 >= sim2, "Results must be ordered by similarity (descending)");
    }
}

#[test]
fn test_fuzzy_search_results_contain_required_fields() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test Requirement
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Test content here.
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
        let result = &results[0];
        // Verify all required fields exist
        assert!(result["index"].is_string());
        assert!(result["title"].is_string());
        assert!(result["text"].is_string());
        assert!(result["similarity"].is_number());
        // Verify values
        assert_eq!(result["index"], "G.C.1");
        assert_eq!(result["title"], "Test Requirement");
        assert!(result["text"].as_str().unwrap().contains("Test content"));
    }
}

#[test]
fn test_fuzzy_search_response_structure() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
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
    
    // Verify response structure
    assert!(parsed.is_object());
    assert!(parsed["success"].is_boolean());
    if parsed["success"].as_bool().unwrap() {
        assert!(parsed["data"].is_object());
        assert!(parsed["data"]["query"].is_string());
        assert!(parsed["data"]["results"].is_array());
    } else {
        assert!(parsed["error"].is_string());
    }
}

#[test]
fn test_fuzzy_search_similarity_range() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
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
    for result in results {
        let similarity = result["similarity"].as_f64().unwrap();
        // Cosine similarity should be in [-1, 1] range
        assert!((-1.0..=1.0).contains(&similarity), 
                "Similarity {} is out of range [-1, 1]", similarity);
    }
}

#[test]
fn test_fuzzy_search_handles_mixed_embeddings() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    // Mix of valid and invalid embeddings
    let content = r#"# Chapter

## G.C.1: Valid1
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content1.

## G.C.2: Invalid
<!--embedding:paraphrase-MiniLM-L3-v2:invalid-->

Content2.

## G.C.3: Valid2
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content3.

## G.C.4: No Embedding

Content4.
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
    // Should only return requirements with valid embeddings
    assert_eq!(results.len(), 2);
    let indices: Vec<&str> = results.iter()
        .map(|r| r["index"].as_str().unwrap())
        .collect();
    assert!(indices.contains(&"G.C.1"));
    assert!(indices.contains(&"G.C.3"));
    assert!(!indices.contains(&"G.C.2"));
    assert!(!indices.contains(&"G.C.4"));
}

#[test]
fn test_fuzzy_search_large_number_of_requirements() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let mut content = String::from("# Chapter\n\n");
    for i in 1..=500 {
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
        limit: Some(50),
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);
    
    let results = parsed["data"]["results"].as_array().unwrap();
    assert_eq!(results.len(), 50);
}

#[test]
fn test_fuzzy_search_consistency_multiple_calls() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
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
    
    // Call multiple times
    let result1 = RequirementsServer::handle_fuzzy_search_requirements(params.clone());
    let result2 = RequirementsServer::handle_fuzzy_search_requirements(params.clone());
    let result3 = RequirementsServer::handle_fuzzy_search_requirements(params);
    
    let parsed1: serde_json::Value = serde_json::from_str(&result1).unwrap();
    let parsed2: serde_json::Value = serde_json::from_str(&result2).unwrap();
    let parsed3: serde_json::Value = serde_json::from_str(&result3).unwrap();
    
    assert_eq!(parsed1["success"], true);
    assert_eq!(parsed2["success"], true);
    assert_eq!(parsed3["success"], true);
    
    // Results should be consistent
    let results1 = parsed1["data"]["results"].as_array().unwrap();
    let results2 = parsed2["data"]["results"].as_array().unwrap();
    let results3 = parsed3["data"]["results"].as_array().unwrap();
    
    assert_eq!(results1.len(), results2.len());
    assert_eq!(results2.len(), results3.len());
}

#[test]
fn test_fuzzy_search_different_queries_same_requirements() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test Requirement
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content here.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let queries = vec!["test", "requirement", "content", "here"];
    let mut all_results = Vec::new();
    
    for query in queries {
        let params = reqlix::FuzzySearchRequirementsParams {
            project_root: temp_dir.path().to_string_lossy().to_string(),
            operation_description: "Test".to_string(),
            query: query.to_string(),
            limit: None,
        };
        let result = RequirementsServer::handle_fuzzy_search_requirements(params);
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["success"], true);
        all_results.push(parsed["data"]["results"].as_array().unwrap().len());
    }
    
    // All queries should find the same requirement (though with different similarities)
    assert!(all_results.iter().all(|&len| len == 1 || len == 0));
}

#[test]
fn test_fuzzy_search_limit_zero_validation() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");

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
}

#[test]
fn test_fuzzy_search_limit_over_1000_validation() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");

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

#[test]
fn test_fuzzy_search_query_over_max_length() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");

    let long_query = "x".repeat(10001); // Over max length
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
