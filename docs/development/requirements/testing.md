
# Testing Requirements

## TE.T.1: Test file structure and organization

Test files must be organized according to requirement chapters. Each requirement chapter should have a corresponding
test file that covers all requirements in that chapter.

Test files are located in `tests/unit/` directory. For the complete mapping of requirement chapters to test files, see
TE.T.5. Common helper functions are located in `tests/unit/common/mod.rs` and must be used instead of duplicating helper
code across test files (see TE.T.6 for details).

## TE.T.2: Test comment format

Each test function must have a documentation comment (`///`) that follows this format:

```rust
/// Test: <brief description>
/// Precondition: <what state the system is in before the test>
/// Action: <what action is performed>
/// Result: <what result is expected>
/// Covers Requirement: <requirement_index>
#[test]
fn test_<function_name>_ < scenario>() {
// ...
}
```

The comment must include:

1. Brief description of what the test verifies
2. **Precondition** - What state the system is in before the test executes
3. **Action** - What action is performed during the test
4. **Result** - What result is expected from the test
5. **Covers Requirement** - List all requirement indices covered by this test

Example:

```rust
/// Test: validate_project_root with empty string
/// Precondition: System has no project_root value
/// Action: Call validate_project_root with empty string
/// Result: Function returns error "project_root is required"
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_project_root_empty() {
    let result = RequirementsServer::validate_project_root("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "project_root is required");
}
```

For tests covering multiple requirements, list all indices separated by commas in the "Covers Requirement" line.

## TE.T.3: Test file header comment

Each test file must start with a header comment that includes:

1. Brief description of what the file tests
2. List of all requirement indices covered by tests in this file

Format:

```rust
// Tests for <Chapter Name> (<Requirement Prefix>.*)
// Covers Requirements: <list of all requirement indices>
```

Example:

```rust
// Tests for Parameter Constraints (G.P.*)
// Covers Requirements: G.P.1, G.P.2, G.P.3, G.P.4
```

For tool-specific test files:

```rust
// Tests for Tool: <tool_name> (<Requirement Prefix>.*)
// Covers Requirements: <list of all requirement indices>
```

Example:

```rust
// Tests for Tool: reqlix_search_requirements (T.REQLIXS.*)
// Covers Requirements: T.REQLIXS.1, T.REQLIXS.2, T.REQLIXS.3, T.REQLIXS.4, T.REQLIXS.5, T.REQLIXS.6
```

## TE.T.4: Test grouping within files

Tests within a file must be grouped by requirement sections using section comments.

Format:

```rust
// =============================================================================
// Tests for <Requirement Index>: <Requirement Title>
// =============================================================================

// Optional: Subsection comment for specific function/feature
// Tests for <function_name> (<Requirement Index>)

/// Test: ...
/// Precondition: ...
/// Action: ...
/// Result: ...
/// Covers Requirement: ...
#[test]
fn test_...() {
// ...
}
```

Example:

```rust
// =============================================================================
// Tests for G.P.1, G.P.2: Parameter constraints and validation
// =============================================================================

// Tests for validate_project_root (G.P.1, G.P.2)

/// Test: validate_project_root with empty string
/// Precondition: System has no project_root value
/// Action: Call validate_project_root with empty string
/// Result: Function returns error "project_root is required"
/// Covers Requirement: G.P.1, G.P.2
#[test]
fn test_validate_project_root_empty() {
    // ...
}
```

This structure makes it easy to:

- Find tests for specific requirements
- Understand test coverage
- Navigate large test files

Note: All test comments must follow the format specified in TE.T.2 (including Precondition, Action, Result, and Covers
Requirement).

## TE.T.5: Requirement chapter to test file mapping

Each requirement chapter must have a corresponding test file. The mapping is as follows:

**General Requirements Chapters:**

- "Parameter Constraints" (G.P.*) → `parameter_constraints_tests.rs`
- "Requirements Storage Format" (G.R.*) → `requirements_storage_format_tests.rs`
- "Configuration" (C.C.*) → `configuration_tests.rs`

**Tool-Specific Chapters:**

- "Tool: reqlix_get_instructions" (T.R.*) → `tool_get_instructions_tests.rs`
- "Tool: reqlix_get_categories" (T.REQLIXGETC.*) → `tool_get_categories_tests.rs`
- "Tool: reqlix_get_chapters" (T.REQLIXGETCH.*) → `tool_get_chapters_tests.rs`
- "Tool: reqlix_get_requirements" (T.REQLIXGETR.*) → `tool_get_requirements_tests.rs`
- "Tool: reqlix_get_requirement" (T.REQLIXGETREQUIREMENT.*) → `tool_get_requirement_tests.rs`
- "Tool: reqlix_insert_requirement" (T.REQLIXI.*) → `tool_insert_requirement_tests.rs`
- "Tool: reqlix_update_requirement" (T.REQLIXU.*) → `tool_update_requirement_tests.rs`
- "Tool: reqlix_delete_requirement" (T.REQLIXD.*) → `tool_delete_requirement_tests.rs`
- "Tool: reqlix_search_requirements" (T.REQLIXS.*) → `tool_search_requirements_tests.rs`
- "Tool: reqlix_get_version" (T.REQLIXGETV.*) → `tool_get_version_tests.rs`

When adding new requirement chapters, create a corresponding test file following this naming convention.

## TE.T.6: Common helper functions usage

All test files must use common helper functions from `tests/unit/common/mod.rs` instead of duplicating helper code.

Available helper functions:

