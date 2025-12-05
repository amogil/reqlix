// Unit tests for markdown parsing functions
// Covers Requirement: G.R.2, G.R.3

use reqlix::RequirementsServer;

/// Test: parse_level1_heading with valid heading
/// Precondition: System has a valid markdown level-1 heading line
/// Action: Call parse_level1_heading with "# Chapter Name"
/// Result: Function returns Some("Chapter Name")
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_valid() {
    let result = RequirementsServer::parse_level1_heading("# Chapter Name");
    assert_eq!(result, Some("Chapter Name".to_string()));
}

/// Test: parse_level1_heading with indented heading (1 space)
/// Precondition: System has a markdown level-1 heading with 1 space indentation
/// Action: Call parse_level1_heading with " # Chapter Name"
/// Result: Function returns Some("Chapter Name")
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_one_space_indent() {
    let result = RequirementsServer::parse_level1_heading(" # Chapter Name");
    assert_eq!(result, Some("Chapter Name".to_string()));
}

/// Test: parse_level1_heading with indented heading (3 spaces)
/// Precondition: System has a markdown level-1 heading with 3 spaces indentation
/// Action: Call parse_level1_heading with "   # Chapter Name"
/// Result: Function returns Some("Chapter Name")
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_three_space_indent() {
    let result = RequirementsServer::parse_level1_heading("   # Chapter Name");
    assert_eq!(result, Some("Chapter Name".to_string()));
}

/// Test: parse_level1_heading with level-2 heading
/// Precondition: System has a markdown level-2 heading
/// Action: Call parse_level1_heading with "## Requirement"
/// Result: Function returns None
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_level2() {
    let result = RequirementsServer::parse_level1_heading("## Requirement");
    assert_eq!(result, None);
}

/// Test: parse_level1_heading with invalid format (no space after #)
/// Precondition: System has an invalid markdown heading without space after #
/// Action: Call parse_level1_heading with "#Chapter Name"
/// Result: Function returns None
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_no_space() {
    let result = RequirementsServer::parse_level1_heading("#Chapter Name");
    assert_eq!(result, None);
}

/// Test: parse_level2_heading with valid heading
/// Precondition: System has a valid markdown level-2 requirement heading
/// Action: Call parse_level2_heading with "## G.G.1: Requirement Title"
/// Result: Function returns Some(("G.G.1", "Requirement Title"))
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_valid() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1: Requirement Title");
    assert_eq!(result, Some(("G.G.1".to_string(), "Requirement Title".to_string())));
}

/// Test: parse_level2_heading with indented heading
/// Precondition: System has a markdown level-2 heading with indentation
/// Action: Call parse_level2_heading with "  ## G.G.1: Requirement Title"
/// Result: Function returns Some(("G.G.1", "Requirement Title"))
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_indented() {
    let result = RequirementsServer::parse_level2_heading("  ## G.G.1: Requirement Title");
    assert_eq!(result, Some(("G.G.1".to_string(), "Requirement Title".to_string())));
}

/// Test: parse_level2_heading with level-3 heading
/// Precondition: System has a markdown level-3 heading
/// Action: Call parse_level2_heading with "### Subsection"
/// Result: Function returns None
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_level3() {
    let result = RequirementsServer::parse_level2_heading("### Subsection");
    assert_eq!(result, None);
}

/// Test: parse_level2_heading with missing colon
/// Precondition: System has a markdown level-2 heading without colon separator
/// Action: Call parse_level2_heading with "## G.G.1 Requirement Title"
/// Result: Function returns None
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_no_colon() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1 Requirement Title");
    assert_eq!(result, None);
}

/// Test: parse_level2_heading with empty index
/// Precondition: System has a markdown level-2 heading with empty index
/// Action: Call parse_level2_heading with "## : Requirement Title"
/// Result: Function returns None
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_empty_index() {
    let result = RequirementsServer::parse_level2_heading("## : Requirement Title");
    assert_eq!(result, None);
}

