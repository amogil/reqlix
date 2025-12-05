
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

Category: a file `{category}.md` in the requirements directory (e.g., `general.md`, `testing.md`). File's format is markdown.

## G.R.2: Chapter definition

Chapter: a level-1 ATX-style heading in markdown within a category file.

A chapter heading is an ATX-style heading with heading level 1 (one `#` character). 
The heading content (text after the `#` and required space) is the chapter name. Example: `# Chapter Name` gives `Chapter Name`

## G.R.3: Requirement definition

Requirement: a level-2 ATX-style heading in markdown. Format: `## {index}: {title}`

The requirement body is the content of this section, excluding the heading.

Example: `## G.G.1: Requirement title`

## G.R.4: Index format

Requirement index format: `{category_prefix}.{chapter_prefix}.{number}`

The dot (`.`) is the delimiter between parts. Each part is parsed by splitting the index on dots.

- `{category_prefix}` - First letter(s) of the category name (uppercase). Algorithm: if the category file already contains requirements, extract the prefix from an existing requirement index; otherwise, calculate a unique prefix that does not conflict with other category files by taking the first letter(s) and adding more letters until unique.
- `{chapter_prefix}` - First letter(s) of the chapter name (uppercase). Algorithm: if the chapter already contains requirements, extract the prefix from an existing requirement index; otherwise, calculate a unique prefix that does not conflict with other chapters in the same category by taking the first character(s) of the chapter name (using uppercase) and adding more characters until unique.
- `{number}` - Sequential number of the requirement within the chapter (1, 2, 3, ...).

Examples:

- Category `general`, chapter `General Requirements` → G.G.1, G.G.2, ...
- Category `general`, chapter `reqlix_get_instructions` → G.R.1, G.R.2, ... (if unique) or longer prefix if conflicts
- Category `general`, chapter `reqlix_get_categories` → G.R.1, G.R.2, ... (if unique) or longer prefix if conflicts
- Category `general`, chapter `reqlix_get_chapters` → G.R.1, G.R.2, ... (if unique) or longer prefix if conflicts
- Category `general`, chapter `reqlix_get_requirements` → G.R.1, G.R.2, ... (if unique) or longer prefix if conflicts
- Category `general`, chapter `reqlix_get_requirement` → G.R.1, G.R.2, ... (if unique) or longer prefix if conflicts
- Category `general`, chapter `reqlix_insert_requirement` → G.R.1, G.R.2, ... (if unique) or longer prefix if conflicts
- Category `general`, chapter `reqlix_update_requirement` → G.R.1, G.R.2, ... (if unique) or longer prefix if conflicts
- Category `testing`, chapter `Unit Tests` → T.U.1, T.U.2, ...

## G.R.5: Requirement parsing boundaries

When parsing requirements from markdown files, tools must correctly identify requirement boundaries:

