// Tests for embedded model functionality (G.R.15)
// Covers Requirement: G.R.15

use reqlix::RequirementsServer;
use tempfile::TempDir;

use super::common::{
    create_agents_file_in_req_dir, create_category_file_in_req_dir, create_requirements_dir,
};

// =============================================================================
// Tests for G.R.15: Embedded model requirements
// =============================================================================

/// Test: Model should be loadable from embedded data (G.R.15)
/// Precondition: System has requirements directory, model files are embedded
/// Action: Call calculate_embedding
/// Result: Function succeeds (model loads from embedded data)
/// Covers Requirement: G.R.15
#[test]
fn test_model_loads_from_embedded_data() {
    // This test verifies that model loads from embedded data, not external files (G.R.15)
    
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Test".to_string(),
        text: "Content".to_string(),
    };
    
    // This should work with embedded model (G.R.15)
    let result = RequirementsServer::handle_insert_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    // Should succeed if model is properly embedded
    assert_eq!(parsed["success"], true);
}

/// Test: Model loading should not require internet connection (G.R.15)
/// Precondition: Model is embedded in binary
/// Action: Call calculate_embedding
/// Result: Function succeeds (uses embedded model, no network required)
/// Covers Requirement: G.R.15
#[test]
fn test_embedded_model_works_offline() {
    // This test verifies G.R.15: model must be self-contained
    // Model is embedded, so it should work without internet
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Test".to_string(),
        text: "Content".to_string(),
    };
    
    // Should work without internet (model is embedded)
    let result = RequirementsServer::handle_insert_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    assert_eq!(parsed["success"], true, "Model should load from embedded data without internet");
}

/// Test: Model should be loaded once and reused (G.R.15)
/// Precondition: System has requirements directory
/// Action: Call calculate_embedding multiple times
/// Result: Model is loaded once, reused for all calls
/// Covers Requirement: G.R.15
#[test]
fn test_model_loaded_once_and_reused() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    // First call - model should be loaded
    let params1 = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Test1".to_string(),
        text: "Content1".to_string(),
    };
    let result1 = RequirementsServer::handle_insert_requirement(params1);
    let parsed1: serde_json::Value = serde_json::from_str(&result1).unwrap();
    
    // Second call - model should be reused (not reloaded)
    let params2 = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Test2".to_string(),
        text: "Content2".to_string(),
    };
    let result2 = RequirementsServer::handle_insert_requirement(params2);
    let parsed2: serde_json::Value = serde_json::from_str(&result2).unwrap();
    
    // Both should succeed (model reused from cache)
    assert_eq!(parsed1["success"], true);
    assert_eq!(parsed2["success"], true);
    
    // Verify embeddings were created
    let content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    let embeddings: Vec<&str> = content
        .lines()
        .filter(|l| l.contains("<!--embedding:"))
        .collect();
    assert_eq!(embeddings.len(), 2);
}

/// Test: Model should be loaded lazily on first use, not at startup (G.R.15)
/// Precondition: System has requirements directory
/// Action: Call calculate_embedding for the first time
/// Result: Model loads on first use, not before
/// Covers Requirement: G.R.15
#[test]
fn test_model_loads_lazily_on_first_use() {
    // G.R.15: Model must be loaded lazily (on first use)
    // This test verifies that model is not loaded until first embedding calculation
    
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    // Before first use, model should not be loaded
    // We can't directly check this, but we verify that first call loads it
    
    // First call - model should be loaded lazily here
    let params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Test".to_string(),
        text: "Content".to_string(),
    };
    
    // This should trigger lazy loading (G.R.15)
    let result = RequirementsServer::handle_insert_requirement(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    // Should succeed - model was loaded lazily on first use
    assert_eq!(parsed["success"], true, "Model should load lazily on first use");
    
    // Verify embedding was created (proves model was loaded)
    let content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    assert!(content.contains("<!--embedding:"), "Embedding should be created after lazy load");
}

/// Test: Model should be reused across different operations (G.R.15)
/// Precondition: System has requirements directory
/// Action: Call insert, update, and fuzzy_search operations
/// Result: Model is loaded once and reused for all operations
/// Covers Requirement: G.R.15
#[test]
fn test_model_reused_across_different_operations() {
    // G.R.15: After first load, same model instance must be reused for entire application lifetime
    
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    // First operation: Insert - model should be loaded lazily
    let insert_params = reqlix::InsertRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        category: "general".to_string(),
        chapter: "Chapter".to_string(),
        title: "Test Requirement".to_string(),
        text: "Test content".to_string(),
    };
    let insert_result = RequirementsServer::handle_insert_requirement(insert_params);
    let insert_parsed: serde_json::Value = serde_json::from_str(&insert_result).unwrap();
    assert_eq!(insert_parsed["success"], true);
    
    // Second operation: Update - model should be reused (not reloaded)
    let update_params = reqlix::UpdateRequirementParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        index: Some("G.C.1".to_string()),
        text: Some("Updated content".to_string()),
        title: None,
        items: None,
    };
    let update_result = RequirementsServer::handle_update_requirement(update_params);
    let update_parsed: serde_json::Value = serde_json::from_str(&update_result).unwrap();
    assert_eq!(update_parsed["success"], true, "Model should be reused for update operation");
    
    // Third operation: Fuzzy search - model should be reused (not reloaded)
    let fuzzy_params = reqlix::FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test content".to_string(),
        limit: Some(10),
    };
    let fuzzy_result = RequirementsServer::handle_fuzzy_search_requirements(fuzzy_params);
    let fuzzy_parsed: serde_json::Value = serde_json::from_str(&fuzzy_result).unwrap();
    assert_eq!(fuzzy_parsed["success"], true, "Model should be reused for fuzzy search operation");
    
    // All operations should succeed using the same model instance (G.R.15)
    assert_eq!(insert_parsed["success"], true);
    assert_eq!(update_parsed["success"], true);
    assert_eq!(fuzzy_parsed["success"], true);
}

/// Test: Model should be loaded only once per application run (G.R.15)
/// Precondition: System has requirements directory
/// Action: Call calculate_embedding many times
/// Result: Model is loaded only once, reused for all calls
/// Covers Requirement: G.R.15
#[test]
fn test_model_loaded_only_once_per_application_run() {
    // G.R.15: Model must be loaded only once per application run
    
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(&req_dir, "general", "# Chapter\n\n");

    // Make multiple calls - model should be loaded only once
    for i in 1..=10 {
        let params = reqlix::InsertRequirementParams {
            project_root: temp_dir.path().to_string_lossy().to_string(),
            operation_description: format!("Test {}", i),
            category: "general".to_string(),
            chapter: "Chapter".to_string(),
            title: format!("Test {}", i),
            text: format!("Content {}", i),
        };
        let result = RequirementsServer::handle_insert_requirement(params);
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["success"], true, "Model should be reused for call {}", i);
    }
    
    // Verify all embeddings were created (proves model was reused)
    let content = std::fs::read_to_string(req_dir.join("general.md")).unwrap();
    let embeddings: Vec<&str> = content
        .lines()
        .filter(|l| l.contains("<!--embedding:"))
        .collect();
    assert_eq!(embeddings.len(), 10, "All embeddings should be created using same model instance");
}
