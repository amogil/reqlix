
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
- `text` - required, max 10000 characters
- `title` - required for `reqlix_insert_requirement`, optional for `reqlix_update_requirement`, max 100 characters

## G.P.2: Constraint violation error

If any parameter does not satisfy the constraints, the tool must return an error.

# Requirements Storage Format

## G.R.1: Category definition

Category: a file `{category}.md` in the requirements directory (e.g., `general.md`, `testing.md`).

## G.R.2: Chapter definition

Chapter: a level-1 ATX-style heading in markdown within a category file that is not inside a code block.

Markdown level-1 heading format: the line must start with exactly one `#` character followed by a space, then the chapter name. Example: `# Chapter Name`

The heading must be parsed according to markdown syntax rules:
- The line must start with `#` (optionally preceded by up to 3 spaces, which are ignored)
- After `#` there must be a space
- The rest of the line (after the space) is the chapter name
- Trailing `#` characters, if present, are part of the heading text
- The heading must not be inside a code block (fenced with ```)

Code blocks are fenced with triple backticks (```). Any level-1 heading that appears between opening ``` and closing ``` is not considered a chapter heading.

## G.R.3: Requirement definition

Requirement: a level-2 ATX-style heading in markdown. Format: `## {index}: {title}`

The requirement body follows the heading and continues until the next level-2 heading (`##`) or end of file.

Markdown level-2 heading format: the line must start with exactly two `#` characters followed by a space, then the requirement index, a colon, a space, and the title. Example: `## G.G.1: Requirement title`

