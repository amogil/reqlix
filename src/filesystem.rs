// File system helpers (G.REQLIX_GET_I.3, G.REQLIX_GET_I.4, G.C.1, G.C.2, G.R.8, G.R.9, G.R.10)

use crate::constants::PLACEHOLDER_CONTENT;
use std::env;
use std::fs;
use std::path::PathBuf;

/// Read file as UTF-8 with proper error handling (G.R.8, G.R.9)
/// Returns content or formatted error message
#[cfg_attr(test, allow(dead_code))]
pub fn read_file_utf8(path: &PathBuf) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => {
            let error_kind = e.kind();
            let path_str = path.to_string_lossy();

            // Handle specific error types (G.R.9)
            let error_msg = match error_kind {
                std::io::ErrorKind::PermissionDenied => {
                    format!("Permission denied: {}", path_str)
                }
                std::io::ErrorKind::NotFound => {
                    format!("File not found: {}", path_str)
                }
                std::io::ErrorKind::InvalidInput => {
                    format!("Encoding error: file is not valid UTF-8: {}", path_str)
                }
                _ => {
                    format!("Failed to read file {}: {}", path_str, e)
                }
            };
            Err(error_msg)
        }
    }
}

/// Write file as UTF-8 with proper error handling (G.R.8, G.R.9)
/// Returns success or formatted error message
#[cfg_attr(test, allow(dead_code))]
pub fn write_file_utf8(path: &PathBuf, content: &str) -> Result<(), String> {
    // Ensure parent directory exists (G.C.2)
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            let path_str = path.to_string_lossy();
            match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    format!("Permission denied: {}", path_str)
                }
                _ => format!("Failed to create directory for {}: {}", path_str, e),
            }
        })?;
    }

    match fs::write(path, content) {
        Ok(_) => Ok(()),
        Err(e) => {
            let error_kind = e.kind();
            let path_str = path.to_string_lossy();

            // Handle specific error types (G.R.9)
            let error_msg = match error_kind {
                std::io::ErrorKind::PermissionDenied => {
                    format!("Permission denied: {}", path_str)
                }
                std::io::ErrorKind::NotFound => {
                    format!("Invalid path: {}", path_str)
                }
                std::io::ErrorKind::OutOfMemory => {
                    format!("Disk full: cannot write to {}", path_str)
                }
                _ => {
                    format!("Failed to write file {}: {}", path_str, e)
                }
            };
            Err(error_msg)
        }
    }
}

/// Check if file is empty (only whitespace) (G.R.10)
#[cfg_attr(test, allow(dead_code))]
pub fn is_file_empty_or_whitespace(content: &str) -> bool {
    content.trim().is_empty()
}

/// Get search paths for AGENTS.md (G.REQLIX_GET_I.3)
#[cfg_attr(test, allow(dead_code))]
pub fn get_search_paths(project_root: &str) -> Vec<PathBuf> {
    let root = PathBuf::from(project_root);
    let mut paths = Vec::new();

    if let Ok(rel_path) = env::var("REQLIX_REQ_REL_PATH") {
        paths.push(root.join(&rel_path).join("AGENTS.md"));
    }

    paths.push(root.join("docs/development/requirements/AGENTS.md"));
    paths.push(root.join("docs/dev/req/AGENTS.md"));

    paths
}

/// Get path for creating AGENTS.md (G.REQLIX_GET_I.4)
#[cfg_attr(test, allow(dead_code))]
pub fn get_create_path(project_root: &str) -> PathBuf {
    let root = PathBuf::from(project_root);

    if let Ok(rel_path) = env::var("REQLIX_REQ_REL_PATH") {
        root.join(&rel_path).join("AGENTS.md")
    } else {
        root.join("docs/development/requirements/AGENTS.md")
    }
}

/// Find or create requirements file (G.REQLIX_GET_I.3, G.REQLIX_GET_I.4, G.REQLIX_GET_I.5)
pub fn find_or_create_requirements_file(project_root: &str) -> Result<PathBuf, String> {
    // Search for existing file
    for path in get_search_paths(project_root) {
        if path.exists() {
            return Ok(path);
        }
    }

    // Create new file with placeholder content
    let create_path = get_create_path(project_root);

    // Create parent directories (G.C.2)
    if let Some(parent) = create_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directories: {}", e))?;
    }

    // Replace {requirements_directory} placeholder (G.REQLIX_GET_I.6)
    let requirements_dir = create_path
        .parent()
        .and_then(|p| {
            p.strip_prefix(project_root)
                .ok()
                .map(|rel| rel.to_string_lossy().to_string())
        })
        .unwrap_or_default();
    let content = PLACEHOLDER_CONTENT.replace("{requirements_directory}", &requirements_dir);

    write_file_utf8(&create_path, &content)
        .map_err(|e| format!("Failed to create requirements file: {}", e))?;

    Ok(create_path)
}

/// Get requirements directory (G.C.1)
pub fn get_requirements_dir(project_root: &str) -> Result<PathBuf, String> {
    let agents_path = find_or_create_requirements_file(project_root)?;
    agents_path
        .parent()
        .map(|p| p.to_path_buf())
        .ok_or_else(|| "Could not determine requirements directory".to_string())
}
