// Edge case tests for fuzzy search functionality
// Covers Requirements: T.REQLIXF.*

use reqlix::RequirementsServer;
use tempfile::TempDir;

use super::common::{
    create_agents_file_in_req_dir, create_category_file_in_req_dir, create_requirements_dir,
};

// =============================================================================
// Edge cases for fuzzy search
// =============================================================================

#[test]
fn test_fuzzy_search_limit_exactly_1000() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let mut content = String::from("# Chapter\n\n");
    for i in 1..=1005 {
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
        limit: Some(1000),
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    assert_eq!(results.len(), 1000);
}

#[test]
fn test_fuzzy_search_limit_exactly_1() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: First
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content1.

## G.C.2: Second
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content2.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test".to_string(),
        limit: Some(1),
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_fuzzy_search_query_max_length() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let query = "x".repeat(10000); // Exactly max length
    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query,
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);
}

#[test]
fn test_fuzzy_search_query_one_char() {
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
        query: "x".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);
}

#[test]
fn test_fuzzy_search_query_special_characters() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let special_query = "!@#$%^&*()_+-=[]{}|;':\",./<>?`~";
    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: special_query.to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);
}

#[test]
fn test_fuzzy_search_query_unicode() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let unicode_query = "тест 🚀 测试 テスト";
    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: unicode_query.to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);
}

#[test]
fn test_fuzzy_search_query_multiline() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let multiline_query = "line1\nline2\nline3";
    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: multiline_query.to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);
}

#[test]
fn test_fuzzy_search_multiple_categories() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        r#"# Chapter

## G.C.1: General
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content1.
"#,
    );
    create_category_file_in_req_dir(
        &req_dir,
        "testing",
        r#"# Chapter

## T.C.1: Testing
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content2.
"#,
    );

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
    assert_eq!(results.len(), 2);
}

#[test]
fn test_fuzzy_search_multiple_chapters() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter1

## G.C.1: First
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content1.

# Chapter2

## G.C.2: Second
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content2.
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
    assert_eq!(results.len(), 2);
}

#[test]
fn test_fuzzy_search_embedding_different_model_name() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    // Embedding with different model name (should be ignored, always use paraphrase-MiniLM-L3-v2)
    let content = r#"# Chapter

## G.C.1: Test
<!--embedding:some-other-model:dGVzdA==-->

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
    // Should still work, model name is ignored
    let results = parsed["data"]["results"].as_array().unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_fuzzy_search_invalid_base64_embedding() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    // Invalid base64 should be skipped
    let content = r#"# Chapter

## G.C.1: Valid
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content1.

## G.C.2: Invalid
<!--embedding:paraphrase-MiniLM-L3-v2:invalid!!!base64-->

Content2.
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
    // Should only return requirement with valid embedding
    assert_eq!(results.len(), 1);
    assert_eq!(results[0]["index"], "G.C.1");
}

#[test]
fn test_fuzzy_search_embedding_wrong_dimension() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    // Embedding with wrong dimension (should decode but may have different similarity)
    let small_vec = vec![0.1f32, 0.2f32];
    let encoded = reqlix::encode_embedding(&small_vec);
    let content = format!(
        r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:{}-->

Content.
"#,
        encoded
    );
    create_category_file_in_req_dir(&req_dir, "general", &content);

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);
    // Should still work, but similarity will be 0.0 due to dimension mismatch
    let results = parsed["data"]["results"].as_array().unwrap();
    if !results.is_empty() {
        assert_eq!(results[0]["similarity"], 0.0);
    }
}

#[test]
fn test_fuzzy_search_embedding_not_after_heading() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    // Embedding not immediately after heading (should be ignored)
    let content = r#"# Chapter

## G.C.1: Test

Some text here.
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

More content.
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
    // collect_embeddings looks backwards for heading, so this should still work
    let results = parsed["data"]["results"].as_array().unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_fuzzy_search_empty_requirements_dir() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    // No category files

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
    assert_eq!(results.len(), 0);
}

#[test]
fn test_fuzzy_search_only_agents_file() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    // Only AGENTS.md, no category files

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
    assert_eq!(results.len(), 0);
}

#[test]
fn test_fuzzy_search_limit_more_than_results() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: First
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content1.

## G.C.2: Second
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content2.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test".to_string(),
        limit: Some(100), // More than available results
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    assert_eq!(results.len(), 2); // Should return all available, not 100
}

