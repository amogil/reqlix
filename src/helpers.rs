// Category helpers (G.C.7, G.F.4) and insert/update helpers

use crate::parsing::{
    parse_level1_heading, parse_level2_heading, read_chapters_streaming,
    read_requirements_streaming,
};
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// List all category files (excluding AGENTS.md)
#[cfg_attr(test, allow(dead_code))]
pub fn list_categories(requirements_dir: &PathBuf) -> Result<Vec<String>, String> {
    let entries = read_dir(requirements_dir)
        .map_err(|e| format!("Failed to read requirements directory: {}", e))?;

    let mut categories: Vec<String> = entries
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if path.extension()? == "md" {
                let stem = path.file_stem()?.to_string_lossy().to_string();
                if stem != "AGENTS" {
                    return Some(stem);
                }
            }
            None
        })
        .collect();

    categories.sort();
    Ok(categories)
}

/// Calculate unique prefix for a name among a list of names (G.F.4)
#[cfg_attr(test, allow(dead_code))]
pub fn calculate_unique_prefix(name: &str, all_names: &[String]) -> String {
    // Extract only letters (A-Z, a-z) from the name (G.R.4)
    let letters: Vec<char> = name.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    if letters.is_empty() {
        return String::new();
    }

    let mut prefix_len = 1;

    loop {
        let prefix: String = letters
            .iter()
            .take(prefix_len)
            .collect::<String>()
            .to_uppercase();

        // Check if this prefix is unique
        let mut conflicts = 0;
        for other in all_names {
            if other == name {
                continue;
            }
            let other_letters: Vec<char> =
                other.chars().filter(|c| c.is_ascii_alphabetic()).collect();
            if other_letters.is_empty() {
                continue;
            }
            let other_prefix: String = other_letters
                .iter()
                .take(prefix_len)
                .collect::<String>()
                .to_uppercase();
            if other_prefix == prefix {
                conflicts += 1;
            }
        }

        if conflicts == 0 || prefix_len >= letters.len() {
            return prefix;
        }
        prefix_len += 1;
    }
}

/// Calculate unique prefix for chapter names (G.R.4)
/// Only considers letters (A-Z, a-z), ignoring spaces, colons, hyphens, etc.
#[cfg_attr(test, allow(dead_code))]
pub fn calculate_chapter_prefix(name: &str, all_names: &[String]) -> String {
    // Extract only letters (A-Z, a-z) from the name (G.R.4)
    let letters: Vec<char> = name.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    if letters.is_empty() {
        return String::new();
    }

    let mut prefix_len = 1;

    loop {
        let prefix: String = letters
            .iter()
            .take(prefix_len)
            .collect::<String>()
            .to_uppercase();

        // Check if this prefix is unique
        let mut conflicts = 0;
        for other in all_names {
            if other == name {
                continue;
            }
            let other_letters: Vec<char> =
                other.chars().filter(|c| c.is_ascii_alphabetic()).collect();
            if other_letters.is_empty() {
                continue;
            }
            let other_prefix: String = other_letters
                .iter()
                .take(prefix_len)
                .collect::<String>()
                .to_uppercase();
            if other_prefix == prefix {
                conflicts += 1;
            }
        }

        if conflicts == 0 || prefix_len >= letters.len() {
            return prefix;
        }
        prefix_len += 1;
    }
}

/// Find category by prefix (G.C.7)
#[cfg_attr(test, allow(dead_code))]
pub fn find_category_by_prefix(
    requirements_dir: &PathBuf,
    search_prefix: &str,
) -> Result<String, String> {
    let categories = list_categories(requirements_dir)?;

    for category in &categories {
        let prefix = calculate_unique_prefix(category, &categories);
        if prefix == search_prefix {
            return Ok(category.clone());
        }
    }

    Err("Category not found".to_string())
}

/// Get existing category prefix from requirements, or calculate new one
pub fn get_or_calculate_category_prefix(
    category_path: &PathBuf,
    category_name: &str,
    all_categories: &[String],
) -> Result<String, String> {
    // Try to find existing prefix from requirements in the file
    if category_path.exists() {
        let file = File::open(category_path)
            .map_err(|e| format!("Failed to open category file: {}", e))?;
        let reader = BufReader::new(file);
        let mut in_code_block = false;

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
            let trimmed = line.trim();

            // Track code block boundaries
            // Code blocks are fenced with triple backticks (```)
            if trimmed.starts_with("```") {
                in_code_block = !in_code_block;
                continue;
            }

            // Parse markdown level-2 heading (G.R.3)
            // Only parse headings that are NOT inside code blocks
            if !in_code_block {
                if let Some((index, _)) = parse_level2_heading(&line) {
                    let parts: Vec<&str> = index.split('.').collect();
                    if !parts.is_empty() {
                        return Ok(parts[0].to_string());
                    }
                }
            }
        }
    }

    // Calculate new prefix
    Ok(calculate_unique_prefix(category_name, all_categories))
}

/// Get existing chapter prefix from requirements, or calculate new one
pub fn get_or_calculate_chapter_prefix(
    category_path: &PathBuf,
    chapter_name: &str,
) -> Result<String, String> {
    let chapters = read_chapters_streaming(category_path)?;

    // Try to find existing prefix from requirements in this chapter
    if category_path.exists() {
        let file = File::open(category_path)
            .map_err(|e| format!("Failed to open category file: {}", e))?;
        let reader = BufReader::new(file);
        let mut in_target_chapter = false;
        let mut in_code_block = false;

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
            let trimmed = line.trim();

            // Track code block boundaries
            // Code blocks are fenced with triple backticks (```)
            if trimmed.starts_with("```") {
                in_code_block = !in_code_block;
                continue;
            }

            // Check for chapter heading (G.R.2)
            // Only parse headings that are NOT inside code blocks
            if !in_code_block {
                if let Some(ch_name) = parse_level1_heading(&line) {
                    in_target_chapter = ch_name == chapter_name;
                    continue;
                }
            }

            // Parse requirement heading in target chapter (G.R.3)
            // Only parse headings that are NOT inside code blocks
            if in_target_chapter && !in_code_block {
                if let Some((index, _)) = parse_level2_heading(&line) {
                    let parts: Vec<&str> = index.split('.').collect();
                    if parts.len() >= 2 {
                        return Ok(parts[1].to_string());
                    }
                }
            }
        }
    }

    // Calculate new prefix (G.R.4)
    Ok(calculate_chapter_prefix(chapter_name, &chapters))
}

/// Get next requirement number in a chapter
pub fn get_next_requirement_number(
    category_path: &PathBuf,
    chapter_name: &str,
) -> Result<u32, String> {
    let requirements = read_requirements_streaming(category_path, chapter_name)?;
    let mut max_num: u32 = 0;

    for req in &requirements {
        let parts: Vec<&str> = req.index.split('.').collect();
        if parts.len() == 3 {
            if let Ok(num) = parts[2].parse::<u32>() {
                max_num = max_num.max(num);
            }
        }
    }

    Ok(max_num + 1)
}

/// Check if title exists in chapter
pub fn title_exists_in_chapter(
    category_path: &PathBuf,
    chapter_name: &str,
    title: &str,
    exclude_index: Option<&str>,
) -> Result<bool, String> {
    let requirements = read_requirements_streaming(category_path, chapter_name)?;

    for req in &requirements {
        if let Some(exclude) = exclude_index {
            if req.index == exclude {
                continue;
            }
        }
        if req.title == title {
            return Ok(true);
        }
    }

    Ok(false)
}
