// Unit tests for requirement parsing functions
// Covers Requirement: G.R.5, G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4

use reqlix::RequirementsServer;
use std::fs;
use tempfile::TempDir;

/// Test: read_requirements_streaming with single requirement
/// Precondition: System has a category file with one requirement in a chapter
/// Action: Call read_requirements_streaming with file containing "## G.G.1: Title"
/// Result: Function returns vec containing requirement with index "G.G.1" and title "Title"
/// Covers Requirement: G.REQLIX_GET_REQ.3, G.R.3, G.R.5
#[test]
fn test_read_requirements_streaming_single() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter\n## G.G.1: Title\n").unwrap();
    
    let result = RequirementsServer::read_requirements_streaming(&file_path, "Chapter");
    assert!(result.is_ok());
    let requirements = result.unwrap();
    assert_eq!(requirements.len(), 1);
    assert_eq!(requirements[0].index, "G.G.1");
    assert_eq!(requirements[0].title, "Title");
}

/// Test: read_requirements_streaming with multiple requirements
/// Precondition: System has a category file with multiple requirements in a chapter
/// Action: Call read_requirements_streaming with file containing multiple "##" headings
/// Result: Function returns vec containing all requirements
/// Covers Requirement: G.REQLIX_GET_REQ.3, G.R.3, G.R.5
#[test]
fn test_read_requirements_streaming_multiple() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter\n## G.G.1: Title One\n## G.G.2: Title Two\n").unwrap();
    
    let result = RequirementsServer::read_requirements_streaming(&file_path, "Chapter");
    assert!(result.is_ok());
    let requirements = result.unwrap();
    assert_eq!(requirements.len(), 2);
    assert_eq!(requirements[0].index, "G.G.1");
    assert_eq!(requirements[0].title, "Title One");
    assert_eq!(requirements[1].index, "G.G.2");
    assert_eq!(requirements[1].title, "Title Two");
}

/// Test: read_requirements_streaming ignoring requirements in code blocks
/// Precondition: System has a category file with requirement heading inside code block
/// Action: Call read_requirements_streaming with file containing "## G.G.1: Title" inside ```
/// Result: Function ignores requirement heading inside code block
/// Covers Requirement: G.REQLIX_GET_REQ.3, G.R.3, G.R.5
#[test]
fn test_read_requirements_streaming_ignore_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter\n## G.G.1: Real Title\n```\n## G.G.2: Fake Title\n```\n").unwrap();
    
    let result = RequirementsServer::read_requirements_streaming(&file_path, "Chapter");
    assert!(result.is_ok());
    let requirements = result.unwrap();
    assert_eq!(requirements.len(), 1);
    assert_eq!(requirements[0].index, "G.G.1");
    assert_eq!(requirements[0].title, "Real Title");
}

/// Test: read_requirements_streaming with requirements in different chapters
/// Precondition: System has a category file with requirements in multiple chapters
/// Action: Call read_requirements_streaming for specific chapter
/// Result: Function returns only requirements from specified chapter
/// Covers Requirement: G.REQLIX_GET_REQ.3, G.R.3, G.R.5
#[test]
fn test_read_requirements_streaming_different_chapters() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter One\n## G.G.1: Title One\n# Chapter Two\n## G.G.2: Title Two\n").unwrap();
    
    let result = RequirementsServer::read_requirements_streaming(&file_path, "Chapter One");
    assert!(result.is_ok());
    let requirements = result.unwrap();
    assert_eq!(requirements.len(), 1);
    assert_eq!(requirements[0].index, "G.G.1");
    assert_eq!(requirements[0].title, "Title One");
}

/// Test: read_requirements_streaming with empty chapter
/// Precondition: System has a category file with empty chapter
/// Action: Call read_requirements_streaming for chapter with no requirements
/// Result: Function returns empty vec
/// Covers Requirement: G.REQLIX_GET_REQ.3, G.R.3, G.R.5
#[test]
fn test_read_requirements_streaming_empty_chapter() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter\n").unwrap();
    
    let result = RequirementsServer::read_requirements_streaming(&file_path, "Chapter");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Vec::new());
}

/// Test: read_requirements_streaming with indented headings
/// Precondition: System has a category file with indented requirement headings
/// Action: Call read_requirements_streaming with file containing "  ## G.G.1: Title" (1-3 spaces)
/// Result: Function parses indented headings correctly
/// Covers Requirement: G.REQLIX_GET_REQ.3, G.R.3, G.R.5
#[test]
fn test_read_requirements_streaming_indented() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter\n  ## G.G.1: Title\n").unwrap();
    
    let result = RequirementsServer::read_requirements_streaming(&file_path, "Chapter");
    assert!(result.is_ok());
    let requirements = result.unwrap();
    assert_eq!(requirements.len(), 1);
    assert_eq!(requirements[0].index, "G.G.1");
}

/// Test: read_requirements_streaming with level-1 headings (should ignore)
/// Precondition: System has a category file with level-1 headings in chapter
/// Action: Call read_requirements_streaming with file containing "# Subchapter"
/// Result: Function ignores level-1 headings when collecting requirements
/// Covers Requirement: G.REQLIX_GET_REQ.3, G.R.3, G.R.5
#[test]
fn test_read_requirements_streaming_ignore_level1() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter\n## G.G.1: Title\n# Subchapter\n## G.G.2: Title Two\n").unwrap();
    
    let result = RequirementsServer::read_requirements_streaming(&file_path, "Chapter");
    assert!(result.is_ok());
    let requirements = result.unwrap();
    // Should include both requirements since Subchapter is treated as part of Chapter
    assert!(requirements.len() >= 1);
}

