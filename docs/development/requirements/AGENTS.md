# Requirements

## General Requirements

**R.1.** All requirements must be written in English.

**R.2.** Requirement text must be formatted so that each line does not exceed 120 characters.

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
# Requirements Documentation

## General Rules

**R.1.** All requirements must be written in English.

**R.2.** Requirement text must be formatted so that each line does not exceed 120 characters.

## Requirements

```

**T.3.5.** The tool must return the content of the AGENTS.md file that was found or created.