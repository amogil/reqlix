// Tests for Tool: reqlix_get_instructions (T.R.*)
// Covers Requirements: T.R.1, T.R.2, T.R.3, T.R.4, T.R.5, T.R.6, T.R.7

use reqlix::RequirementsServer;
use tempfile::TempDir;

use super::common::create_requirements_dir;

// =============================================================================
// Tests for reqlix_get_instructions (G.REQLIX_GET_I.*)
// =============================================================================

/// Test: reqlix_get_instructions creates AGENTS.md if not found
/// Precondition: System has no AGENTS.md file
/// Action: Call reqlix_get_instructions
/// Result: Function creates AGENTS.md with placeholder content
/// Covers Requirement: G.REQLIX_GET_I.4, G.REQLIX_GET_I.6
#[test]
fn test_get_instructions_creates_file() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    
    // Verify AGENTS.md doesn't exist initially
    assert!(!req_dir.join("AGENTS.md").exists());

    let params = reqlix::GetInstructionsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test get instructions".to_string(),
    };

    let result = RequirementsServer::handle_get_instructions(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true, "Get instructions should succeed: {}", result);
    
    // Verify AGENTS.md was created
    assert!(req_dir.join("AGENTS.md").exists(), "AGENTS.md should be created");
    
    // Verify content contains placeholder
    let content = std::fs::read_to_string(req_dir.join("AGENTS.md")).unwrap();
    assert!(content.contains("# Instructions"), "AGENTS.md should contain instructions");
}

/// Test: reqlix_get_instructions returns existing content if file exists
/// Precondition: System has AGENTS.md file with content
/// Action: Call reqlix_get_instructions
/// Result: Function returns existing content
/// Covers Requirement: G.REQLIX_GET_I.4
#[test]
fn test_get_instructions_returns_existing_content() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    
    // Create AGENTS.md with custom content
    let custom_content = "# Custom Instructions\n\nCustom content here.";
    std::fs::write(req_dir.join("AGENTS.md"), custom_content).unwrap();

    let params = reqlix::GetInstructionsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test get instructions".to_string(),
    };

    let result = RequirementsServer::handle_get_instructions(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["data"]["content"], custom_content);
}

// =============================================================================
// Tests for T.R.1: Description and T.R.7: Response format
// =============================================================================

/// Test: reqlix_get_instructions returns correct response format (T.R.1, T.R.7)
/// Precondition: System has AGENTS.md file
/// Action: Call handle_get_instructions
/// Result: Function returns JSON with success: true and data.content field
/// Covers Requirement: T.R.1, T.R.7
#[test]
fn test_get_instructions_response_format() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);
    create_requirements_dir(&temp_dir);
    let content = "# Instructions\n\nTest content.";
    std::fs::write(req_dir.join("AGENTS.md"), content).unwrap();

    let params = reqlix::GetInstructionsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test get instructions".to_string(),
    };

    let result = RequirementsServer::handle_get_instructions(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Verify response format (T.R.7)
    assert_eq!(parsed["success"], true);
    assert!(parsed["data"].is_object());
    assert!(parsed["data"]["content"].is_string());
    assert_eq!(parsed["data"]["content"], content);
}

// =============================================================================
// Tests for T.R.2: Parameters
// =============================================================================

/// Test: reqlix_get_instructions validates project_root parameter (T.R.2)
/// Precondition: System has empty project_root
/// Action: Call handle_get_instructions with empty project_root
/// Result: Function returns validation error
/// Covers Requirement: T.R.2, G.P.1, G.P.2
#[test]
fn test_get_instructions_validates_project_root() {
    let params = reqlix::GetInstructionsParams {
        project_root: "".to_string(),
        operation_description: "Test".to_string(),
    };

    let result = RequirementsServer::handle_get_instructions(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("project_root"));
}

/// Test: reqlix_get_instructions validates operation_description parameter (T.R.2)
/// Precondition: System has empty operation_description
/// Action: Call handle_get_instructions with empty operation_description
/// Result: Function returns validation error
/// Covers Requirement: T.R.2, G.P.1, G.P.2
#[test]
fn test_get_instructions_validates_operation_description() {
    let temp_dir = TempDir::new().unwrap();
    let params = reqlix::GetInstructionsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "".to_string(),
    };

    let result = RequirementsServer::handle_get_instructions(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], false);
    assert!(parsed["error"].as_str().unwrap().contains("operation_description"));
}

// =============================================================================
// Tests for T.R.6: Placeholder content
// =============================================================================

/// Test: reqlix_get_instructions replaces {requirements_directory} placeholder (T.R.6)
/// Precondition: System has no AGENTS.md file
/// Action: Call handle_get_instructions
/// Result: Created AGENTS.md contains placeholder replaced with actual directory path
/// Covers Requirement: T.R.6
#[test]
fn test_get_instructions_replaces_placeholder() {
    let temp_dir = TempDir::new().unwrap();
    let req_dir = create_requirements_dir(&temp_dir);

    let params = reqlix::GetInstructionsParams {
        project_root: temp_dir.path().to_string_lossy().to_string(),
        operation_description: "Test get instructions".to_string(),
    };

    let result = RequirementsServer::handle_get_instructions(params);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["success"], true);

    // Verify placeholder was replaced (T.R.6)
    let content = std::fs::read_to_string(req_dir.join("AGENTS.md")).unwrap();
    assert!(
        !content.contains("{requirements_directory}"),
        "Placeholder should be replaced. Content: {}",
        content
    );
    assert!(
        content.contains("docs/development/requirements"),
        "Should contain actual directory path"
    );
}