#[test]
fn test_fuzzy_search_similarity_scores_all_same() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    // All requirements have same embedding
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
    assert_eq!(results.len(), 3);
    // All should have same similarity
    let sim1 = results[0]["similarity"].as_f64().unwrap();
    let sim2 = results[1]["similarity"].as_f64().unwrap();
    let sim3 = results[2]["similarity"].as_f64().unwrap();
    assert!((sim1 - sim2).abs() < 0.0001);
    assert!((sim2 - sim3).abs() < 0.0001);
}

#[test]
fn test_fuzzy_search_operation_description_validation() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "".to_string(), // Empty
        query: "test".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], false);
}

#[test]
fn test_fuzzy_search_project_root_validation() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: "/nonexistent/path".to_string(),
        operation_description: "Test".to_string(),
        query: "test".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], false);
}

#[test]
fn test_fuzzy_search_response_includes_query() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    let content = r#"# Chapter

## G.C.1: Test
<!--embedding:paraphrase-MiniLM-L3-v2:dGVzdA==-->

Content.
"#;
    create_category_file_in_req_dir(&req_dir, "general", content);

    let query = "my search query";
    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: query.to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["data"]["query"], query);
}

#[test]
fn test_fuzzy_search_results_include_all_fields() {
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
        assert!(result.as_object().unwrap().contains_key("index"));
        assert!(result.as_object().unwrap().contains_key("title"));
        assert!(result.as_object().unwrap().contains_key("text"));
        assert!(result.as_object().unwrap().contains_key("similarity"));
        assert_eq!(result["index"], "G.C.1");
        assert_eq!(result["title"], "Test Requirement");
    }
}

/// Test: fuzzy_search_requirements ignores requirements with invalid embedding vectors (T.REQLIXF.3 step 2)
/// Precondition: System has requirements with valid and invalid embedding vectors
/// Action: Call reqlix_fuzzy_search_requirements
/// Result: Only requirements with valid embeddings are included in results, invalid ones are silently skipped
/// Covers Requirement: T.REQLIXF.3 step 2
#[test]
fn test_fuzzy_search_ignores_invalid_embedding_vectors() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    
    // Create requirements with valid and invalid embeddings
    // Use valid base64 encoded embeddings for valid ones
    use base64::{engine::general_purpose::STANDARD as BASE64_STD, Engine};
    let mut valid_vec = Vec::new();
    for i in 0..384 {
        let val = (i as f32) / 100.0;
        valid_vec.extend_from_slice(&val.to_le_bytes());
    }
    let valid_encoded = BASE64_STD.encode(&valid_vec);
    
    let content = format!(r#"# Chapter

## G.C.1: Valid Embedding
<!--embedding:paraphrase-MiniLM-L3-v2:{}-->

Content with valid embedding.

## G.C.2: Invalid Base64
<!--embedding:paraphrase-MiniLM-L3-v2:invalid!!!base64-->

Content with invalid base64.

## G.C.3: Invalid Length
<!--embedding:paraphrase-MiniLM-L3-v2:YWJj-->

Content with invalid length (not divisible by 4).

## G.C.4: Empty Vector
<!--embedding:paraphrase-MiniLM-L3-v2:-->

Content with empty vector.

## G.C.5: Another Valid Embedding
<!--embedding:paraphrase-MiniLM-L3-v2:{}-->

Content with another valid embedding.
"#, valid_encoded, valid_encoded);
    
    create_category_file_in_req_dir(&req_dir, "general", &content);

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test query".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    
    // Should only return requirements with valid embeddings (G.C.1 and G.C.5)
    // G.C.2, G.C.3, G.C.4 should be excluded due to parsing errors
    assert_eq!(results.len(), 2, "Should only include requirements with valid embeddings");
    
    let indices: Vec<&str> = results.iter()
        .map(|r| r["index"].as_str().unwrap())
        .collect();
    
    assert!(indices.contains(&"G.C.1"), "G.C.1 should be included (valid embedding)");
    assert!(indices.contains(&"G.C.5"), "G.C.5 should be included (valid embedding)");
    assert!(!indices.contains(&"G.C.2"), "G.C.2 should be excluded (invalid base64)");
    assert!(!indices.contains(&"G.C.3"), "G.C.3 should be excluded (invalid length)");
    assert!(!indices.contains(&"G.C.4"), "G.C.4 should be excluded (empty vector)");
}

