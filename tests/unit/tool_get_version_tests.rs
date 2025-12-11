// Tests for Tool: reqlix_get_version (G.TOOLREQLIXGETV.*)
// Covers Requirements: G.TOOLREQLIXGETV.2, G.TOOLREQLIXGETV.3

use reqlix::RequirementsServer;

// =============================================================================
// Tests for reqlix_get_version (G.TOOLREQLIXGETV.*)
// =============================================================================

/// Test: reqlix_get_version returns version string
/// Precondition: Server is running
/// Action: Call handle_get_version
/// Result: Function returns JSON with version from Cargo.toml
/// Covers Requirement: G.TOOLREQLIXGETV.2, G.TOOLREQLIXGETV.3
#[test]
fn test_get_version_returns_version() {
    let params = reqlix::GetVersionParams {};
    let result = RequirementsServer::handle_get_version(params);

    // Parse JSON response
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Verify success (G.TOOLREQLIXGETV.2)
    assert_eq!(parsed["success"], true);

    // Verify version is present and matches Cargo.toml (G.TOOLREQLIXGETV.3)
    let version = parsed["data"]["version"].as_str().unwrap();
    assert_eq!(version, env!("CARGO_PKG_VERSION"));
}

/// Test: reqlix_get_version always succeeds
/// Precondition: Server is running
/// Action: Call handle_get_version multiple times
/// Result: Function always returns success: true
/// Covers Requirement: G.TOOLREQLIXGETV.2
#[test]
fn test_get_version_always_succeeds() {
    // Call multiple times to verify consistent behavior
    for _ in 0..3 {
        let params = reqlix::GetVersionParams {};
        let result = RequirementsServer::handle_get_version(params);
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["success"], true);
        assert!(parsed["data"]["version"].is_string());
    }
}
