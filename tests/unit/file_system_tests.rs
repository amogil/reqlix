// Unit tests for file system helper functions
// Covers Requirement: G.REQLIX_GET_INST.3, G.REQLIX_GET_INST.4, G.C.1, G.C.2

use reqlix::RequirementsServer;
use std::path::PathBuf;

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
    assert!(result.contains(&PathBuf::from("/test/project/docs/development/requirements/AGENTS.md")));
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
    assert!(result.len() >= 3);
    assert!(result[0] == PathBuf::from("/test/project/custom/path/AGENTS.md"));
    
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
    assert!(result.contains(&PathBuf::from("project/docs/development/requirements/AGENTS.md")));
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
    assert_eq!(result, PathBuf::from("/test/project/docs/development/requirements/AGENTS.md"));
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
    assert_eq!(result, PathBuf::from("docs/development/requirements/AGENTS.md"));
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
    assert_eq!(result, PathBuf::from("project/docs/development/requirements/AGENTS.md"));
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