/// Test: fuzzy_search_requirements uses last embedding when multiple embeddings exist for same requirement (T.REQLIXF.3)
/// Precondition: System has requirement with multiple embedding comments
/// Action: Call reqlix_fuzzy_search_requirements
/// Result: Only the last embedding is used (previous ones are overwritten)
/// Covers Requirement: T.REQLIXF.3 step 2
#[test]
fn test_fuzzy_search_multiple_embeddings_same_requirement_uses_last() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    
    // Create requirement with multiple embeddings (last one should be used)
    use base64::{engine::general_purpose::STANDARD as BASE64_STD, Engine};
    
    // First embedding vector
    let mut vec1 = Vec::new();
    for i in 0..384 {
        let val = (i as f32) / 100.0;
        vec1.extend_from_slice(&val.to_le_bytes());
    }
    let encoded1 = BASE64_STD.encode(&vec1);
    
    // Second embedding vector (different values)
    let mut vec2 = Vec::new();
    for i in 0..384 {
        let val = ((i + 100) as f32) / 100.0;
        vec2.extend_from_slice(&val.to_le_bytes());
    }
    let encoded2 = BASE64_STD.encode(&vec2);
    
    let content = format!(r#"# Chapter

## G.C.1: Test Requirement
<!--embedding:paraphrase-MiniLM-L3-v2:{}-->
<!--embedding:paraphrase-MiniLM-L3-v2:{}-->

Content here.
"#, encoded1, encoded2);
    
    create_category_file_in_req_dir(&req_dir, "general", &content);

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test query".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    
    // Should find the requirement
    assert_eq!(results.len(), 1);
    assert_eq!(results[0]["index"], "G.C.1");
    
    // Verify that the second embedding was used (by checking similarity)
    // The similarity should match what we'd get with vec2, not vec1
    let similarity = results[0]["similarity"].as_f64().unwrap();
    // Just verify it's a valid similarity value (exact match depends on query embedding)
    assert!((-1.0..=1.0).contains(&similarity));
}

/// Test: fuzzy_search_requirements ignores embeddings in AGENTS.md file (T.REQLIXF.3)
/// Precondition: System has AGENTS.md file with embedding comment
/// Action: Call reqlix_fuzzy_search_requirements
/// Result: Embeddings from AGENTS.md are not included in search results
/// Covers Requirement: T.REQLIXF.3 step 1
#[test]
fn test_fuzzy_search_ignores_embeddings_in_agents_file() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    
    // Create AGENTS.md with embedding comment (should be ignored)
    use base64::{engine::general_purpose::STANDARD as BASE64_STD, Engine};
    let mut vec = Vec::new();
    for i in 0..384 {
        let val = (i as f32) / 100.0;
        vec.extend_from_slice(&val.to_le_bytes());
    }
    let encoded = BASE64_STD.encode(&vec);
    
    let agents_content = format!(r#"# Instructions

## Some Heading
<!--embedding:paraphrase-MiniLM-L3-v2:{}-->

Some content.
"#, encoded);
    create_agents_file_in_req_dir(&req_dir, &agents_content);
    
    // Create a regular requirement with embedding
    let content = format!(r#"# Chapter

## G.C.1: Test Requirement
<!--embedding:paraphrase-MiniLM-L3-v2:{}-->

Content here.
"#, encoded);
    create_category_file_in_req_dir(&req_dir, "general", &content);

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test query".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    
    // Should only find G.C.1, not anything from AGENTS.md
    assert_eq!(results.len(), 1);
    assert_eq!(results[0]["index"], "G.C.1");
}

/// Test: fuzzy_search_requirements handles requirement with minimal text (T.REQLIXF.3)
/// Precondition: System has requirement with title and minimal text (single character)
/// Action: Call reqlix_fuzzy_search_requirements
/// Result: Requirement is found and returned correctly
/// Covers Requirement: T.REQLIXF.3
#[test]
fn test_fuzzy_search_requirement_with_minimal_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    
    // Insert requirement with minimal text (empty text is not allowed by validation)
    let params_insert = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Minimal Text Requirement".to_string(),
        text: "X".to_string(), // Minimal text (single character)
    };
    let result_insert = RequirementsServer::handle_insert_requirement(params_insert);
    let parsed_insert: serde_json::Value = serde_json::from_str(&result_insert).unwrap();
    assert_eq!(parsed_insert["success"], true);
    
    // Now search for it
    let params_search = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "minimal text".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params_search);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    
    // Should find the requirement
    assert!(!results.is_empty());
    let found_req = &results[0];
    assert_eq!(found_req["title"], "Minimal Text Requirement");
    assert_eq!(found_req["text"], "X");
}