A requirement starts with a level-2 ATX-style heading (see [G.R.3](#gr3-requirement-definition)). The requirement body is the content of this markdown section. The markdown parser automatically determines section boundaries (section ends at the next heading of the same or higher level, or at end of file).

## G.C.1: Requirements directory location

All tools must locate the requirements directory using the same algorithm as defined in
[G.REQLIX_GET_I.3](#greqlix_get_i3-requirements-file-search-order) for `reqlix_get_instructions`. If AGENTS.md is not found,
it must be created as defined in [G.REQLIX_GET_I.4](#greqlix_get_i4-requirements-file-creation).

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

# Tool: reqlix_get_instructions

## G.REQLIX_GET_I.1: Description

Description (shown to LLM in tool list):

```
CALL THIS BEFORE ANY CODE OPERATION (reading or writing). 
Returns instructions on how to work with requirements.
This MCP server is the single source of truth for everything related to requirements.
```

## G.REQLIX_GET_I.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.

## G.REQLIX_GET_I.3: Requirements file search order

The tool must locate the requirements file using the following search order (check each path in order,
proceed to next if file not found):

1. `{project_root}/{REQLIX_REQ_REL_PATH}/AGENTS.md` (if environment variable `REQLIX_REQ_REL_PATH` is set)
2. `{project_root}/docs/development/requirements/AGENTS.md`
3. `{project_root}/docs/dev/req/AGENTS.md`

## G.REQLIX_GET_I.4: Requirements file creation

If no file is found, the tool must create a file with placeholder content at:

1. `{project_root}/{REQLIX_REQ_REL_PATH}/AGENTS.md` (if environment variable `REQLIX_REQ_REL_PATH` is set)
2. `{project_root}/docs/development/requirements/AGENTS.md` (if `REQLIX_REQ_REL_PATH` is not set)

## G.REQLIX_GET_I.5: Error handling

If file creation fails or a permission error occurs at any stage, the tool must return an error.

## G.REQLIX_GET_I.6: Placeholder content

Placeholder content for new requirements file (note: "# Categories" is not included as it is generated
dynamically per [G.REQLIX_GET_I.7](#greqlix_get_i7-return-value)).

The placeholder `{requirements_directory}` must be replaced with the relative path to the requirements
directory from the project root at runtime (e.g., `docs/development/requirements` or `{REQLIX_REQ_REL_PATH}` if set):

```

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

```

## G.REQLIX_GET_I.7: Return value

The tool must return the combined content:

1. Content of the AGENTS.md file that was found or created
2. Automatically generated "# Categories" chapter with a markdown list of all categories in the
   requirements directory (excluding AGENTS.md), sorted alphabetically

The categories list is generated dynamically at runtime, not stored in AGENTS.md.

Format of generated Categories chapter:

```
# Categories

- general
- testing
- code_quality
```

## G.REQLIX_GET_I.8: Response format

```json
{
  "success": true,
  "data": {
    "content": "# Instructions\n\nThese instructions are mandatory...\n\n# Categories\n\n- general\n- testing"
  }
}
```

# Tool: reqlix_get_categories

## G.REQLIX_GET_CA.1: Description

Description (shown to LLM in tool list):

```
Returns a list of all available requirement categories.
Use this to discover what categories exist before querying chapters or requirements.
```

## G.REQLIX_GET_CA.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.

## G.REQLIX_GET_CA.3: Return value

Returns a list of category names derived from `*.md` file names in the requirements directory
(excluding AGENTS.md), sorted alphabetically.

## G.REQLIX_GET_CA.4: Response format

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

# Tool: reqlix_get_chapters

## G.REQLIX_GET_CH.1: Description

Description (shown to LLM in tool list):

```
Returns a list of all chapters in the specified category.
Use this to discover what chapters exist in a category before querying requirements.
```

## G.REQLIX_GET_CH.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `category` (string, required) - Category key (e.g., "general", "testing").

## G.REQLIX_GET_CH.3: Implementation details

The tool must parse level-1 ATX-style headings according to [G.R.2](#gr2-chapter-definition), ignoring those inside fenced code blocks.

## G.REQLIX_GET_CH.4: Response format

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

## G.REQLIX_GET_REQUIREMENTS.1: Description

Description (shown to LLM in tool list):

```
Returns a list of all requirement titles (with indices) in the specified category and chapter.
Use this to browse requirements in a chapter. To get full requirement content, use reqlix_get_requirement.
```

## G.REQLIX_GET_REQUIREMENTS.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `category` (string, required) - Category key (e.g., "general", "testing").
- `chapter` (string, required) - Chapter name (e.g., "General Requirements", "Unit Tests").

## G.REQLIX_GET_REQUIREMENTS.3: Implementation details

The tool must parse level-2 ATX-style headings within the specified chapter.

When parsing markdown headings:
1. Track whether the current position is inside a fenced code block (delimited by ```)
2. Start collecting requirements after finding the matching chapter heading (level-1 ATX-style heading with content matching the chapter name) and stop when reaching the next level-1 ATX-style heading or EOF
3. For each ATX-style heading encountered within the target chapter, check if it is a level-2 heading:
   - The heading must be an ATX-style heading of level 2 (exactly two `#` characters)
   - The heading may be indented with up to 3 spaces (indentation is ignored in markdown)
   - The heading content must follow the format `{index}: {title}` where `{index}` is the requirement index and `{title}` is the requirement title
4. Ignore level-2 ATX-style headings that appear inside fenced code blocks
5. Extract requirement indices and titles from the heading content by parsing the markdown heading syntax correctly, not just by string prefix matching

The parser must correctly handle markdown syntax, not just search for lines starting with `## `.

## G.REQLIX_GET_REQUIREMENTS.4: Response format

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

## G.REQLIX_GET_REQUIREMENT.1: Description

Description (shown to LLM in tool list):

```
Returns the full content (title and text) of a requirement by its index.
Index format: {CATEGORY}.{CHAPTER}.{NUMBER} (e.g., G.REQLIX_GET_I.1, T.U.2).
Use this to read a specific requirement when you know its index.
```

## G.REQLIX_GET_REQUIREMENT.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `index` (string, required) - Requirement index (e.g., "G.G.1", "T.U.2").

## G.REQLIX_GET_REQUIREMENT.3: Index parsing and file lookup

The tool must parse the index by splitting on dots (`.`):

- First part → category prefix
- Second part → chapter prefix
- Third part → requirement number

To find the requirement:

1. Use algorithm from [G.C.7](#gc7-category-lookup-by-prefix) to find category by prefix
2. Scan the category file to find a chapter containing a requirement with matching chapter prefix:
   - Track current chapter (last seen level-1 ATX-style heading, parsed according to [G.R.2](#gr2-chapter-definition))
   - For each level-2 ATX-style heading encountered, parse it according to [G.R.3](#gr3-requirement-definition) and extract the chapter prefix `Y` from the index in the heading content
   - If `Y` matches the search chapter prefix, the current chapter is the target
   - Ignore headings inside fenced code blocks
3. If no chapter contains requirements with this chapter prefix, return error "Requirement not found"
4. Find the requirement by full index within the chapter using proper markdown parsing (see [G.REQLIX_GET_REQUIREMENT.4](#greqlix_get_requirement4-implementation-details))

## G.REQLIX_GET_REQUIREMENT.4: Implementation details

When finding and reading a requirement:
1. Track whether the current position is inside a fenced code block (delimited by ```)
2. Parse level-2 ATX-style headings to find the requirement heading:
   - The heading must be an ATX-style heading of level 2 (exactly two `#` characters)
   - The heading may be indented with up to 3 spaces (indentation is ignored in markdown)
   - The heading content must follow the format `{index}: {title}` where `{index}` matches the search index
3. When the matching requirement heading is found, collect all following content until:
   - The next level-2 ATX-style heading is encountered (outside fenced code blocks), OR
   - The end of file is reached
4. Return both the title (extracted from the heading content) and body text (all content after the heading until the next level-2 heading or EOF)

The parser must correctly handle markdown syntax, not just search for lines starting with `## `.

## G.REQLIX_GET_REQUIREMENT.5: Response format

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

## G.REQLIX_I.1: Description

Description (shown to LLM in tool list):

```
Inserts a new requirement into the specified category and chapter.
The title must be generated by the LLM and provided as a parameter.
The title must be unique within the chapter. Returns the requirement with index and title.
```

## G.REQLIX_I.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `category` (string, required) - Category key (e.g., "general", "testing").
- `chapter` (string, required) - Chapter name (e.g., "General Requirements", "Unit Tests").
- `text` (string, required) - Requirement text (body content).
- `title` (string, required) - Requirement title. A concise name that reflects the essence of the requirement.
   Must be generated by the LLM and be unique within the chapter.

## G.REQLIX_I.3: Algorithm

The tool must execute the following steps:

0. **Validate parameters**: Validate all input parameters according to [G.REQLIX_I.6](#greqlix_i6-parameter-validation).

1. **Find or create category**: Locate the category file `{category}.md`. If not found, create a new empty file.

2. **Find or create chapter**: Search for a level-1 ATX-style heading with heading content matching the chapter name in the category file using proper markdown parsing (see [G.R.2](#gr2-chapter-definition)). If not found, append a level-1 ATX-style heading with the chapter name to the end of the file.

3. **Validate title uniqueness**: Check that the title is unique within the chapter. Parse all requirements in the chapter using proper markdown parsing (see [G.R.3](#gr3-requirement-definition)). If a requirement with the same title already exists, return an error "Title already exists in chapter".

4. **Generate index**: Create the requirement index `{category_prefix}.{chapter_prefix}.{number}`:
    - If first requirement in category: determine `{category_prefix}` using the unique prefix algorithm
      (see [G.R.4](#gr4-index-format))
    - If not first: reuse existing category prefix from other requirements
    - If first requirement in chapter: determine `{chapter_prefix}` from chapter name using unique prefix algorithm
    - If not first: reuse existing chapter prefix from other requirements in this chapter
    - `{number}`: next sequential number ensuring uniqueness within the chapter

5. **Insert requirement**: Append a level-2 ATX-style heading with content `{index}: {title}` followed by the requirement text to the chapter.

6. **Return result**: Return the full requirement data.

## G.REQLIX_I.4: Implementation details

The tool must read the category file to check existing requirements and prefixes, then append new content.
When creating a new chapter, ensure proper markdown formatting with blank lines before and after headings.

## G.REQLIX_I.5: Response format

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

## G.REQLIX_I.6: Parameter validation

Before executing the insertion algorithm, the tool must validate all input parameters according to the constraints defined in [G.P.1](#gp1-parameter-constraints). If any parameter violates these constraints, the tool must return an error as specified in [G.P.2](#gp2-constraint-violation-error).

This validation must occur before any file system operations or requirement processing.

# reqlix_update_requirement

## G.REQLIX_U.1: Description

Description (shown to LLM in tool list):

```
Updates an existing requirement by its index with new text and optional new title.
If title is provided, it must be unique within the chapter. If not provided, the existing title is kept.
```

## G.REQLIX_U.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `index` (string, required) - Requirement index (e.g., "G.G.1", "T.U.2").
- `text` (string, required) - New requirement text (body content).
- `title` (string, optional) - New requirement title. A concise name that reflects the essence of the requirement.
   If provided, must be unique within the chapter. If not provided, the existing title is kept.

## G.REQLIX_U.3: Algorithm

The tool must execute the following steps:

0. **Validate parameters**: Validate all input parameters according to [G.REQLIX_U.6](#greqlix_u6-parameter-validation).

1. **Parse index**: Extract category prefix, chapter prefix, and requirement number from the index
   (see [G.REQLIX_GET_REQUIREMENT.3](#greqlix_get_requirement3-index-parsing-and-file-lookup)).

2. **Find requirement**: Locate the requirement by its index using proper markdown parsing (see [G.REQLIX_GET_REQUIREMENT.4](#greqlix_get_requirement4-implementation-details)). If not found, return error.

3. **Determine new title**: If `title` parameter is provided, use it. Otherwise, keep the existing title.

4. **Validate title uniqueness**: If a new title was provided, check that it is unique within the chapter
   (excluding the current requirement). Parse all requirements in the chapter using proper markdown parsing (see [G.R.3](#gr3-requirement-definition)). If a requirement with the same title already exists, return an error
   "Title already exists in chapter".

5. **Update requirement**: Replace the existing level-2 ATX-style requirement heading and body with the new title (or keep existing)
   and new text. Keep the same index.

6. **Return result**: Return the full updated requirement data.

## G.REQLIX_U.4: Implementation details

The tool must read the category file, locate the requirement, replace its content in place, and write
the updated file. Preserve all other content unchanged.

## G.REQLIX_U.5: Response format

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

## G.REQLIX_U.6: Parameter validation

Before executing the update algorithm, the tool must validate all input parameters according to the constraints defined in [G.P.1](#gp1-parameter-constraints). If any parameter violates these constraints, the tool must return an error as specified in [G.P.2](#gp2-constraint-violation-error).

This validation must occur before any file system operations or requirement processing.
nt processing.
ing.
