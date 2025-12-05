// Unit tests for chapter parsing functions
// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5

use reqlix::RequirementsServer;
use std::fs;
use tempfile::TempDir;

/// Test: read_chapters_streaming with single chapter
/// Precondition: System has a category file with one chapter heading
/// Action: Call read_chapters_streaming with file containing "# Chapter One"
/// Result: Function returns vec containing "Chapter One"
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_single() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter One\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec!["Chapter One"]);
}

/// Test: read_chapters_streaming with multiple chapters
/// Precondition: System has a category file with multiple chapter headings
/// Action: Call read_chapters_streaming with file containing multiple "# Chapter" headings
/// Result: Function returns vec containing all chapter names
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_multiple() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter One\n\nSome text\n\n# Chapter Two\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 2);
    assert_eq!(chapters[0], "Chapter One");
    assert_eq!(chapters[1], "Chapter Two");
}

/// Test: read_chapters_streaming ignoring headings in code blocks
/// Precondition: System has a category file with chapter heading inside code block
/// Action: Call read_chapters_streaming with file containing "# Chapter" inside ```
/// Result: Function ignores heading inside code block
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n```\n# Fake Chapter\n```\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming with indented headings
/// Precondition: System has a category file with indented chapter headings
/// Action: Call read_chapters_streaming with file containing " # Chapter" (1-3 spaces)
/// Result: Function parses indented headings correctly
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_indented() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, " # Chapter One\n   # Chapter Two\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 2);
}

/// Test: read_chapters_streaming with empty file
/// Precondition: System has an empty category file
/// Action: Call read_chapters_streaming with empty file
/// Result: Function returns empty vec
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_empty() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Vec::<String>::new());
}

/// Test: read_chapters_streaming with level-2 headings (should ignore)
/// Precondition: System has a category file with level-2 headings
/// Action: Call read_chapters_streaming with file containing "## Requirement"
/// Result: Function ignores level-2 headings
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_ignore_level2() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter\n## Requirement\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Chapter");
}

/// Test: read_chapters_streaming with multi-line code block
/// Precondition: System has a category file with multi-line code block
/// Action: Call read_chapters_streaming with file containing code block spanning multiple lines
/// Result: Function correctly tracks code block boundaries
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_multiline_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n```\nline1\nline2\n# Fake Chapter\nline3\n```\n# Another Chapter\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 2);
    assert_eq!(chapters[0], "Real Chapter");
    assert_eq!(chapters[1], "Another Chapter");
}

/// Test: read_chapters_streaming with code block language identifier
/// Precondition: System has a category file with code block having language identifier
/// Action: Call read_chapters_streaming with file containing "```json" code block
/// Result: Function correctly identifies code block boundaries
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_code_block_language() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n```json\n# Fake Chapter\n```\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming with trailing spaces in chapter name
/// Precondition: System has a category file with chapter heading having trailing spaces
/// Action: Call read_chapters_streaming with file containing "# Chapter   "
/// Result: Function trims trailing spaces from chapter name
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_trailing_spaces() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter One   \n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Chapter One");
}

/// Test: read_chapters_streaming with unicode chapter names
/// Precondition: System has a category file with unicode chapter names
/// Action: Call read_chapters_streaming with file containing "# Глава"
/// Result: Function correctly parses unicode chapter names
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_unicode() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Глава\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Глава");
}