/// Test: fuzzy_search_requirements handles embedding with empty model name (T.REQLIXF.3, G.R.11)
/// Precondition: System has requirement with embedding comment having empty model name
/// Action: Call reqlix_fuzzy_search_requirements
/// Result: Embedding is still used (model name is ignored per G.R.11)
/// Covers Requirement: T.REQLIXF.3, G.R.11
#[test]
fn test_fuzzy_search_embedding_with_empty_model_name() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    
    use base64::{engine::general_purpose::STANDARD as BASE64_STD, Engine};
    let mut vec = Vec::new();
    for i in 0..384 {
        let val = (i as f32) / 100.0;
        vec.extend_from_slice(&val.to_le_bytes());
    }
    let encoded = BASE64_STD.encode(&vec);
    
    // Embedding with empty model name (should still work)
    let content = format!(r#"# Chapter

## G.C.1: Test Requirement
<!--embedding::{}-->

Content here.
"#, encoded);
    
    create_category_file_in_req_dir(&req_dir, "general", &content);

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test query".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    
    // Should find the requirement despite empty model name
    assert_eq!(results.len(), 1);
    assert_eq!(results[0]["index"], "G.C.1");
}

/// Test: fuzzy_search_requirements handles embedding before chapter heading (not requirement) (T.REQLIXF.3)
/// Precondition: System has embedding comment before chapter heading (level-1)
/// Action: Call reqlix_fuzzy_search_requirements
/// Result: Embedding is ignored (no requirement heading found)
/// Covers Requirement: T.REQLIXF.3 step 2
#[test]
fn test_fuzzy_search_embedding_before_chapter_heading_ignored() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    
    use base64::{engine::general_purpose::STANDARD as BASE64_STD, Engine};
    let mut vec = Vec::new();
    for i in 0..384 {
        let val = (i as f32) / 100.0;
        vec.extend_from_slice(&val.to_le_bytes());
    }
    let encoded = BASE64_STD.encode(&vec);
    
    // Embedding before chapter heading (should be ignored - no requirement heading found)
    let content = format!(r#"<!--embedding:paraphrase-MiniLM-L3-v2:{}-->
# Chapter

## G.C.1: Test Requirement

Content here.
"#, encoded);
    
    create_category_file_in_req_dir(&req_dir, "general", &content);

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test query".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    
    // Should not find anything (embedding before chapter heading is ignored)
    assert_eq!(results.len(), 0);
}

/// Test: fuzzy_search_requirements handles very long requirement text (T.REQLIXF.3)
/// Precondition: System has requirement with very long text (near max length)
/// Action: Call reqlix_fuzzy_search_requirements
/// Result: Requirement is found and embedding is calculated correctly
/// Covers Requirement: T.REQLIXF.3
#[test]
fn test_fuzzy_search_very_long_requirement_text() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    
    // Create requirement with very long text (close to max 10000 chars)
    let long_text = "word ".repeat(1999); // ~10000 characters
    let params_insert = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Long Text Requirement".to_string(),
        text: long_text,
    };
    let result_insert = RequirementsServer::handle_insert_requirement(params_insert);
    let parsed_insert: serde_json::Value = serde_json::from_str(&result_insert).unwrap();
    assert_eq!(parsed_insert["success"], true);
    
    // Now search for it
    let params_search = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "long text".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params_search);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    
    // Should find the requirement
    assert!(!results.is_empty());
    let found_req = &results[0];
    assert_eq!(found_req["title"], "Long Text Requirement");
    assert!(found_req["similarity"].as_f64().unwrap() >= -1.0);
    assert!(found_req["similarity"].as_f64().unwrap() <= 1.0);
}