/// Test: parse_level1_heading with trailing spaces
/// Precondition: System has a markdown level-1 heading with trailing spaces
/// Action: Call parse_level1_heading with "# Chapter Name   "
/// Result: Function returns Some("Chapter Name") (trailing spaces trimmed)
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_trailing_spaces() {
    let result = RequirementsServer::parse_level1_heading("# Chapter Name   ");
    assert_eq!(result, Some("Chapter Name".to_string()));
}

/// Test: parse_level1_heading with 4 spaces indentation (too many)
/// Precondition: System has a markdown level-1 heading with 4 spaces indentation
/// Action: Call parse_level1_heading with "    # Chapter Name"
/// Result: Function returns None (4 spaces is too many)
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_four_space_indent() {
    let result = RequirementsServer::parse_level1_heading("    # Chapter Name");
    assert_eq!(result, None);
}

/// Test: parse_level1_heading with trailing hash characters
/// Precondition: System has a markdown level-1 heading with trailing hashes
/// Action: Call parse_level1_heading with "# Chapter Name ###"
/// Result: Function returns Some("Chapter Name") (trailing hashes removed per standard markdown)
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_trailing_hashes() {
    let result = RequirementsServer::parse_level1_heading("# Chapter Name ###");
    assert_eq!(result, Some("Chapter Name".to_string()));
}

/// Test: parse_level1_heading with empty chapter name
/// Precondition: System has a markdown level-1 heading with empty name
/// Action: Call parse_level1_heading with "# "
/// Result: Function returns Some("") (empty string)
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_empty_name() {
    let result = RequirementsServer::parse_level1_heading("# ");
    assert_eq!(result, Some("".to_string()));
}

/// Test: parse_level2_heading with empty title
/// Precondition: System has a markdown level-2 heading with empty title
/// Action: Call parse_level2_heading with "## G.G.1: "
/// Result: Function returns None (empty title is invalid)
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_empty_title() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1: ");
    assert_eq!(result, None);
}

/// Test: parse_level2_heading with spaces around colon
/// Precondition: System has a markdown level-2 heading with spaces around colon
/// Action: Call parse_level2_heading with "## G.G.1 : Requirement Title"
/// Result: Function returns Some(("G.G.1", "Requirement Title")) (spaces trimmed)
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_spaces_around_colon() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1 : Requirement Title");
    assert_eq!(result, Some(("G.G.1".to_string(), "Requirement Title".to_string())));
}

/// Test: parse_level2_heading with trailing hash characters
/// Precondition: System has a markdown level-2 heading with trailing hashes
/// Action: Call parse_level2_heading with "## G.G.1: Title ###"
/// Result: Function returns Some(("G.G.1", "Title")) (trailing hashes removed per standard markdown)
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_trailing_hashes() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1: Title ###");
    assert_eq!(result, Some(("G.G.1".to_string(), "Title".to_string())));
}

/// Test: parse_level2_heading with complex index format
/// Precondition: System has a markdown level-2 heading with complex index
/// Action: Call parse_level2_heading with "## GET.GET_C.123: Complex Requirement Title"
/// Result: Function returns Some(("GET.GET_C.123", "Complex Requirement Title"))
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_complex_index() {
    let result = RequirementsServer::parse_level2_heading("## GET.GET_C.123: Complex Requirement Title");
    assert_eq!(result, Some(("GET.GET_C.123".to_string(), "Complex Requirement Title".to_string())));
}

/// Test: parse_level2_heading with title containing colon
/// Precondition: System has a markdown level-2 heading with colon in title
/// Action: Call parse_level2_heading with "## G.G.1: Title: Subtitle"
/// Result: Function returns Some(("G.G.1", "Title: Subtitle")) (first colon is separator)
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_colon_in_title() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1: Title: Subtitle");
    assert_eq!(result, Some(("G.G.1".to_string(), "Title: Subtitle".to_string())));
}

