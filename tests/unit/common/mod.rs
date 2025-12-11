// Common helper functions for unit tests

use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Create requirements directory structure
pub fn create_requirements_dir(temp_dir: &TempDir) -> std::path::PathBuf {
    let req_dir = temp_dir.path().join("docs/development/requirements");
    fs::create_dir_all(&req_dir).unwrap();
    req_dir
}

/// Create a category file with content in temp directory
pub fn create_category_file(temp_dir: &TempDir, category: &str, content: &str) {
    let file_path = temp_dir.path().join(format!("{}.md", category));
    fs::write(&file_path, content).unwrap();
}

/// Create AGENTS.md file in temp directory
pub fn create_agents_file(temp_dir: &TempDir, content: &str) {
    let file_path = temp_dir.path().join("AGENTS.md");
    fs::write(&file_path, content).unwrap();
}

/// Create a category file in requirements directory
pub fn create_category_file_in_req_dir(req_dir: &Path, category: &str, content: &str) {
    let file_path = req_dir.join(format!("{}.md", category));
    fs::write(&file_path, content).unwrap();
}

/// Create AGENTS.md in requirements directory
pub fn create_agents_file_in_req_dir(req_dir: &Path, content: &str) {
    let file_path = req_dir.join("AGENTS.md");
    fs::write(&file_path, content).unwrap();
}

/// Parse JSON response string
pub fn parse_response(response: &str) -> serde_json::Value {
    serde_json::from_str(response).unwrap()
}
