// Parameter validation (G.P.2, G.P.3)

use crate::constants::*;
use crate::params::KeywordsParam;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};

/// Validate project_root parameter (G.P.1, G.P.2)
#[cfg_attr(test, allow(dead_code))]
pub fn validate_project_root(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err("project_root is required".to_string());
    }
    if value.len() > MAX_PROJECT_ROOT_LEN {
        return Err(format!(
            "project_root exceeds maximum length of {} characters",
            MAX_PROJECT_ROOT_LEN
        ));
    }
    Ok(())
}

/// Validate operation_description parameter (G.P.1, G.P.2)
#[cfg_attr(test, allow(dead_code))]
pub fn validate_operation_description(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err("operation_description is required".to_string());
    }
    if value.len() > MAX_OPERATION_DESC_LEN {
        return Err(format!(
            "operation_description exceeds maximum length of {} characters",
            MAX_OPERATION_DESC_LEN
        ));
    }
    Ok(())
}

/// Validate category parameter (G.P.1, G.P.3)
#[cfg_attr(test, allow(dead_code))]
pub fn validate_category(value: &str) -> Result<(), String> {
    // Basic constraints (G.P.1)
    if value.is_empty() {
        return Err("category is required".to_string());
    }
    if value.len() > MAX_CATEGORY_LEN {
        return Err(format!(
            "category exceeds maximum length of {} characters",
            MAX_CATEGORY_LEN
        ));
    }

    // Name validation (G.P.3)
    // Must not start or end with whitespace
    if value.trim() != value {
        return Err("category name must not start or end with whitespace".to_string());
    }

    // Must contain only lowercase English letters (a-z) and underscore (_)
    if !value.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
        return Err(
            "category name must contain only lowercase English letters (a-z) and underscore (_)"
                .to_string(),
        );
    }

    // Must be a valid filename (cannot contain invalid characters)
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    if let Some(ch) = value.chars().find(|c| invalid_chars.contains(c)) {
        return Err(format!(
            "category name contains invalid character: '{}' (invalid for filename)",
            ch
        ));
    }

    // Must not be reserved name
    if value == "AGENTS" {
        return Err("category name 'AGENTS' is reserved".to_string());
    }

    // Must not contain consecutive dots
    if value.contains("..") {
        return Err("category name must not contain consecutive dots".to_string());
    }

    // Must not be . or ..
    if value == "." || value == ".." {
        return Err("category name must not be '.' or '..'".to_string());
    }

    Ok(())
}

/// Validate chapter parameter (G.P.1, G.P.3)
#[cfg_attr(test, allow(dead_code))]
pub fn validate_chapter(value: &str) -> Result<(), String> {
    // Basic constraints (G.P.1)
    if value.is_empty() {
        return Err("chapter is required".to_string());
    }
    if value.len() > MAX_CHAPTER_LEN {
        return Err(format!(
            "chapter exceeds maximum length of {} characters",
            MAX_CHAPTER_LEN
        ));
    }

    // Name validation (G.P.3)
    // Must not start or end with whitespace
    if value.trim() != value {
        return Err("chapter name must not start or end with whitespace".to_string());
    }

    // Must contain only uppercase and lowercase English letters (A-Z, a-z), spaces, colons (:), hyphens (-), and underscores (_) - G.P.3
    if !value
        .chars()
        .all(|c| c.is_ascii_alphabetic() || c == ' ' || c == ':' || c == '-' || c == '_')
    {
        return Err("chapter name must contain only uppercase and lowercase English letters (A-Z, a-z), spaces, colons (:), hyphens (-), and underscores (_)".to_string());
    }

    // Must not contain newline characters (would break markdown heading structure)
    if value.contains('\n') || value.contains('\r') {
        return Err(
            "chapter name must not contain newline characters (invalid for markdown heading)"
                .to_string(),
        );
    }

    // Must be valid markdown heading content
    // Verify by parsing a test heading
    let test_heading = format!("# {}", value);
    let parser = Parser::new(&test_heading);
    let events: Vec<Event> = parser.collect();

    // Check if we can parse it as a valid level-1 heading
    if events.len() < 2 {
        return Err("chapter name is not valid markdown heading content".to_string());
    }

    // Verify it's a level-1 heading with text content
    match (&events[0], &events[1]) {
        (Event::Start(Tag::Heading(level, _, _)), Event::Text(_)) => {
            if level != &HeadingLevel::H1 {
                return Err("chapter name is not valid markdown heading content".to_string());
            }
        }
        _ => {
            return Err("chapter name is not valid markdown heading content".to_string());
        }
    }

    Ok(())
}