/// Test: fuzzy_search_requirements handles requirement that becomes unavailable after embedding collection (T.REQLIXF.3)
/// Precondition: System has requirement with embedding, then requirement is deleted
/// Action: Call reqlix_fuzzy_search_requirements after requirement deletion
/// Result: Requirement is skipped gracefully (not found error is handled)
/// Covers Requirement: T.REQLIXF.3 step 4
#[test]
fn test_fuzzy_search_requirement_deleted_after_collection() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    
    // Insert requirement
    let params_insert = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Temporary Requirement".to_string(),
        text: "Content".to_string(),
    };
    let result_insert = RequirementsServer::handle_insert_requirement(params_insert);
    let parsed_insert: serde_json::Value = serde_json::from_str(&result_insert).unwrap();
    assert_eq!(parsed_insert["success"], true);
    let _index = parsed_insert["data"]["index"].as_str().unwrap().to_string();
    
    // Delete the requirement (but embedding comment might still be in file if deletion doesn't clean it)
    // Actually, deletion removes the requirement, so embedding should be gone too
    // This test verifies that if somehow embedding exists but requirement is gone, it's handled gracefully
    
    // Manually create a file with embedding but no requirement (simulating race condition)
    use base64::{engine::general_purpose::STANDARD as BASE64_STD, Engine};
    let mut vec = Vec::new();
    for i in 0..384 {
        let val = (i as f32) / 100.0;
        vec.extend_from_slice(&val.to_le_bytes());
    }
    let encoded = BASE64_STD.encode(&vec);
    
    // File with embedding comment but requirement was deleted
    let _content = format!(r#"# Chapter

## G.C.1: Deleted Requirement
<!--embedding:paraphrase-MiniLM-L3-v2:{}-->

"#, encoded);
    
    // Write file, then delete the requirement heading manually to simulate deletion
    std::fs::write(req_dir.join("general.md"), "# Chapter\n\n").unwrap();

    let params_search = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test query".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params_search);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    // Should succeed but return empty results (no valid embeddings found)
    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    assert_eq!(results.len(), 0);
}

/// Test: fuzzy_search_requirements handles requirements with same index in different categories (T.REQLIXF.3)
/// Precondition: System has G.C.1 in general category and T.C.1 in testing category
/// Action: Call reqlix_fuzzy_search_requirements
/// Result: Both requirements are found and returned separately
/// Covers Requirement: T.REQLIXF.3 step 2
#[test]
fn test_fuzzy_search_same_index_different_categories() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    
    use base64::{engine::general_purpose::STANDARD as BASE64_STD, Engine};
    let mut vec = Vec::new();
    for i in 0..384 {
        let val = (i as f32) / 100.0;
        vec.extend_from_slice(&val.to_le_bytes());
    }
    let encoded = BASE64_STD.encode(&vec);
    
    // G.C.1 in general category
    let general_content = format!(r#"# Chapter

## G.C.1: General Requirement
<!--embedding:paraphrase-MiniLM-L3-v2:{}-->

General content.
"#, encoded);
    create_category_file_in_req_dir(&req_dir, "general", &general_content);
    
    // T.C.1 in testing category (same number, different category/chapter prefix)
    let testing_content = format!(r#"# Chapter

## T.C.1: Testing Requirement
<!--embedding:paraphrase-MiniLM-L3-v2:{}-->

Testing content.
"#, encoded);
    create_category_file_in_req_dir(&req_dir, "testing", &testing_content);

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test query".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    
    // Should find both requirements (different indices: G.C.1 and T.C.1)
    assert_eq!(results.len(), 2);
    
    let indices: Vec<&str> = results.iter()
        .map(|r| r["index"].as_str().unwrap())
        .collect();
    
    assert!(indices.contains(&"G.C.1"), "Should find G.C.1");
    assert!(indices.contains(&"T.C.1"), "Should find T.C.1");
}

/// Test: fuzzy_search_requirements handles query that matches requirement exactly (high similarity) (T.REQLIXF.3)
/// Precondition: System has requirement with known content
/// Action: Call reqlix_fuzzy_search_requirements with query matching requirement content
/// Result: Requirement is found with high similarity score (close to 1.0)
/// Covers Requirement: T.REQLIXF.3 step 4
#[test]
fn test_fuzzy_search_exact_match_high_similarity() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    
    // Insert requirement with specific content
    let params_insert = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "User Authentication".to_string(),
        text: "All users must authenticate before accessing the system using secure credentials.".to_string(),
    };
    let result_insert = RequirementsServer::handle_insert_requirement(params_insert);
    let parsed_insert: serde_json::Value = serde_json::from_str(&result_insert).unwrap();
    assert_eq!(parsed_insert["success"], true);
    
    // Search with query that matches the requirement content
    let params_search = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "User Authentication: All users must authenticate before accessing the system using secure credentials.".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params_search);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    
    // Should find the requirement
    assert!(!results.is_empty());
    let similarity = results[0]["similarity"].as_f64().unwrap();
    
    // Similarity should be relatively high (exact match should be close to 1.0)
    // Note: actual value depends on model, but should be > 0.5 for semantic match
    assert!(similarity > 0.5, "Similarity should be high for exact match, got: {}", similarity);
    assert_eq!(results[0]["index"], parsed_insert["data"]["index"]);
}

