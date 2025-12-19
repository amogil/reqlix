// Tests for MCP server search tools availability (G.G.3)
// Verifies that search tools are available through MCP server in main.rs
// Covers Requirements: G.G.3, T.REQLIXS.1, T.REQLIXF.1

use reqlix::{FuzzySearchRequirementsParams, RequirementsServer, SearchRequirementsParams};
use tempfile::TempDir;

use super::common::{
    create_agents_file_in_req_dir, create_category_file_in_req_dir, create_requirements_dir,
};

// =============================================================================
// Tests for MCP server search tools availability (G.G.3)
// =============================================================================

/// Test: reqlix_search_requirements is available through MCP server
/// Precondition: MCP server in main.rs delegates to RequirementsServer::handle_search_requirements
/// Action: Call RequirementsServer::handle_search_requirements
/// Result: Function works correctly (verifies MCP server can delegate to it)
/// Covers Requirement: G.G.3, T.REQLIXS.1
#[test]
fn test_mcp_search_requirements_available() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test Requirement\n\nContent with keyword.\n",
    );

    let params = SearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test MCP search".to_string(),
        keywords: reqlix::KeywordsParam::Single("keyword".to_string()),
    };

    // This verifies that the handler method exists and works
    // The MCP server in main.rs delegates to this method
    let result = RequirementsServer::handle_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert!(parsed["data"]["results"].is_array());
}

/// Test: reqlix_fuzzy_search_requirements is available through MCP server
/// Precondition: MCP server in main.rs delegates to RequirementsServer::handle_fuzzy_search_requirements
/// Action: Call RequirementsServer::handle_fuzzy_search_requirements
/// Result: Function works correctly (verifies MCP server can delegate to it)
/// Covers Requirement: G.G.3, T.REQLIXF.1
#[test]
fn test_mcp_fuzzy_search_requirements_available() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test\n\nContent.\n",
    );

    let params = FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test MCP fuzzy search".to_string(),
        query: "test query".to_string(),
        limit: None,
    };

    // This verifies that the handler method exists and works
    // The MCP server in main.rs delegates to this method
    let result = RequirementsServer::handle_fuzzy_search_requirements(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["data"]["query"], "test query");
    assert!(parsed["data"]["results"].is_array());
}

/// Test: Both search tools are callable through RequirementsServer (G.G.3)
/// Precondition: All business logic is in lib.rs, main.rs delegates to RequirementsServer
/// Action: Call both search handlers
/// Result: Both handlers work correctly
/// Covers Requirement: G.G.3
#[test]
fn test_mcp_both_search_tools_available() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_agents_file_in_req_dir(&req_dir, "# Instructions\n");
    create_category_file_in_req_dir(
        &req_dir,
        "general",
        "# Chapter\n\n## G.C.1: Test Requirement\n\nContent with keyword.\n",
    );

    // Test keyword search
    let search_params = SearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        keywords: reqlix::KeywordsParam::Single("keyword".to_string()),
    };
    let search_result = RequirementsServer::handle_search_requirements(search_params);
    let search_parsed: serde_json::Value = serde_json::from_str(&search_result).unwrap();
    assert_eq!(search_parsed["success"], true);

    // Test fuzzy search
    let fuzzy_params = FuzzySearchRequirementsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test".to_string(),
        query: "test query".to_string(),
        limit: None,
    };
    let fuzzy_result = RequirementsServer::handle_fuzzy_search_requirements(fuzzy_params);
    let fuzzy_parsed: serde_json::Value = serde_json::from_str(&fuzzy_result).unwrap();
    assert_eq!(fuzzy_parsed["success"], true);

    // Both tools are available through RequirementsServer, which main.rs delegates to (G.G.3)
}