/// Test: read_chapters_streaming ignoring "## Categories" mention in code block
/// Precondition: System has a category file with "## Categories" mentioned in code block
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```\n## Categories\n```"
/// Result: Function ignores "## Categories" inside code block
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_categories_in_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n```\n## Categories\n```\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring "# Categories" mention in code block
/// Precondition: System has a category file with "# Categories" mentioned in code block
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```\n# Categories\n```"
/// Result: Function ignores "# Categories" inside code block
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_categories_level1_in_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n```\n# Categories\n```\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in requirement text
/// Precondition: System has a category file with chapter mention in requirement text (not code block)
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n## G.G.1: Title\nText with # Categories mention\n"
/// Result: Function ignores "# Categories" mention in requirement text (not a real heading)
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_ignore_chapter_mention_in_text() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n## G.G.1: Title\nText with # Categories mention\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring "## Categories" in requirement text
/// Precondition: System has a category file with "## Categories" in requirement text
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n## G.G.1: Title\nText with ## Categories\n"
/// Result: Function ignores "## Categories" in requirement text (not a real heading)
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_ignore_categories_level2_in_text() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n## G.G.1: Title\nText with ## Categories\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in json code block
/// Precondition: System has a category file with chapter mention in json code block
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```json\n{\"heading\": \"# Categories\"}\n```"
/// Result: Function ignores chapter mention inside json code block
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_chapter_in_json_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n```json\n{\"heading\": \"# Categories\"}\n```\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in markdown code block
/// Precondition: System has a category file with chapter mention in markdown code block
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```markdown\n# Categories\n```"
/// Result: Function ignores chapter mention inside markdown code block
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_chapter_in_markdown_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n```markdown\n# Categories\n```\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring multiple chapter mentions in code block
/// Precondition: System has a category file with multiple chapter mentions in code block
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```\n# Categories\n# Chapters\n# Chapter List\n```"
/// Result: Function ignores all chapter mentions inside code block
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_multiple_chapters_in_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n```\n# Categories\n# Chapters\n# Chapter List\n```\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in nested code block
/// Precondition: System has a category file with chapter mention in nested code block structure
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```\nouter\n```\n```\n# Categories\n```"
/// Result: Function correctly handles nested code block boundaries
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_chapter_in_nested_code_blocks() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n```\nouter\n```\n```\n# Categories\n```\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in requirement body text
/// Precondition: System has a category file with chapter mention in requirement body
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n## G.G.1: Title\nThis mentions # Categories in the text\n"
/// Result: Function ignores chapter mention in requirement body text
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_ignore_chapter_mention_in_requirement_body() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n## G.G.1: Title\nThis mentions # Categories in the text\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring "## Categories" in inline code
/// Precondition: System has a category file with "## Categories" in inline code (not fenced block)
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\nText with `## Categories` inline\n"
/// Result: Function ignores inline code (not fenced block, so not tracked)
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_ignore_chapter_in_inline_code() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\nText with `## Categories` inline\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming with real chapter after code block with mention
/// Precondition: System has a category file with real chapter after code block containing chapter mention
/// Action: Call read_chapters_streaming with file containing "# First\n```\n# Categories\n```\n# Second Chapter\n"
/// Result: Function correctly identifies real chapter after code block
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_real_chapter_after_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# First Chapter\n```\n# Categories\n```\n# Second Chapter\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 2);
    assert_eq!(chapters[0], "First Chapter");
    assert_eq!(chapters[1], "Second Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in requirement text with indentation
/// Precondition: System has a category file with indented chapter mention in requirement text
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n## G.G.1: Title\n  Text with # Categories\n"
/// Result: Function ignores indented chapter mention in requirement text
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_ignore_indented_chapter_mention() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n## G.G.1: Title\n  Text with # Categories\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in multi-line code block
/// Precondition: System has a category file with chapter mention in multi-line code block
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```\nline1\n# Categories\nline2\n```"
/// Result: Function ignores chapter mention in multi-line code block
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_chapter_in_multiline_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n```\nline1\n# Categories\nline2\n```\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring "## Categories" in code block with language
/// Precondition: System has a category file with "## Categories" in code block with language identifier
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```python\n## Categories\n```"
/// Result: Function ignores chapter mention in code block with language
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_categories_in_python_code_block() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n```python\n## Categories\n```\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in requirement text with special chars
/// Precondition: System has a category file with chapter mention containing special characters
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n## G.G.1: Title\nText: # Categories (list)\n"
/// Result: Function ignores chapter mention with special characters in requirement text
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_ignore_chapter_with_special_chars() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n## G.G.1: Title\nText: # Categories (list)\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in code block at file start
/// Precondition: System has a category file starting with code block containing chapter mention
/// Action: Call read_chapters_streaming with file containing "```\n# Categories\n```\n# Real Chapter\n"
/// Result: Function ignores chapter mention in code block at start
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_chapter_at_file_start_in_code() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "```\n# Categories\n```\n# Real Chapter\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming ignoring chapter mention in code block at file end
/// Precondition: System has a category file ending with code block containing chapter mention
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```\n# Categories\n```"
/// Result: Function ignores chapter mention in code block at end
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_chapter_at_file_end_in_code() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n```\n# Categories\n```").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming with "Categories" as real chapter
/// Precondition: System has a category file with actual chapter named "Categories"
/// Action: Call read_chapters_streaming with file containing "# Categories\n"
/// Result: Function returns "Categories" as a valid chapter
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_real_categories_chapter() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Categories\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Categories");
}

