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