/// Validate index parameter (G.P.1, G.P.2)
#[cfg_attr(test, allow(dead_code))]
pub fn validate_index(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err("index is required".to_string());
    }
    if value.len() > MAX_INDEX_LEN {
        return Err(format!(
            "index exceeds maximum length of {} characters",
            MAX_INDEX_LEN
        ));
    }
    Ok(())
}

/// Validate text parameter (G.P.1, G.P.2)
#[cfg_attr(test, allow(dead_code))]
pub fn validate_text(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err("text is required".to_string());
    }
    if value.len() > MAX_TEXT_LEN {
        return Err(format!(
            "text exceeds maximum length of {} characters",
            MAX_TEXT_LEN
        ));
    }
    Ok(())
}

/// Validate title parameter (G.P.1, G.P.2, G.P.3)
#[cfg_attr(test, allow(dead_code))]
pub fn validate_title(value: &str, required: bool) -> Result<(), String> {
    if required && value.is_empty() {
        return Err("title is required".to_string());
    }
    if value.len() > MAX_TITLE_LEN {
        return Err(format!(
            "title exceeds maximum length of {} characters",
            MAX_TITLE_LEN
        ));
    }

    // Validate that title is valid markdown heading content (G.P.3, G.R.3)
    // Title will be used in a level-2 ATX-style heading: ## {index}: {title}
    // A valid heading content must not contain newlines (which would break the heading structure)
    if !value.is_empty() {
        if value.contains('\n') || value.contains('\r') {
            return Err(
                "title must not contain newlines (invalid for markdown heading)".to_string(),
            );
        }

        // Verify that the title can be used in a markdown heading by parsing a test heading
        let test_heading = format!("## G.G.1: {}", value);
        let parser = Parser::new(&test_heading);
        let events: Vec<Event> = parser.collect();

        // Check if we can parse it as a valid level-2 heading
        if events.len() < 2 {
            return Err("title is not valid markdown heading content".to_string());
        }

        // Verify it's a level-2 heading with text content
        match (&events[0], &events[1]) {
            (Event::Start(Tag::Heading(level, _, _)), Event::Text(_)) => {
                if level != &HeadingLevel::H2 {
                    return Err("title is not valid markdown heading content".to_string());
                }
            }
            _ => {
                return Err("title is not valid markdown heading content".to_string());
            }
        }
    }

    Ok(())
}

/// Validate keywords parameter (G.TOOLREQLIXS.5, G.TOOLREQLIXS.6)
/// Returns filtered non-empty keywords or error
#[cfg_attr(test, allow(dead_code))]
pub fn validate_keywords(keywords: &KeywordsParam) -> Result<Vec<String>, String> {
    let keywords_vec = match keywords {
        KeywordsParam::Single(s) => vec![s.clone()],
        KeywordsParam::Batch(v) => v.clone(),
    };

    // G.TOOLREQLIXS.5: Maximum 100 keywords
    if keywords_vec.len() > MAX_BATCH_SIZE {
        return Err("Keywords count exceeds maximum limit of 100".to_string());
    }

    // G.TOOLREQLIXS.5: Validate each keyword length and filter empty strings
    let mut filtered: Vec<String> = Vec::new();
    for keyword in keywords_vec {
        if keyword.len() > MAX_KEYWORD_LEN {
            return Err(format!(
                "Keyword exceeds maximum length of {} characters",
                MAX_KEYWORD_LEN
            ));
        }
        // Filter out empty strings (G.TOOLREQLIXS.5)
        if !keyword.is_empty() {
            filtered.push(keyword);
        }
    }

    Ok(filtered)
}
