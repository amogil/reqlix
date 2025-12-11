# Instructions

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

6. Never edit files in docs/development/requirements directly. Always use this MCP server for all
   requirements operations.

7. When making code changes, follow this workflow:
    a. Update requirements if needed, then validate them (completeness, consistency, no redundancy or duplication)
    b. Request user review and confirmation of requirement changes
    c. Implement code changes according to the updated requirements
    d. Validate code changes for correctness and compliance with requirements; fix any issues
    e. Format all code
    f. Run automated checks (tests, code analyzers, etc.); fix any issues found

