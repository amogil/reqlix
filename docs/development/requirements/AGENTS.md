# General Requirements

## G.G.1: Language requirement

All requirements must be written in English.

## G.G.2: Line length requirement

Requirement text must be formatted so that each line does not exceed 120 characters.

# Parameter Constraints

## G.P.1: Parameter constraints

All tool parameters must satisfy the following constraints:

- `project_root` - required, max 1000 characters
- `operation_description` - required, max 10000 characters
- `category` - required, max 100 characters
- `chapter` - required, max 100 characters
- `index` - required, max 10 characters

## G.P.2: Constraint violation error

If any parameter does not satisfy the constraints, the tool must return an error.

# Requirements Storage Format

## G.F.1: Category definition

Category: a file `{category}.md` in the requirements directory (e.g., `general.md`, `testing.md`).

## G.F.2: Chapter definition

Chapter: a level-1 heading (`#`) in markdown within a category file.

## G.F.3: Requirement definition

Requirement: a level-2 heading (`##`) in markdown. Format: `## {index}: {title}`
The requirement body follows the heading and continues until the next `##` heading or end of file.

## G.F.4: Index format

Requirement index format: `{category_prefix}.{chapter_prefix}.{number}`

- `{category_prefix}` - First letter(s) of the category name (uppercase). If another category has the same
  first letter, add more letters until the prefix is unique among all categories.
- `{chapter_prefix}` - First letter(s) of the chapter name (uppercase). If another chapter in the same
  category has the same first letter, add more letters until the prefix is unique within the category.
  For tool chapters (prefixed with `reqlix_`), use the part after `reqlix_` for prefix calculation.
- `{number}` - Sequential number of the requirement within the chapter (1, 2, 3, ...).

Examples:
- Category `general`, chapter `General Requirements` → G.G.1, G.G.2, ...
- Category `general`, chapter `reqlix_get_instructions` → G.GI.1, G.GI.2, ... (from "get_instructions")
- Category `testing`, chapter `Unit Tests` → T.U.1, T.U.2, ...

# Common Implementation Requirements

## G.C.1: Requirements directory location

