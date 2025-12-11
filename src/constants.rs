// Parameter constraints (G.P.1) and placeholder content (G.REQLIX_GET_I.6)

pub(crate) const MAX_PROJECT_ROOT_LEN: usize = 1000;
pub(crate) const MAX_OPERATION_DESC_LEN: usize = 10000;
pub(crate) const MAX_CATEGORY_LEN: usize = 100;
pub(crate) const MAX_CHAPTER_LEN: usize = 100;
pub(crate) const MAX_INDEX_LEN: usize = 100;
pub(crate) const MAX_TEXT_LEN: usize = 10000;
pub(crate) const MAX_TITLE_LEN: usize = 100;
// G.REQLIX_GET_REQUIREMENT.5, G.REQLIX_U.7, G.TOOLREQLIXD.6, G.TOOLREQLIXS.5
pub(crate) const MAX_BATCH_SIZE: usize = 100;
// G.TOOLREQLIXS.5, G.P.1
pub(crate) const MAX_KEYWORD_LEN: usize = 200;

// G.REQLIX_GET_I.6
pub(crate) const PLACEHOLDER_CONTENT: &str = r#"# Instructions

These instructions are mandatory for all code operations:

1. Always verify that code matches requirements. If there are discrepancies, propose to the user
   to fix either the code or the requirements.

2. Make maximum effort to find relevant requirements for the code being modified and apply changes
   according to those requirements.

3. Document code thoroughly by leaving references to requirement indices in comments.

4. Requirement index format: `{CATEGORY}.{CHAPTER}.{NUMBER}` (e.g., `G.REQLIX_GET_I.1`, `T.U.2`).
   Requirements are organized hierarchically: **Category** groups related requirements together (e.g., general requirements, testing requirements).
   **Chapter** groups related requirements within a category (e.g., a specific tool or feature). **Requirement** is a single, atomic requirement with a unique index.

5. All requirements must be written in English.

6. Never edit files in {requirements_directory} directly. Always use this MCP server for all
   requirements operations.

7. When making code changes, follow this workflow:
    a. Update requirements if needed, then validate them (completeness, consistency, no redundancy or duplication)
    b. Request user review and confirmation of requirement changes
    c. Implement code changes according to the updated requirements
    d. Validate code changes for correctness and compliance with requirements; fix any issues
    e. Format all code
    f. Run automated checks (tests, code analyzers, etc.); fix any issues found

"#;