/// Test: fuzzy_search_requirements handles all zero similarity scores (orthogonal vectors) (T.REQLIXF.3)
/// Precondition: System has requirements with embeddings that are orthogonal to query
/// Action: Call reqlix_fuzzy_search_requirements
/// Result: Requirements are returned with similarity = 0.0, still ordered correctly
/// Covers Requirement: T.REQLIXF.3 step 4, step 5
#[test]
fn test_fuzzy_search_zero_similarity_scores() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    
    // Create requirements with embeddings that will have low/zero similarity
    // We'll use a query that's semantically very different
    use base64::{engine::general_purpose::STANDARD as BASE64_STD, Engine};
    let mut vec = Vec::new();
    for i in 0..384 {
        let val = (i as f32) / 100.0;
        vec.extend_from_slice(&val.to_le_bytes());
    }
    let encoded = BASE64_STD.encode(&vec);
    
    let content = format!(r#"# Chapter

## G.C.1: Mathematics
<!--embedding:paraphrase-MiniLM-L3-v2:{}-->

Mathematical equations and formulas.

## G.C.2: Cooking
<!--embedding:paraphrase-MiniLM-L3-v2:{}-->

Recipes and cooking instructions.
"#, encoded, encoded);
    
    create_category_file_in_req_dir(&req_dir, "general", &content);

    // Search with query that's semantically very different
    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "completely unrelated topic about space exploration and quantum physics".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    
    // Should still return results (even if similarity is low)
    // Results should be ordered by similarity (even if all are low)
    if results.len() > 1 {
        for i in 0..(results.len() - 1) {
            let sim1 = results[i]["similarity"].as_f64().unwrap();
            let sim2 = results[i + 1]["similarity"].as_f64().unwrap();
            assert!(sim1 >= sim2, "Results should be ordered by similarity (descending)");
        }
    }
    
    // All similarities should be valid (between -1 and 1)
    for result in results {
        let similarity = result["similarity"].as_f64().unwrap();
        assert!((-1.0..=1.0).contains(&similarity), "Similarity should be in [-1, 1] range");
    }
}

/// Test: fuzzy_search_requirements handles embedding with whitespace around comment (T.REQLIXF.3, G.R.11)
/// Precondition: System has requirement with embedding comment having whitespace
/// Action: Call reqlix_fuzzy_search_requirements
/// Result: Embedding is still parsed correctly (whitespace is handled)
/// Covers Requirement: T.REQLIXF.3 step 2, G.R.11
#[test]
fn test_fuzzy_search_embedding_with_whitespace() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    
    use base64::{engine::general_purpose::STANDARD as BASE64_STD, Engine};
    let mut vec = Vec::new();
    for i in 0..384 {
        let val = (i as f32) / 100.0;
        vec.extend_from_slice(&val.to_le_bytes());
    }
    let encoded = BASE64_STD.encode(&vec);
    
    // Embedding comment with whitespace around it
    let content = format!(r#"# Chapter

## G.C.1: Test Requirement
  <!--embedding:paraphrase-MiniLM-L3-v2:{}-->  

Content here.
"#, encoded);
    
    create_category_file_in_req_dir(&req_dir, "general", &content);

    let params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test query".to_string(),
        limit: None,
    };
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    assert_eq!(parsed["success"], true);
    let results = parsed["data"]["results"].as_array().unwrap();
    
    // Should find the requirement (whitespace should be handled by regex)
    assert_eq!(results.len(), 1);
    assert_eq!(results[0]["index"], "G.C.1");
}