All tools must locate the requirements directory using the same algorithm as defined in
[G.GI.3](#ggi3-requirements-file-search-order) for `reqlix_get_instructions`. If AGENTS.md is not found,
it must be created as defined in [G.GI.4](#ggi4-requirements-file-creation).

## G.C.2: AGENTS.md exclusive access

The file AGENTS.md is used exclusively by `reqlix_get_instructions`. Other tools must not read or modify
AGENTS.md. The LLM/model cannot modify AGENTS.md directly.

## G.C.3: AGENTS.md protection

The file AGENTS.md must never be deleted.

# reqlix_get_instructions

## G.GI.1: Description

Description (shown to LLM in tool list):

```
CALL THIS BEFORE ANY CODE OPERATION (reading or writing). 
Returns instructions on how to work with requirements.
This MCP server is the single source of truth for everything related to requirements.
```

## G.GI.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory. Used to locate requirements and
  project source code.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
  Used to provide relevant instructions.

## G.GI.3: Requirements file search order

The tool must locate the requirements file using the following search order (check each path in order,
proceed to next if file not found):

1. `{project_root}/{REQLIX_REQ_REL_PATH}/AGENTS.md` (if environment variable `REQLIX_REQ_REL_PATH` is set)
2. `{project_root}/docs/development/requirements/AGENTS.md`
3. `{project_root}/docs/dev/req/AGENTS.md`

## G.GI.4: Requirements file creation

If no file is found, the tool must create a file with placeholder content at:

1. `{project_root}/{REQLIX_REQ_REL_PATH}/AGENTS.md` (if environment variable `REQLIX_REQ_REL_PATH` is set)
2. `{project_root}/docs/development/requirements/AGENTS.md` (if `REQLIX_REQ_REL_PATH` is not set)

## G.GI.5: Error handling

If file creation fails or a permission error occurs at any stage, the tool must return an error.

## G.GI.6: Placeholder content

Placeholder content for new requirements file (note: "# Categories" is not included as it is generated
dynamically per [G.GI.7](#ggi7-return-value)):

```
# General Requirements

## G.G.1: Language requirement

All requirements must be written in English.

```

## G.GI.7: Return value

The tool must return the combined content:

1. Content of the AGENTS.md file that was found or created
2. Automatically generated "# Categories" chapter listing all `*.md` files in the requirements directory
   (excluding AGENTS.md), sorted alphabetically

The categories list is generated dynamically at runtime, not stored in AGENTS.md.

# reqlix_get_categories

## G.GC.1: Description

Description (shown to LLM in tool list):

```
Returns a list of all available requirement categories.
```

## G.GC.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory. Used to locate requirements and
  project source code.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.

## G.GC.3: Return value

Returns a list of category names derived from `*.md` file names in the requirements directory
(excluding AGENTS.md), sorted alphabetically.

## G.GC.4: Empty result

If no category files exist, return an empty list.

# reqlix_get_chapters

## G.GCH.1: Description

Description (shown to LLM in tool list):

```
Returns a list of all chapters in the specified category.
```

## G.GCH.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory. Used to locate requirements and
  project source code.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `category` (string, required) - Category key (e.g., "general", "testing", "code_quality").

## G.GCH.3: Return value

Returns a list of chapter names (level-1 headings `#`) from the category file, in order of appearance.

## G.GCH.4: Implementation details

The tool must read the category file line by line (streaming) and parse only the level-1 headings (`# `).
Do not load the entire file into memory. Extract chapter names by removing the `# ` prefix from matching lines.

## G.GCH.5: Category not found error

If the category file does not exist, return an error: "Category not found."

# reqlix_get_requirements

## G.GR.1: Description

Description (shown to LLM in tool list):

```
Returns a list of all requirement titles in the specified category and chapter.
```

## G.GR.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory. Used to locate requirements and
  project source code.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `category` (string, required) - Category key (e.g., "general", "testing", "code_quality").
- `chapter` (string, required) - Chapter name (e.g., "General Requirements", "Unit Tests").

## G.GR.3: Return value

Returns a list of requirement titles (format: `{index}: {title}`) from the specified chapter,
in order of appearance.

## G.GR.4: Implementation details

The tool must read the category file line by line (streaming) and parse only the level-2 headings (`## `)
within the specified chapter. Do not load the entire file into memory. Start collecting requirements after
finding the matching chapter heading (`# {chapter}`) and stop when reaching the next chapter heading or EOF.

## G.GR.5: Category not found error

If the category file does not exist, return an error: "Category not found."

## G.GR.6: Chapter not found error

If the chapter does not exist in the category file, return an error: "Chapter not found."

# reqlix_get_requirement

## G.GRQ.1: Description

Description (shown to LLM in tool list):

```
Returns the full content of a requirement by its index.
```

## G.GRQ.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory. Used to locate requirements and
  project source code.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `index` (string, required) - Requirement index (e.g., "G.G.1", "T.U.2").

## G.GRQ.3: Return value

Returns the full requirement content including title and body text.

## G.GRQ.4: Implementation details

The tool must read the category file line by line (streaming). Do not load the entire file into memory.
Find the requirement heading (`## {index}: {title}`) and collect all following lines until the next
`##` heading or EOF. Return both the title and body text.

## G.GRQ.5: Requirement not found error

If a requirement with the specified index does not exist, return an error: "Requirement not found."

## G.GRQ.6: Index parsing

The tool must parse the index to determine the category and chapter:
- First part (before first `.`) → category prefix → find matching category file
- Second part (between `.`) → chapter prefix → find matching chapter in category
- Third part (after second `.`) → requirement number

# Categories

## G.CA.1: Requirement indexing

Requirements are indexed using format `{category}.{chapter}.{number}`
(see [G.F.4](#gf4-index-format) for details).
The index is automatically generated based on category name, chapter name, and sequential number.