/// Test: read_chapters_streaming ignoring chapter mention in code block between real chapters
/// Precondition: System has a category file with chapter mention in code block between real chapters
/// Action: Call read_chapters_streaming with file containing "# First\n```\n# Categories\n```\n# Second\n"
/// Result: Function correctly identifies both real chapters, ignoring mention in code block
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_ignore_chapter_between_real_chapters() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# First Chapter\n```\n# Categories\n```\n# Second Chapter\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 2);
    assert_eq!(chapters[0], "First Chapter");
    assert_eq!(chapters[1], "Second Chapter");
}

/// Test: read_chapters_streaming ignoring "## Categories" in requirement text with formatting
/// Precondition: System has a category file with formatted "## Categories" mention in requirement text
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n## G.G.1: Title\nText with **## Categories** bold\n"
/// Result: Function ignores formatted chapter mention in requirement text
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_ignore_formatted_chapter_mention() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n## G.G.1: Title\nText with **## Categories** bold\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0], "Real Chapter");
}

/// Test: read_chapters_streaming with "Categories" and other chapters
/// Precondition: System has a category file with "# Categories" chapter heading
/// Action: Call read_chapters_streaming with file containing "# Categories\n# Real Chapter\n"
/// Result: Function returns both chapters including "Categories"
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_categories_with_other() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Categories\n# Real Chapter\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 2);
    assert_eq!(chapters[0], "Categories");
    assert_eq!(chapters[1], "Real Chapter");
}

/// Test: read_chapters_streaming with "Categories" among other chapters
/// Precondition: System has a category file with "Categories" and other chapters
/// Action: Call read_chapters_streaming with file containing "# Chapter One\n# Categories\n# Chapter Two\n"
/// Result: Function returns all chapters including "Categories"
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2
#[test]
fn test_read_chapters_streaming_categories_with_others() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Chapter One\n# Categories\n# Chapter Two\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 3);
    assert_eq!(chapters[0], "Chapter One");
    assert_eq!(chapters[1], "Categories");
    assert_eq!(chapters[2], "Chapter Two");
}


/// Test: read_chapters_streaming with "Categories" both in code block and as real heading
/// Precondition: System has a category file with "# Categories" in code block and real "# Categories" chapter
/// Action: Call read_chapters_streaming with file containing "# Real Chapter\n```\n# Categories\n```\n# Categories\n# Another Chapter\n"
/// Result: Function ignores "Categories" in code block and includes real "Categories" chapter
/// Covers Requirement: G.REQLIX_GET_CH.3, G.R.2, G.R.5
#[test]
fn test_read_chapters_streaming_categories_in_code_block_and_real() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Real Chapter\n```\n# Categories\n```\n# Categories\n# Another Chapter\n").unwrap();
    
    let result = RequirementsServer::read_chapters_streaming(&file_path);
    assert!(result.is_ok());
    let chapters = result.unwrap();
    assert_eq!(chapters.len(), 3);
    assert_eq!(chapters[0], "Real Chapter");
    assert_eq!(chapters[1], "Categories");
    assert_eq!(chapters[2], "Another Chapter");
}
