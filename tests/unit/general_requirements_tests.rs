// Tests for General Requirements (G.G.*)
// Covers Requirements: G.G.3

use reqlix::RequirementsServer;

// =============================================================================
// Tests for G.G.3: Code architecture separation
// =============================================================================

/// Test: main.rs delegates to lib.rs (G.G.3)
/// Precondition: System has main.rs and lib.rs modules
/// Action: Verify that RequirementsServer methods are callable from tests
/// Result: All handler methods are accessible through RequirementsServer
/// Covers Requirement: G.G.3
#[test]
fn test_main_delegates_to_lib() {
    // Verify that RequirementsServer exists and has public handler methods
    // This ensures main.rs can delegate to lib.rs
    
    // Test that we can call handler methods directly (which main.rs delegates to)
    let params = reqlix::GetVersionParams {};
    let result = RequirementsServer::handle_get_version(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    // If this succeeds, it means the handler is accessible from lib.rs
    // and main.rs can delegate to it
    assert_eq!(parsed["success"], true);
    assert!(parsed["data"]["version"].is_string());
}

/// Test: RequirementsServer has all required handler methods (G.G.3)
/// Precondition: System has lib.rs with RequirementsServer
/// Action: Verify all handler methods exist
/// Result: All handler methods are accessible
/// Covers Requirement: G.G.3
#[test]
fn test_requirements_server_has_all_handlers() {
    // Verify that all handler methods exist on RequirementsServer
    // This ensures main.rs can delegate all tool calls to lib.rs
    
    // These should all compile and be callable
    let _get_instructions = RequirementsServer::handle_get_instructions;
    let _get_categories = RequirementsServer::handle_get_categories;
    let _get_chapters = RequirementsServer::handle_get_chapters;
    let _get_requirements = RequirementsServer::handle_get_requirements;
    let _get_requirement = RequirementsServer::handle_get_requirement;
    let _insert_requirement = RequirementsServer::handle_insert_requirement;
    let _update_requirement = RequirementsServer::handle_update_requirement;
    let _delete_requirement = RequirementsServer::handle_delete_requirement;
    let _search_requirements = RequirementsServer::handle_search_requirements;
    let _fuzzy_search_requirements = RequirementsServer::handle_fuzzy_search_requirements;
    let _get_version = RequirementsServer::handle_get_version;
    
    // If this compiles, all handlers are accessible from lib.rs
    // and main.rs can delegate to them
}