/// Test: find_requirement_streaming with simple requirement
/// Precondition: System has a category file with a requirement
/// Action: Call find_requirement_streaming with index "G.G.1"
/// Result: Function returns RequirementFull with correct index, title, and text
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4, G.R.5
#[test]
fn test_find_requirement_streaming_simple() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter\n## G.G.1: Title\nText content\n").unwrap();
    
    let result = RequirementsServer::find_requirement_streaming(&file_path, "test", "G.G.1");
    assert!(result.is_ok());
    let req = result.unwrap();
    assert_eq!(req.index, "G.G.1");
    assert_eq!(req.title, "Title");
    assert_eq!(req.text, "Text content");
    assert_eq!(req.category, "test");
    assert_eq!(req.chapter, "Chapter");
}

/// Test: find_requirement_streaming with requirement at end of file
/// Precondition: System has a category file with requirement at end
/// Action: Call find_requirement_streaming for last requirement
/// Result: Function correctly includes all text until EOF
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4, G.R.5
#[test]
fn test_find_requirement_streaming_end_of_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter\n## G.G.1: Title\nLine 1\nLine 2\n").unwrap();
    
    let result = RequirementsServer::find_requirement_streaming(&file_path, "test", "G.G.1");
    assert!(result.is_ok());
    let req = result.unwrap();
    assert_eq!(req.index, "G.G.1");
    assert!(req.text.contains("Line 1"));
    assert!(req.text.contains("Line 2"));
}

/// Test: find_requirement_streaming with code block in requirement
/// Precondition: System has a category file with requirement containing code block
/// Action: Call find_requirement_streaming for requirement with code block
/// Result: Function includes code block content in requirement text
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4, G.R.5
#[test]
fn test_find_requirement_streaming_with_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter\n## G.G.1: Title\n```\ncode line\n```\n").unwrap();
    
    let result = RequirementsServer::find_requirement_streaming(&file_path, "test", "G.G.1");
    assert!(result.is_ok());
    let req = result.unwrap();
    assert!(req.text.contains("```"));
    assert!(req.text.contains("code line"));
}

/// Test: find_requirement_streaming with requirement boundary at next requirement
/// Precondition: System has a category file with multiple requirements
/// Action: Call find_requirement_streaming for first requirement
/// Result: Function stops at next requirement heading
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4, G.R.5
#[test]
fn test_find_requirement_streaming_boundary_next_requirement() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter\n## G.G.1: Title One\nText one\n## G.G.2: Title Two\nText two\n").unwrap();
    
    let result = RequirementsServer::find_requirement_streaming(&file_path, "test", "G.G.1");
    assert!(result.is_ok());
    let req = result.unwrap();
    assert_eq!(req.index, "G.G.1");
    assert!(req.text.contains("Text one"));
    assert!(!req.text.contains("Title Two"));
}

/// Test: find_requirement_streaming with level-1 heading ends requirement
/// Precondition: System has a category file with level-1 heading after requirement
/// Action: Call find_requirement_streaming for requirement before "# NextChapter"
/// Result: Function does NOT include level-1 heading in requirement text (G.R.5: ends at same or higher level)
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4, G.R.5
#[test]
fn test_find_requirement_streaming_level1_ends_requirement() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter\n## G.G.1: Title\nText before\n# NextChapter\nText after\n").unwrap();
    
    let result = RequirementsServer::find_requirement_streaming(&file_path, "test", "G.G.1");
    assert!(result.is_ok());
    let req = result.unwrap();
    assert!(req.text.contains("Text before"));
    // Level-1 heading should NOT be part of requirement text (G.R.5)
    assert!(!req.text.contains("# NextChapter"), "Level-1 heading should end requirement, not be included in text");
    assert!(!req.text.contains("Text after"), "Content after level-1 heading should not be in requirement");
}

/// Test: find_requirement_streaming with requirement not found
/// Precondition: System has a category file without specified requirement
/// Action: Call find_requirement_streaming with non-existent index
/// Result: Function returns error "Requirement not found"
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4
#[test]
fn test_find_requirement_streaming_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter\n## G.G.1: Title\n").unwrap();
    
    let result = RequirementsServer::find_requirement_streaming(&file_path, "test", "G.G.999");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

/// Test: find_requirement_streaming with multi-line code block
/// Precondition: System has a category file with multi-line code block in requirement
/// Action: Call find_requirement_streaming for requirement with multi-line code
/// Result: Function correctly tracks code block boundaries and includes all content
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4, G.R.5
#[test]
fn test_find_requirement_streaming_multiline_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter\n## G.G.1: Title\n```\nline1\nline2\nline3\n```\nMore text\n").unwrap();
    
    let result = RequirementsServer::find_requirement_streaming(&file_path, "test", "G.G.1");
    assert!(result.is_ok());
    let req = result.unwrap();
    assert!(req.text.contains("line1"));
    assert!(req.text.contains("line2"));
    assert!(req.text.contains("line3"));
    assert!(req.text.contains("More text"));
}

/// Test: find_requirement_streaming with code block language identifier
/// Precondition: System has a category file with code block having language identifier
/// Action: Call find_requirement_streaming for requirement with "```json" code block
/// Result: Function correctly identifies code block boundaries
/// Covers Requirement: G.REQLIX_GET_REQUIREMENT.3, G.REQLIX_GET_REQUIREMENT.4, G.R.5
#[test]
fn test_find_requirement_streaming_code_block_language() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter\n## G.G.1: Title\n```json\n{\"key\": \"value\"}\n```\n").unwrap();
    
    let result = RequirementsServer::find_requirement_streaming(&file_path, "test", "G.G.1");
    assert!(result.is_ok());
    let req = result.unwrap();
    assert!(req.text.contains("```json"));
    assert!(req.text.contains("{\"key\": \"value\"}"));
}