The heading must be parsed according to markdown syntax rules:
- The line must start with `##` (optionally preceded by up to 3 spaces, which are ignored)
- After `##` there must be a space
- The rest of the line follows the format `{index}: {title}` where `{index}` is the requirement index and `{title}` is the requirement title
- Trailing `#` characters, if present, are part of the heading text
- The heading must not be inside a code block (fenced with ```)

## G.R.4: Index format

Requirement index format: `{category_prefix}.{chapter_prefix}.{number}`

The dot (`.`) is the delimiter between parts. Each part is parsed by splitting the index on dots.

- `{category_prefix}` - First letter(s) of the category name (uppercase). If another category has the same
  first letter, add more letters until the prefix is unique among all categories.
- `{chapter_prefix}` - First letter(s) of the chapter name (uppercase). If another chapter in the same
  category has the same first letter, add more letters until the prefix is unique within the category.
  For tool chapters (prefixed with `reqlix_`), use the part after `reqlix_` for prefix calculation.
  The prefix is formed by taking the first letter(s) of each word in the function name, using uppercase.
  If multiple chapters would have the same prefix, add more letters from subsequent words until unique.
- `{number}` - Sequential number of the requirement within the chapter (1, 2, 3, ...).

Examples:

- Category `general`, chapter `General Requirements` → G.G.1, G.G.2, ...
- Category `general`, chapter `reqlix_get_instructions` → G.GET.1, G.GET.2, ... (from "get_instructions": GET)
- Category `general`, chapter `reqlix_get_categories` → G.GET_C.1, G.GET_C.2, ... (from "get_categories": GET_C)
- Category `general`, chapter `reqlix_get_chapters` → G.GET_CH.1, G.GET_CH.2, ... (from "get_chapters": GET_CH)
- Category `general`, chapter `reqlix_get_requirements` → G.GET_R.1, G.GET_R.2, ... (from "get_requirements": GET_R)
- Category `general`, chapter `reqlix_get_requirement` → G.GRQ.1, G.GRQ.2, ... (from "get_requirement": GRQ)
- Category `general`, chapter `reqlix_insert_requirement` → G.INSE.1, G.INSE.2, ... (from "insert_requirement": INSE)
- Category `general`, chapter `reqlix_update_requirement` → G.U.1, G.U.2, ... (from "update_requirement": U)
- Category `testing`, chapter `Unit Tests` → T.U.1, T.U.2, ...

## G.R.5: Requirement parsing boundaries

When parsing requirements from markdown files, tools must correctly identify requirement boundaries using proper markdown parsing:

1. A requirement starts with a markdown level-2 heading (`## {index}: {title}`) parsed according to markdown syntax rules (see [G.R.3](#gr3-requirement-definition)) and includes all following lines until:
   - The next markdown level-2 heading (`##`) is encountered (outside code blocks), OR
   - The end of file is reached

2. Code blocks (fenced with ```) must be parsed correctly:
   - All content within a code block (between opening ``` and closing ```) is part of the requirement body
   - Code block boundaries must be identified by matching opening and closing fences
   - Multi-line code blocks must be handled correctly
   - Headings inside code blocks must not be treated as requirement or chapter boundaries

3. Level-1 headings (`#`) within a requirement body are still part of that requirement and should not be treated as chapter boundaries when parsing requirement content. These headings must be parsed according to markdown syntax rules but are treated as regular content, not chapter boundaries.

4. When reading a requirement, the parser must:
   - Parse markdown level-2 headings correctly (not just search for lines starting with `## `)
   - Collect the requirement heading line (`## {index}: {title}`)
   - Collect all body text until the next markdown level-2 heading (outside code blocks) or EOF
   - Preserve original formatting including line breaks and indentation

5. Examples and code snippets within requirements are part of the requirement text and must be included in full

The parser must correctly handle markdown syntax for all headings, not just search for string prefixes. This ensures that requirements with complex formatting, code examples, or nested structures are parsed correctly and completely.

## G.C.1: Requirements directory location

All tools must locate the requirements directory using the same algorithm as defined in
[G.GET.3](#gget3-requirements-file-search-order) for `reqlix_get_instructions`. If AGENTS.md is not found,
it must be created as defined in [G.GET.4](#gget4-requirements-file-creation).

## G.C.2: Directory creation

If the requirements directory does not exist, it must be created automatically (including parent directories).

## G.C.3: AGENTS.md exclusive access

The file AGENTS.md is used exclusively by `reqlix_get_instructions`. Other tools must not read or modify
AGENTS.md. The LLM/model cannot modify AGENTS.md directly.

## G.C.4: AGENTS.md protection

The file AGENTS.md must never be deleted.

## G.C.5: JSON response format

All tool responses must be in JSON format. This includes both successful responses and errors.

## G.C.6: Error response format

Error responses must use the following JSON format:

```json
{
  "success": false,
  "error": "Human-readable error message"
}
```

## G.C.7: Category lookup by prefix

To find a category file by prefix:

1. List all `*.md` files in the requirements directory (excluding AGENTS.md)
2. For each file, extract category name from filename (without `.md`)
3. Calculate what prefix this category would have using the algorithm in [G.R.4](#gr4-index-format)
4. Return the category whose calculated prefix matches the search prefix
5. If no category matches the prefix, return an error "Category not found"

# reqlix_get_instructions

## G.GET.1: Description

Description (shown to LLM in tool list):

```
CALL THIS BEFORE ANY CODE OPERATION (reading or writing). 
Returns instructions on how to work with requirements.
This MCP server is the single source of truth for everything related to requirements.
```

## G.GET.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.

## G.GET.3: Requirements file search order

The tool must locate the requirements file using the following search order (check each path in order,
proceed to next if file not found):

1. `{project_root}/{REQLIX_REQ_REL_PATH}/AGENTS.md` (if environment variable `REQLIX_REQ_REL_PATH` is set)
2. `{project_root}/docs/development/requirements/AGENTS.md`
3. `{project_root}/docs/dev/req/AGENTS.md`

## G.GET.4: Requirements file creation

If no file is found, the tool must create a file with placeholder content at:

1. `{project_root}/{REQLIX_REQ_REL_PATH}/AGENTS.md` (if environment variable `REQLIX_REQ_REL_PATH` is set)
2. `{project_root}/docs/development/requirements/AGENTS.md` (if `REQLIX_REQ_REL_PATH` is not set)

## G.GET.5: Error handling

If file creation fails or a permission error occurs at any stage, the tool must return an error.

## G.GET.6: Placeholder content

Placeholder content for new requirements file (note: "# Categories" is not included as it is generated
dynamically per [G.GET.7](#gget7-return-value)).

The placeholder `{requirements_directory}` must be replaced with the actual path to the requirements
directory at runtime:

```

These instructions are mandatory for all code operations:

1. Always verify that code matches requirements. If there are discrepancies, propose to the user
   to fix either the code or the requirements.

2. Make maximum effort to find relevant requirements for the code being modified and apply changes
   according to those requirements.

3. Document code thoroughly by leaving references to requirement indices in comments.
   Requirement index format: `{CATEGORY}.{CHAPTER}.{NUMBER}` (e.g., `G.GET.1`, `T.U.2`).

4. All requirements must be written in English.

5. Never edit files in {requirements_directory} directly. Always use this MCP server for all
   requirements operations.

```

## G.GET.7: Return value

The tool must return the combined content:

1. Content of the AGENTS.md file that was found or created
2. Automatically generated "# Categories" chapter with a markdown list of all `*.md` files in the
   requirements directory (excluding AGENTS.md), sorted alphabetically

The categories list is generated dynamically at runtime, not stored in AGENTS.md.

Format of generated Categories chapter:

```
# Categories

- general
- testing
- code_quality
```

## G.GET.8: Response format

```json
{
  "success": true,
  "data": {
    "content": "# Instructions\n\nThese instructions are mandatory...\n\n# Categories\n\n- general\n- testing"
  }
}
```

# reqlix_get_categories

## G.GET_C.1: Description

Description (shown to LLM in tool list):

```
Returns a list of all available requirement categories (category file names without .md extension).
Use this to discover what categories exist before querying chapters or requirements.
```

## G.GET_C.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.

## G.GET_C.3: Return value

Returns a list of category names derived from `*.md` file names in the requirements directory
(excluding AGENTS.md), sorted alphabetically.

## G.GET_C.4: Response format

```json
{
  "success": true,
  "data": {
    "categories": [
      "general",
      "testing",
      "code_quality"
    ]
  }
}
```

If no category files exist, return empty array: `"categories": []`

# reqlix_get_chapters

## G.GET_CH.1: Description

Description (shown to LLM in tool list):

```
Returns a list of all chapters (level-1 headings) in the specified category file.
Use this to discover what chapters exist in a category before querying requirements.
```

## G.GET_CH.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `category` (string, required) - Category key (e.g., "general", "testing").

## G.GET_CH.3: Implementation details

The tool must read the category file line by line (streaming) and parse markdown level-1 headings that are not inside code blocks. Do not load the entire file into memory.

When parsing markdown headings:
1. Track whether the current line is inside a code block (fenced with ```)
2. For each line, check if it is a valid markdown level-1 heading:
   - The line may start with up to 3 spaces (indentation, which is ignored)
   - The line must start with exactly one `#` character
   - After `#` there must be a space
   - The rest of the line (after the space) is the chapter name
3. Ignore level-1 headings that appear inside code blocks
4. Extract chapter names by parsing the markdown heading syntax correctly, not just by string prefix matching

Code block boundaries must be identified by matching opening and closing fences (```). All content between opening ``` and closing ``` is considered inside a code block and should not be parsed as chapter headings.

The parser must correctly handle markdown syntax, not just search for lines starting with `# `.

## G.GET_CH.4: Response format

Success:

```json
{
  "success": true,
  "data": {
    "category": "general",
    "chapters": [
      "General Requirements",
      "Parameter Constraints",
      "reqlix_get_instructions"
    ]
  }
}
```

If category has no chapters, return empty array: `"chapters": []`

Error (category not found): Use error format from [G.C.6](#gc6-error-response-format).

# reqlix_get_requirements

## G.GET_R.1: Description

Description (shown to LLM in tool list):

```
Returns a list of all requirement titles (with indices) in the specified category and chapter.
Use this to browse requirements in a chapter. To get full requirement content, use reqlix_get_requirement.
```

## G.GET_R.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `category` (string, required) - Category key (e.g., "general", "testing").
- `chapter` (string, required) - Chapter name (e.g., "General Requirements", "Unit Tests").

## G.GET_R.3: Implementation details

The tool must read the category file line by line (streaming) and parse markdown level-2 headings within the specified chapter. Do not load the entire file into memory.

When parsing markdown headings:
1. Track whether the current line is inside a code block (fenced with ```)
2. Start collecting requirements after finding the matching chapter heading (`# {chapter}`) and stop when reaching the next chapter heading or EOF
3. For each line within the target chapter, check if it is a valid markdown level-2 heading:
   - The line may start with up to 3 spaces (indentation, which is ignored)
   - The line must start with exactly two `#` characters
   - After `##` there must be a space
   - The rest of the line follows the format `{index}: {title}` where `{index}` is the requirement index and `{title}` is the requirement title
4. Ignore level-2 headings that appear inside code blocks
5. Extract requirement indices and titles by parsing the markdown heading syntax correctly, not just by string prefix matching

The parser must correctly handle markdown syntax, not just search for lines starting with `## `.

## G.GET_R.4: Response format

Success:

```json
{
  "success": true,
  "data": {
    "category": "general",
    "chapter": "General Requirements",
    "requirements": [
      {
        "index": "G.G.1",
        "title": "Language requirement"
      },
      {
        "index": "G.G.2",
        "title": "Line length requirement"
      }
    ]
  }
}
```

If chapter has no requirements, return empty array: `"requirements": []`

Errors (category/chapter not found): Use error format from [G.C.6](#gc6-error-response-format).

# reqlix_get_requirement

## G.GRQ.1: Description

Description (shown to LLM in tool list):

```
Returns the full content (title and text) of a requirement by its index.
Index format: {CATEGORY}.{CHAPTER}.{NUMBER} (e.g., G.GET.1, T.U.2).
Use this to read a specific requirement when you know its index.
```

## G.GRQ.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `index` (string, required) - Requirement index (e.g., "G.G.1", "T.U.2").

## G.GRQ.3: Index parsing and file lookup

The tool must parse the index by splitting on dots (`.`):

- First part → category prefix
- Second part → chapter prefix
- Third part → requirement number

To find the requirement:

1. Use algorithm from [G.C.7](#gc7-category-lookup-by-prefix) to find category by prefix
2. Scan the category file to find a chapter containing a requirement with matching chapter prefix:
   - Read file line by line, tracking current chapter (last seen markdown level-1 heading `#`, parsed according to [G.R.2](#gr2-chapter-definition))
   - For each markdown level-2 heading (`## X.Y.Z: title`), parse it according to [G.R.3](#gr3-requirement-definition) and extract `Y` (chapter prefix from index)
   - If `Y` matches the search chapter prefix, the current chapter is the target
   - Ignore headings inside code blocks
3. If no chapter contains requirements with this chapter prefix, return error "Requirement not found"
4. Find the requirement by full index within the chapter using proper markdown parsing (see [G.GRQ.4](#ggrq4-implementation-details))

## G.GRQ.4: Implementation details

The tool must read the category file line by line (streaming). Do not load the entire file into memory.

When finding and reading a requirement:
1. Track whether the current line is inside a code block (fenced with ```)
2. Parse markdown level-2 headings to find the requirement heading (`## {index}: {title}`):
   - The line may start with up to 3 spaces (indentation, which is ignored)
   - The line must start with exactly two `#` characters
   - After `##` there must be a space
   - The rest of the line follows the format `{index}: {title}`
3. When the matching requirement heading is found, collect all following lines until:
   - The next markdown level-2 heading (`##`) is encountered (outside code blocks), OR
   - The end of file is reached
4. Return both the title (extracted from the heading) and body text (all lines after the heading until the next heading or EOF)

The parser must correctly handle markdown syntax, not just search for lines starting with `## `.

## G.GRQ.5: Response format

Success:

```json
{
  "success": true,
  "data": {
    "index": "G.G.1",
    "title": "Language requirement",
    "text": "All requirements must be written in English.",
    "category": "general",
    "chapter": "General Requirements"
  }
}
```

Error (requirement not found): Use error format from [G.C.6](#gc6-error-response-format).

# reqlix_insert_requirement

## G.INSE.1: Description

Description (shown to LLM in tool list):

```
Inserts a new requirement into the specified category and chapter.
The title must be generated by the LLM and provided as a parameter.
The title must be unique within the chapter. Returns the requirement with index and title.
```

## G.INSE.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `category` (string, required) - Category key (e.g., "general", "testing").
- `chapter` (string, required) - Chapter name (e.g., "General Requirements", "Unit Tests").
- `text` (string, required) - Requirement text (body content).
- `title` (string, required) - Requirement title. A concise name that reflects the essence of the requirement.
   Must be generated by the LLM and be unique within the chapter.

## G.INSE.3: Algorithm

The tool must execute the following steps:

0. **Validate parameters**: Validate all input parameters according to [G.INSE.6](#ginse6-parameter-validation).

1. **Find or create category**: Locate the category file `{category}.md`. If not found, create a new empty file.

2. **Find or create chapter**: Search for the chapter heading (`# {chapter}`) in the category file using proper markdown parsing (see [G.R.2](#gr2-chapter-definition)). If not found, append the chapter heading to the end of the file.

3. **Validate title uniqueness**: Check that the title is unique within the chapter. Parse all requirements in the chapter using proper markdown parsing (see [G.R.3](#gr3-requirement-definition)). If a requirement with the same title already exists, return an error "Title already exists in chapter".

4. **Generate index**: Create the requirement index `{category_prefix}.{chapter_prefix}.{number}`:
    - If first requirement in category: determine `{category_prefix}` using the unique prefix algorithm
      (see [G.R.4](#gr4-index-format))
    - If not first: reuse existing category prefix from other requirements
    - If first requirement in chapter: determine `{chapter_prefix}` from chapter name using unique prefix algorithm
    - If not first: reuse existing chapter prefix from other requirements in this chapter
    - `{number}`: next sequential number ensuring uniqueness within the chapter

5. **Insert requirement**: Append the requirement (`## {index}: {title}` followed by text) to the chapter.

6. **Return result**: Return the full requirement data.

## G.INSE.4: Implementation details

The tool must read the category file to check existing requirements and prefixes, then append new content.
When creating a new chapter, ensure proper markdown formatting with blank lines before and after headings.

## G.INSE.5: Response format

Success:

```json
{
  "success": true,
  "data": {
    "index": "G.G.3",
    "title": "Generated title",
    "text": "Requirement text content...",
    "category": "general",
    "chapter": "General Requirements"
  }
}
```

Errors (file system error, title already exists): Use error format from [G.C.6](#gc6-error-response-format).

## G.INSE.6: Parameter validation

Before executing the insertion algorithm, the tool must validate all input parameters according to the constraints defined in [G.P.1](#gp1-parameter-constraints). If any parameter violates these constraints, the tool must return an error as specified in [G.P.2](#gp2-constraint-violation-error).

This validation must occur before any file system operations or requirement processing.

# reqlix_update_requirement

## G.U.1: Description

Description (shown to LLM in tool list):

```
Updates an existing requirement by its index with new text and optional new title.
If title is provided, it must be unique within the chapter. If not provided, the existing title is kept.
```

## G.U.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `index` (string, required) - Requirement index (e.g., "G.G.1", "T.U.2").
- `text` (string, required) - New requirement text (body content).
- `title` (string, optional) - New requirement title. A concise name that reflects the essence of the requirement.
   If provided, must be unique within the chapter. If not provided, the existing title is kept.

## G.U.3: Algorithm

The tool must execute the following steps:

0. **Validate parameters**: Validate all input parameters according to [G.U.6](#gu6-parameter-validation).

1. **Parse index**: Extract category prefix, chapter prefix, and requirement number from the index
   (see [G.GRQ.3](#ggrq3-index-parsing-and-file-lookup)).

2. **Find requirement**: Locate the requirement by its index using proper markdown parsing (see [G.GRQ.4](#ggrq4-implementation-details)). If not found, return error.

3. **Determine new title**: If `title` parameter is provided, use it. Otherwise, keep the existing title.

4. **Validate title uniqueness**: If a new title was provided, check that it is unique within the chapter
   (excluding the current requirement). Parse all requirements in the chapter using proper markdown parsing (see [G.R.3](#gr3-requirement-definition)). If a requirement with the same title already exists, return an error
   "Title already exists in chapter".

5. **Update requirement**: Replace the existing requirement heading and body with the new title (or keep existing)
   and new text. Keep the same index.

6. **Return result**: Return the full updated requirement data.

## G.U.4: Implementation details

The tool must read the category file, locate the requirement, replace its content in place, and write
the updated file. Preserve all other content unchanged.

## G.U.5: Response format

Success:

```json
{
  "success": true,
  "data": {
    "index": "G.G.1",
    "title": "Updated title",
    "text": "Updated requirement text...",
    "category": "general",
    "chapter": "General Requirements"
  }
}
```

Errors (requirement not found, file system error, title already exists): Use error format from
[G.C.6](#gc6-error-response-format).

## G.U.6: Parameter validation

Before executing the update algorithm, the tool must validate all input parameters according to the constraints defined in [G.P.1](#gp1-parameter-constraints). If any parameter violates these constraints, the tool must return an error as specified in [G.P.2](#gp2-constraint-violation-error).

This validation must occur before any file system operations or requirement processing.
