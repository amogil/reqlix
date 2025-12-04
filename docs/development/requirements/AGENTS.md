# Requirements

## General Requirements

**R.1.** All requirements must be written in English.

**R.2.** Requirement text must be formatted so that each line does not exceed 120 characters.

## Parameter Constraints

**P.1.** All tool parameters must satisfy the following constraints:

- `project_root` - required, max 1000 characters
- `operation_description` - required, max 10000 characters
- `section` - required, max 100 characters
- `index` - required, max 10 characters
- `text` - required, max 10000 characters

**P.2.** If any parameter does not satisfy the constraints, the tool must return an error.

## Common Implementation Requirements

**C.1.** All tools must locate the requirements directory using the same algorithm as defined in T.3.1 for
`reqlix_get_instructions`. If AGENTS.md is not found, it must be created as defined in T.3.2.

**C.2.** Requirement format in section files: `**{index}.** {text}`

**C.3.** Requirement index format is flexible (e.g., "1", "2.1", "3.1.2").

# Tool Descriptions

## reqlix_get_instructions

**T.1.** Description (shown to LLM in tool list):

```
CALL THIS BEFORE ANY CODE OPERATION (reading or writing). 
Returns instructions on how to work with requirements.
This MCP server is the single source of truth for everything related to requirements.
```

**T.2.** Parameters:

- `project_root` (string, required) - Path to the project root directory. Used to locate requirements and
  project source code.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
  Used to provide relevant instructions.

**T.3.** Implementation requirements:

**T.3.1.** The tool must locate the requirements file using the following search order (check each path in order,
proceed to next if file not found):

1. `{project_root}/{REQLIX_REQ_REL_PATH}/AGENTS.md` (if environment variable `REQLIX_REQ_REL_PATH` is set)
2. `{project_root}/docs/development/requirements/AGENTS.md`
3. `{project_root}/docs/dev/req/AGENTS.md`

**T.3.2.** If no file is found, the tool must create a file with placeholder content at:

1. `{project_root}/{REQLIX_REQ_REL_PATH}/AGENTS.md` (if environment variable `REQLIX_REQ_REL_PATH` is set)
2. `{project_root}/docs/development/requirements/AGENTS.md` (if `REQLIX_REQ_REL_PATH` is not set)

**T.3.3.** If file creation fails or a permission error occurs at any stage, the tool must return an error.

**T.3.4.** Placeholder content for new requirements file:

```
# Requirements

## General Rules

**R.1.** All requirements must be written in English.

## Sections

Requirements are organized into the following sections:

- General requirements (key: general)
- Requirements change management (key: requirements_change_management)
- Testing requirements (key: testing)
- Code quality requirements (key: code_quality)
- Code writing requirements (key: code_style)
- Change validation requirements (key: change_validation)

```

**T.3.5.** The tool must return the content of the AGENTS.md file that was found or created.

## reqlix_get_requirements

**G.1.** Description (shown to LLM in tool list):

```
Returns all requirements in the specified section.
```

**G.2.** Parameters:

- `project_root` (string, required) - Path to the project root directory. Used to locate requirements and
  project source code.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
  Used to provide relevant instructions.
- `section` (string, required) - Section key (e.g., "general", "testing", "code_quality").

**G.3.** Implementation requirements:

**G.3.1.** The requirements directory is the directory where AGENTS.md is located (as per C.1).

**G.3.2.** Each section key corresponds to a file in the requirements directory: `{section}.md`.

**G.3.3.** If the section file exists, return its content.

**G.3.4.** If the section file does not exist, return the string: "No requirements in this section."

## reqlix_set_requirements

**S.1.** Description (shown to LLM in tool list):

```
Modifies a requirement in a section. To add a new requirement, specify a new index number.
Returns the added/modified requirement (text or index may be adjusted according to policies).
```

**S.2.** Parameters:

- `project_root` (string, required) - Path to the project root directory. Used to locate requirements and
  project source code.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
  Used to provide relevant instructions.
- `section` (string, required) - Section key (e.g., "general", "testing", "code_quality").
- `index` (string, required) - Requirement index (e.g., "1", "2.1", "3").
- `text` (string, required) - Requirement text.

**S.3.** Implementation requirements:

**S.3.1.** If the section file does not exist, create it.

**S.3.2.** If a requirement with the specified index exists, update it with the new text.

**S.3.3.** If a requirement with the specified index does not exist, add it while preserving index order.

**S.3.4.** Validate index order after modification. If the order is violated, fix it by reindexing.

**S.3.5.** After modification, rebuild the "## Sections" list in AGENTS.md based on all `*.md` files in the
requirements directory (excluding AGENTS.md).

**S.3.6.** Return the final requirement (after any adjustments to text or index).

## reqlix_delete_requirements

**D.1.** Description (shown to LLM in tool list):

```
Deletes a requirement with the specified index from a section.
Returns the deleted requirement text, or an error if not found.
```

**D.2.** Parameters:

- `project_root` (string, required) - Path to the project root directory. Used to locate requirements and
  project source code.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
  Used to provide relevant instructions.
- `section` (string, required) - Section key (e.g., "general", "testing", "code_quality").
- `index` (string, required) - Requirement index to delete (e.g., "1", "2.1", "3").

**D.3.** Implementation requirements:

**D.3.1.** If the section file does not exist, return an error: "Section not found."

**D.3.2.** If a requirement with the specified index does not exist, return an error: "Requirement not found."

**D.3.3.** If the requirement is found, delete it and return its content as the response.

**D.3.4.** If the section file becomes empty after deletion, delete the file.

**D.3.5.** After deletion, rebuild the "## Sections" list in AGENTS.md based on all `*.md` files in the
requirements directory (excluding AGENTS.md).
