// Parameter constraints (G.P.1) and placeholder content (T.R.6)

pub(crate) const MAX_PROJECT_ROOT_LEN: usize = 1000;
pub(crate) const MAX_OPERATION_DESC_LEN: usize = 10000;
pub(crate) const MAX_CATEGORY_LEN: usize = 100;
pub(crate) const MAX_CHAPTER_LEN: usize = 100;
pub(crate) const MAX_INDEX_LEN: usize = 100;
pub(crate) const MAX_TEXT_LEN: usize = 10000;
pub(crate) const MAX_TITLE_LEN: usize = 100;
// T.REQLIXGETREQUIREMENT.5, T.REQLIXU.6, T.REQLIXD.6, T.REQLIXS.5
pub(crate) const MAX_BATCH_SIZE: usize = 100;
// T.REQLIXS.5, G.P.1
pub(crate) const MAX_KEYWORD_LEN: usize = 200;

// T.R.6
pub(crate) const PLACEHOLDER_CONTENT: &str = r#"# Instructions

These instructions are mandatory for all code operations:

# General

1. All requirements must be written in English.
2. Never edit files in {requirements_directory} directly. Always use this MCP server for all
   requirements operations.
3. Requirement index format: `{CATEGORY}.{CHAPTER}.{NUMBER}` (e.g., `T.R.1`, `T.U.2`).
   Requirements are organized hierarchically: 
   **Category** groups related requirements together (e.g., general requirements, testing requirements).
   **Chapter** groups related requirements within a category (e.g., a specific tool or feature). 
   **Requirement** is a single, atomic requirement with a unique index.

# Finding Requirements

1. Use tools to find requirements by index, category, chapter, or keywords.
2. Make maximum effort to find all relevant requirements before making any code changes.

# Making Code Changes

Follow this mandatory step-by-step checklist for all code modifications. Inform the user about each step and its results:

1. **Find relevant requirements** - Use search tools to identify all requirements related to the code being modified.

2. **Propose requirement changes** - If requirements need updates, propose a plan to the user and obtain confirmation before making changes.

3. **Update requirements** - Make changes to requirements using the MCP server.

4. **Validate requirement changes** - Ensure changes are complete, consistent, and non-redundant. If issues are found, clarify with the user how to fix them.

5. **Confirm requirement changes** - Obtain user confirmation that requirement changes are correct.

6. **Find relevant code** - Identify all code that needs to be modified according to the requirements.

7. **Implement code changes** - Modify code to comply with requirements. Document code thoroughly by leaving references to requirement indices in comments.

8. **Validate code changes** - Use all available tools: code analyzers, compilers, tests. Fix any issues found.

9. **Format code** - Format code using available tools.

10. **Update tests** - If the project has tests, analyze them and fix or add tests that cover the modified requirements and code.

11. **Confirm changes** - Obtain user confirmation that all changes are correct.

"#;