- `create_requirements_dir(temp_dir: &TempDir) -> PathBuf` - Creates requirements directory structure
- `create_category_file(temp_dir: &TempDir, category: &str, content: &str)` - Creates category file in temp directory
- `create_agents_file(temp_dir: &TempDir, content: &str)` - Creates AGENTS.md in temp directory
- `create_category_file_in_req_dir(req_dir: &Path, category: &str, content: &str)` - Creates category file in
  requirements directory
- `create_agents_file_in_req_dir(req_dir: &Path, content: &str)` - Creates AGENTS.md in requirements directory
- `parse_response(response: &str) -> Value` - Parses JSON response string

Import format:

```rust
use super::common::{
    create_agents_file_in_req_dir,
    create_category_file_in_req_dir,
    create_requirements_dir,
    parse_response,
};
```

If a new helper function is needed, add it to `tests/unit/common/mod.rs` instead of creating duplicate code in
individual test files.

## TE.T.7: Test coverage completeness

Each requirement must have at least one test that verifies its implementation.

Test coverage should include:

1. **Happy path** - Normal operation with valid inputs
2. **Error cases** - Invalid inputs, edge cases, boundary conditions
3. **Edge cases** - Empty values, maximum lengths, special characters
4. **Integration** - Interaction between multiple requirements

When adding a new requirement, immediately add corresponding tests to the appropriate test file.

When modifying existing requirements, update or add tests to ensure the modified behavior is verified.

Test names should follow the naming convention specified in TE.T.11 and clearly indicate what scenario is being tested (
e.g., `test_validate_category_empty`, `test_validate_category_too_long`).

## TE.T.8: Avoiding test duplication

Tests must not duplicate each other. Before adding a new test, check if similar functionality is already tested.

Guidelines:

1. **One requirement, multiple scenarios** - If a requirement has multiple scenarios (e.g., empty, valid, too long),
   create separate tests for each scenario, but group them together.

2. **Shared setup** - Use helper functions from `common/mod.rs` to avoid duplicating test setup code.

3. **Test consolidation** - If multiple tests verify the same behavior with different inputs, consider using
   parameterized tests or test cases within a single test function.

4. **Review existing tests** - Before adding a test, search for similar tests to avoid duplication.

If tests are similar but test different requirements, keep them separate but ensure they are clearly documented with
their requirement coverage.

## TE.T.9: Test file registration

All test files must be registered in `tests/unit.rs` using `#[path = "unit/<filename>"]` and `mod` declarations.

Test files are organized in sections:

1. **Common helper functions** - `mod common;`
2. **Tests grouped by requirement chapters** - Parameter constraints, Requirements storage format, Configuration
3. **Tool-specific tests** - One module per tool

Format:

```rust
// Common helper functions
#[path = "unit/common/mod.rs"]
mod common;

// Tests grouped by requirement chapters
#[path = "unit/parameter_constraints_tests.rs"]
mod parameter_constraints_tests;

// Tool-specific tests
#[path = "unit/tool_get_instructions_tests.rs"]
mod tool_get_instructions_tests;
```

When creating a new test file, add it to the appropriate section in `tests/unit.rs`.

## TE.T.10: Test execution before commit

Before committing test changes, ensure:

1. **Code formatting** - Run `cargo fmt` to ensure consistent formatting
2. **No warnings** - Run `cargo clippy -- -D warnings` to ensure no warnings
3. **No compilation errors** - Run `cargo test --no-run` to check compilation
4. **All tests pass** - Run `cargo test` and verify all tests pass
5. **Valid requirement references** - Verify that all requirement references in test comments (format: "Covers
   Requirement: G.X.Y") are:
    - Valid (refer to existing requirements)
    - Relevant (the test actually verifies the specified requirement)
    - Complete (all requirements covered by the test are listed)

Test files should follow the same code quality standards as production code:

- Clear naming conventions
- Proper error handling in test setup
- No unwrap() calls that could panic (use expect() with descriptive messages)
- Proper use of assertions with clear failure messages

When modifying tests, run the full test suite to ensure no regressions.

To verify requirement references, check that:

- Each requirement index in "Covers Requirement:" exists in the requirements system
- The test actually exercises the functionality described in the referenced requirement
- If a test covers multiple requirements, all are listed (not just one)
- Requirement indices follow the correct format: `{CATEGORY}.{CHAPTER}.{NUMBER}` (e.g., `G.P.1`, `T.REQLIXS.3`)

## TE.T.11: Test function naming convention

Test function names must follow the pattern: `test_<function_or_feature>_<scenario>()`

Naming guidelines:

1. **Prefix** - Always start with `test_`
2. **Function/Feature name** - Use the function or feature being tested (e.g., `validate_project_root`,
   `search_requirements`)
3. **Scenario** - Describe the specific scenario being tested (e.g., `empty`, `valid`, `too_long`, `invalid_char`)

Examples:

- `test_validate_project_root_empty()` - Tests validate_project_root with empty input
- `test_validate_category_too_long()` - Tests validate_category with value exceeding max length
- `test_search_finds_by_title()` - Tests search functionality finding by title
- `test_read_chapters_streaming_single()` - Tests read_chapters_streaming with single chapter

Test names should be:

- Descriptive and self-documenting
- Consistent with the naming pattern
- Clear about what scenario is being tested
- Not too long (prefer clarity over brevity)

Avoid:

- Generic names like `test_1()`, `test_basic()`
- Names that don't indicate the scenario (e.g., `test_validate()` instead of `test_validate_category_empty()`)