/// Test: parse_level1_heading with only hash and space
/// Precondition: System has a markdown level-1 heading with only "# "
/// Action: Call parse_level1_heading with "# "
/// Result: Function returns Some("") (empty name)
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_only_hash_space() {
    let result = RequirementsServer::parse_level1_heading("# ");
    assert_eq!(result, Some("".to_string()));
}

/// Test: parse_level1_heading with tab character
/// Precondition: System has a markdown level-1 heading with tab character
/// Action: Call parse_level1_heading with "\t# Chapter Name"
/// Result: Function returns None (tabs are not spaces)
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_tab() {
    let result = RequirementsServer::parse_level1_heading("\t# Chapter Name");
    assert_eq!(result, None);
}

/// Test: parse_level2_heading with tab character
/// Precondition: System has a markdown level-2 heading with tab character
/// Action: Call parse_level2_heading with "\t## G.G.1: Title"
/// Result: Function returns None (tabs are not spaces)
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_tab() {
    let result = RequirementsServer::parse_level2_heading("\t## G.G.1: Title");
    assert_eq!(result, None);
}

/// Test: parse_level1_heading with unicode characters
/// Precondition: System has a markdown level-1 heading with unicode characters
/// Action: Call parse_level1_heading with "# Глава"
/// Result: Function returns Some("Глава")
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_unicode() {
    let result = RequirementsServer::parse_level1_heading("# Глава");
    assert_eq!(result, Some("Глава".to_string()));
}

/// Test: parse_level2_heading with unicode characters
/// Precondition: System has a markdown level-2 heading with unicode characters
/// Action: Call parse_level2_heading with "## G.G.1: Требование"
/// Result: Function returns Some(("G.G.1", "Требование"))
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_unicode() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1: Требование");
    assert_eq!(result, Some(("G.G.1".to_string(), "Требование".to_string())));
}

/// Test: parse_level1_heading with special characters
/// Precondition: System has a markdown level-1 heading with special characters
/// Action: Call parse_level1_heading with "# Chapter-Name_123"
/// Result: Function returns Some("Chapter-Name_123")
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_special_chars() {
    let result = RequirementsServer::parse_level1_heading("# Chapter-Name_123");
    assert_eq!(result, Some("Chapter-Name_123".to_string()));
}

/// Test: parse_level2_heading with special characters in title
/// Precondition: System has a markdown level-2 heading with special characters in title
/// Action: Call parse_level2_heading with "## G.G.1: Title-Name_123"
/// Result: Function returns Some(("G.G.1", "Title-Name_123"))
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_special_chars_title() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1: Title-Name_123");
    assert_eq!(result, Some(("G.G.1".to_string(), "Title-Name_123".to_string())));
}

/// Test: parse_level2_heading with multiple spaces in title
/// Precondition: System has a markdown level-2 heading with multiple spaces in title
/// Action: Call parse_level2_heading with "## G.G.1: Title   With   Spaces"
/// Result: Function returns Some(("G.G.1", "Title   With   Spaces")) (spaces preserved)
/// Covers Requirement: G.R.3
#[test]
fn test_parse_level2_heading_multiple_spaces() {
    let result = RequirementsServer::parse_level2_heading("## G.G.1: Title   With   Spaces");
    assert_eq!(result, Some(("G.G.1".to_string(), "Title   With   Spaces".to_string())));
}

/// Test: parse_level1_heading with multiple spaces in name
/// Precondition: System has a markdown level-1 heading with multiple spaces in name
/// Action: Call parse_level1_heading with "# Chapter   Name"
/// Result: Function returns Some("Chapter   Name") (spaces preserved)
/// Covers Requirement: G.R.2
#[test]
fn test_parse_level1_heading_multiple_spaces() {
    let result = RequirementsServer::parse_level1_heading("# Chapter   Name");
    assert_eq!(result, Some("Chapter   Name".to_string()));
}
