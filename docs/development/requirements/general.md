
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
- `index` - required, max 100 characters per index. Can be:
  - Single string (e.g., "G.G.1")
  - Array of strings for batch operations (max 100 elements) in `reqlix_get_requirement` and `reqlix_delete_requirement`
- `text` - required, max 10000 characters
- `title` - required for `reqlix_insert_requirement`, optional for `reqlix_update_requirement`, max 100 characters
- `items` - array of update objects for batch `reqlix_update_requirement` (max 100 elements). Each object must satisfy constraints for `index`, `text`, and `title`.

## G.P.2: Constraint violation error

If any parameter does not satisfy the constraints, the tool must return an error.

## G.P.3: Name validation

Category and chapter names must satisfy the following validation rules:

**Category name validation:**
- Must not be empty (enforced by [G.P.1](#gp1-parameter-constraints) max length constraint)
- Must contain only lowercase English letters (a-z) and underscore (_)
- Must be a valid filename (cannot contain characters that are invalid in filenames: `/`, `\`, `:`, `*`, `?`, `"`, `<`, `>`, `|`)
- Must not be `AGENTS` (reserved name)
- Must not start or end with whitespace
- Must not contain consecutive dots (`.`)
- Must not be `.` or `..`

**Chapter name validation:**
- Must not be empty (enforced by [G.P.1](#gp1-parameter-constraints) max length constraint)
- Must contain only uppercase and lowercase English letters (A-Z, a-z), spaces, colons (:), hyphens (-), and underscores (_)
- Must not start or end with whitespace
- Must not contain newline characters (would break markdown heading structure)
- Must be a valid markdown heading content

If validation fails, the tool must return an error in the format specified in [G.C.6](#gc6-error-response-format) with a descriptive message indicating which validation rule was violated.

## G.P.4: Empty array handling

When a batch parameter (`index` as array or `items`) is an empty array `[]`, the tool must return success with an empty data array:

```json
{
  "success": true,
  "data": []
}
```

This is not considered an error.

# Requirements Storage Format

## G.R.1: Category definition

Category: a file `{category}.md` in the requirements directory (e.g., `general.md`, `testing.md`). The file format is markdown.

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

- `{category_prefix}` - First letter(s) of the category name (uppercase). Algorithm: if the category file already contains requirements, extract the prefix from an existing requirement index; otherwise, calculate a unique prefix that does not conflict with other category files by taking the first letter(s) and adding more letters until unique. **Only ASCII letters (A-Z, a-z) are considered for prefix calculation; all other characters (spaces, underscores, hyphens, colons, numbers, etc.) are ignored.**
- `{chapter_prefix}` - First letter(s) of the chapter name (uppercase). Algorithm: if the chapter already contains requirements, extract the prefix from an existing requirement index; otherwise, calculate a unique prefix that does not conflict with other chapters in the same category by taking the first letter(s) of the chapter name (using uppercase) and adding more letters until unique. **Only ASCII letters (A-Z, a-z) are considered for prefix calculation; all other characters (spaces, colons, hyphens, numbers, etc.) are ignored.**
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
- Category `general`, chapter `Chapter: Sub-Chapter Name` → G.C.1, G.C.2, ... (only letters "ChapterSubChapterName" are considered)

## G.R.5: Requirement parsing boundaries

When parsing requirements from markdown files, tools must correctly identify requirement boundaries:

A requirement starts with a level-2 ATX-style heading (see [G.R.3](#gr3-requirement-definition)). The requirement body is the content of this markdown section. The markdown parser automatically determines section boundaries (section ends at the next heading of the same or higher level, or at end of file).

## G.R.8: File encoding

All requirement files must be encoded in UTF-8. All tools must read and write files using UTF-8 encoding. If a file cannot be read as UTF-8, the tool must return an error indicating encoding issues.

## G.R.9: File system error handling

All tools must handle file system errors gracefully. Common errors include:

- Permission denied: Return error "Permission denied: {path}"
- File not found: For read operations, return appropriate error (e.g., "Category not found", "Requirement not found"). For write operations, create files/directories as needed (see [G.C.2](#gc2-directory-creation))
- Disk full: Return error "Disk full: cannot write to {path}"
- Invalid path: Return error "Invalid path: {path}"
- Encoding errors: Return error "Encoding error: file is not valid UTF-8"

All file system errors must be returned in the JSON error format specified in [G.C.6](#gc6-error-response-format).

## G.R.10: Empty file handling

Empty files must be handled as follows:

- **Empty category file**: An empty category file (containing only whitespace or no content) is considered valid. It has no chapters and no requirements. Tools must return empty arrays for chapters and requirements when querying an empty category file.

- **Category file with only whitespace**: Files containing only whitespace (spaces, tabs, newlines) are treated as empty files.

- **Chapter with no requirements**: A chapter that exists but contains no requirements (only the level-1 heading) is valid. Tools must return an empty requirements array for such chapters.

- **File creation**: When creating a new category file, it must be created as an empty file (or with only the initial chapter heading if a chapter is being added).

## G.R.11: Blank line before headings

When writing requirements to files, there must always be a blank line between the requirement text and the next heading (level-1 or level-2).

**Correct:**
```markdown
Requirement text content.

## G.G.2: Next requirement
```

**Incorrect:**
```markdown
Requirement text content.
## G.G.2: Next requirement
```

This ensures proper markdown rendering and readability.

## G.R.12: Exact heading match

When searching for chapters or requirements by name/index, tools must use exact heading match after proper markdown parsing, not substring search.

**Correct approach:**
1. Parse file line by line
2. Identify headings using markdown parser
3. Extract heading text and compare exactly

**Incorrect approach:**
- Using `content.find("# ChapterName")` which may match `# ChapterName` as substring of `# ChapterNameExtended`

This prevents bugs where chapter "Foo" is incorrectly matched when searching in a file containing both "# Foobar" and "# Foo".

# Tool: reqlix_get_instructions

## G.REQLIX_GET_I.1: Description

Description (shown to LLM in tool list):

```
CALL THIS BEFORE ANY CODE OPERATION (reading or writing). 
Returns instructions on how to work with requirements.
This MCP server is the single source of truth for everything related to requirements.

Returns JSON with "success": true and "data": {"content": "..."} containing instructions and categories list.
On error, returns JSON with "success": false and "error": "error message".
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

Returns JSON with "success": true and "data": {"categories": [...]} (alphabetically sorted).
If no categories exist, returns empty array: "categories": [].
On error, returns JSON with "success": false and "error": "error message".
```

## G.REQLIX_GET_CA.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.

## G.REQLIX_GET_CA.3: Response format

Returns a list of category names derived from `*.md` file names in the requirements directory
(excluding AGENTS.md), sorted alphabetically.

Success:

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

Error: Use error format from [G.C.6](#gc6-error-response-format).

# Tool: reqlix_get_chapters

## G.REQLIX_GET_CH.1: Description

Description (shown to LLM in tool list):

```
Returns a list of all chapters in the specified category.
Use this to discover what chapters exist in a category before querying requirements.

Returns JSON with "success": true and "data": {"category": "...", "chapters": [...]}.
If category has no chapters, returns empty array: "chapters": [].
On error (category not found), returns JSON with "success": false and "error": "error message".
```

## G.REQLIX_GET_CH.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `category` (string, required) - Category key (e.g., "general", "testing").

## G.REQLIX_GET_CH.3: Implementation details

The tool must parse level-1 ATX-style headings according to [G.R.2](#gr2-chapter-definition).

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

# Tool: reqlix_get_requirements

## G.REQLIX_GET_REQUIREMENTS.1: Description

Description (shown to LLM in tool list):

```
Returns a list of all requirement titles (with indices) in the specified category and chapter.
Use this to browse requirements in a chapter. To get full requirement content, use reqlix_get_requirement.

Returns JSON with "success": true and "data": {"category": "...", "chapter": "...", "requirements": [{"index": "...", "title": "..."}, ...]}.
If chapter has no requirements, returns empty array: "requirements": [].
On error (category/chapter not found), returns JSON with "success": false and "error": "error message".
```

## G.REQLIX_GET_REQUIREMENTS.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `category` (string, required) - Category key (e.g., "general", "testing").
- `chapter` (string, required) - Chapter name (e.g., "General Requirements", "Unit Tests").

## G.REQLIX_GET_REQUIREMENTS.3: Implementation details

The tool must parse requirements according to [G.R.3](#gr3-requirement-definition) within the specified chapter (see [G.R.2](#gr2-chapter-definition), [G.R.5](#gr5-requirement-parsing-boundaries)).

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

# Tool: reqlix_get_requirement

## G.REQLIX_GET_REQUIREMENT.1: Description

Description (shown to LLM in tool list):

```
Returns the full content (title and text) of one or more requirements by index.
Index format: {CATEGORY}.{CHAPTER}.{NUMBER} (e.g., G.G.1, T.U.2).
Supports batch requests with up to 100 indices.

Single request: Returns JSON with "success": true and "data": {...}.
On error, returns JSON with "success": false and "error": "error message".

Batch request: Returns JSON with "success": true and "data": [{...}, ...].
Each element in the array has its own "success" and "data" or "error" field.
```

## G.REQLIX_GET_REQUIREMENT.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `index` (string | string[], required) - Requirement index or array of indices (max 100). Example: "G.G.1" or ["G.G.1", "G.G.2", "T.U.1"].

## G.REQLIX_GET_REQUIREMENT.3: Index parsing and file lookup

The tool must parse the index according to [G.R.4](#gr4-index-format) by splitting on dots (`.`).

**Single index (string):**
1. Use algorithm from [G.C.7](#gc7-category-lookup-by-prefix) to find category by prefix
2. Find the requirement by full index in the category file (see [G.R.3](#gr3-requirement-definition), [G.R.5](#gr5-requirement-parsing-boundaries)). Return both the title (extracted from the heading content) and body text.
3. If requirement not found, return error "Requirement not found"

**Batch request (array of strings):**
1. Validate array length does not exceed 100 (see [G.REQLIX_GET_REQUIREMENT.5](#greqlix_get_requirement5-batch-request-limit))
2. Process **all** indices in order using the single index algorithm
3. For each index, return either success result or error object
4. Return array of results in the same order as input indices (each element is either success data or error object)

## G.REQLIX_GET_REQUIREMENT.4: Response format

**Single request success:**

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

**Batch request (always returns array, each element has its own success/error):**

```json
{
  "success": true,
  "data": [
    {
      "success": true,
      "data": {
        "index": "G.G.1",
        "title": "Language requirement",
        "text": "All requirements must be written in English.",
        "category": "general",
        "chapter": "General Requirements"
      }
    },
    {
      "success": false,
      "error": "Requirement not found"
    },
    {
      "success": true,
      "data": {
        "index": "G.G.3",
        "title": "Another requirement",
        "text": "Requirement body text.",
        "category": "general",
        "chapter": "General Requirements"
      }
    }
  ]
}
```

**Single request error** (requirement not found): Use error format from [G.C.6](#gc6-error-response-format).

## G.REQLIX_GET_REQUIREMENT.5: Batch request limit

When `index` parameter is an array, the maximum number of indices allowed is **100**.

If more than 100 indices are provided, return error: "Batch request exceeds maximum limit of 100 indices".

# Tool: reqlix_insert_requirement

## G.REQLIX_I.1: Description

Description (shown to LLM in tool list):

```
Inserts a new requirement into the specified category and chapter.
The title must be generated by the LLM and provided as a parameter.
The title must be a concise name that reflects the essence of the requirement.

Returns JSON with "success": true and "data": {"index": "...", "title": "...", "text": "...", "category": "...", "chapter": "..."}.
On error (title already exists, file system error, validation error), returns JSON with "success": false and "error": "error message".
```

## G.REQLIX_I.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `category` (string, required) - Category key (e.g., "general", "testing").
- `chapter` (string, required) - Chapter name (e.g., "General Requirements", "Unit Tests").
- `text` (string, required) - Requirement text (body content).
- `title` (string, required) - Requirement title. A concise name that reflects the essence of the requirement.

## G.REQLIX_I.3: Algorithm

The tool must execute the following steps:

0. **Validate parameters**: Validate all input parameters according to [G.REQLIX_I.6](#greqlix_i6-parameter-validation).

1. **Find or create category**: Locate the category file `{category}.md`. If not found, create a new empty file.

2. **Find or create chapter**: Search for a chapter heading matching the chapter name (see [G.R.2](#gr2-chapter-definition)). If not found, append a chapter heading to the end of the file.

3. **Validate title uniqueness**: Check that the title is unique within the chapter (see [G.R.3](#gr3-requirement-definition)). If a requirement with the same title already exists, return an error "Title already exists in chapter".

4. **Generate index**: Create the requirement index according to [G.R.4](#gr4-index-format). Reuse existing prefixes when available, otherwise calculate unique prefixes.

5. **Insert requirement**: Append a requirement heading with content `{index}: {title}` followed by the requirement text (see [G.R.3](#gr3-requirement-definition)).

6. **Return result**: Return the full requirement data.

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

# Tool: reqlix_update_requirement

## G.REQLIX_U.1: Description

Description (shown to LLM in tool list):

```
Updates one or more existing requirements by index with new text and optional new title.
If title is provided, it must be unique within the chapter. If not provided, the existing title is kept.
Supports batch updates with up to 100 requirements.

Category must contain only lowercase English letters (a-z) and underscore (_).
Chapter must contain only uppercase and lowercase English letters (A-Z, a-z), spaces, colons (:), and hyphens (-).

Single update: Returns JSON with "success": true and "data": {...}.
On error, returns JSON with "success": false and "error": "error message".

Batch update: Returns JSON with "success": true and "data": [{...}, ...].
Each element in the array has its own "success" and "data" or "error" field.
```

## G.REQLIX_U.2: Parameters

Parameters:

**Single update:**
- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `index` (string, required) - Requirement index (e.g., "G.G.1", "T.U.2").
- `text` (string, required) - New requirement text (body content).
- `title` (string, optional) - New requirement title. If provided, must be unique within the chapter.

**Batch update:**
- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `items` (array, required) - Array of update objects (max 100). Each object contains:
  - `index` (string, required) - Requirement index.
  - `text` (string, required) - New requirement text.
  - `title` (string, optional) - New requirement title.

Note: Use either `index`+`text`+`title` for single update OR `items` for batch update, not both.

## G.REQLIX_U.3: Algorithm

The tool must execute the following steps:

**Single update (when `index` parameter is provided):**

1. **Validate parameters**: Validate all input parameters according to [G.REQLIX_U.6](#greqlix_u6-parameter-validation).

2. **Parse index**: Extract category prefix, chapter prefix, and requirement number from the index
   (see [G.R.4](#gr4-index-format)).

3. **Find requirement**: Locate the requirement by its index (see [G.REQLIX_GET_REQUIREMENT.3](#greqlix_get_requirement3-index-parsing-and-file-lookup)). If not found, return error.

4. **Determine new title**: If `title` parameter is provided, use it. Otherwise, keep the existing title.

5. **Validate title uniqueness**: If a new title was provided, check that it is unique within the chapter
   (excluding the current requirement) (see [G.R.3](#gr3-requirement-definition)). If a requirement with the same title already exists, return an error
   "Title already exists in chapter".

6. **Update requirement**: Replace the existing requirement heading and body with the new title (or keep existing)
   and new text (see [G.R.3](#gr3-requirement-definition)). Keep the same index.

7. **Return result**: Return the full updated requirement data.

**Batch update (when `items` parameter is provided):**

1. **Validate batch size**: Ensure `items` array length does not exceed 100 (see [G.REQLIX_U.7](#greqlix_u7-batch-update-limit)).

2. **Process all items**: For each item in the array, execute steps 1-6 from single update algorithm.

3. For each item, return either success result or error object.

4. **Return results**: Return array of results in the same order as input items (each element is either success data or error object).

## G.REQLIX_U.4: Response format

**Single update success:**

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

**Batch update (always returns array, each element has its own success/error):**

```json
{
  "success": true,
  "data": [
    {
      "success": true,
      "data": {
        "index": "G.G.1",
        "title": "Updated title 1",
        "text": "Updated text 1...",
        "category": "general",
        "chapter": "General Requirements"
      }
    },
    {
      "success": false,
      "error": "Requirement not found"
    }
  ]
}
```

**Single update error** (requirement not found, file system error, title already exists, validation error): Use error format from [G.C.6](#gc6-error-response-format).

## G.REQLIX_U.6: Parameter validation

Before executing the update algorithm, the tool must validate all input parameters according to the constraints defined in [G.P.1](#gp1-parameter-constraints). If any parameter violates these constraints, the tool must return an error as specified in [G.P.2](#gp2-constraint-violation-error).

This validation must occur before any file system operations or requirement processing.

## G.REQLIX_U.7: Batch update limit

When `items` parameter is provided, the maximum number of items allowed is **100**.

If more than 100 items are provided, return error: "Batch update exceeds maximum limit of 100 items".

# Tool: reqlix_get_version

## G.TOOLREQLIXGETV.1: Description

Description (shown to LLM in tool list):

```
Returns the version of the reqlix MCP server.
Use this to check which version of the server is running.
This tool has no parameters.

Returns JSON with "success": true and "data": {"version": "x.y.z"}.
```
## G.TOOLREQLIXGETV.2: Response format

Success:

```json
{
  "success": true,
  "data": {
    "version": "0.1.0"
  }
}
```

This tool always succeeds and does not return errors.
## G.TOOLREQLIXGETV.3: Implementation details

The tool must return the version string from `Cargo.toml` using the `env!("CARGO_PKG_VERSION")` macro at compile time.

This tool has no parameters and does not require validation.

# Tool: reqlix_delete_requirement

## G.TOOLREQLIXD.1: Description

Description (shown to LLM in tool list):

```
Deletes one or more existing requirements by index.
The requirements will be permanently removed from the category file.
Supports batch deletions with up to 100 indices.

Single delete: Returns JSON with "success": true and "data": {...}.
On error, returns JSON with "success": false and "error": "error message".

Batch delete: Returns JSON with "success": true and "data": [{...}, ...].
Each element in the array has its own "success" and "data" or "error" field.
```

## G.TOOLREQLIXD.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `index` (string | string[], required) - Requirement index or array of indices to delete (max 100). Example: "G.G.1" or ["G.G.1", "G.G.2", "T.U.1"].

## G.TOOLREQLIXD.3: Algorithm

The tool must execute the following steps:

**Single delete (when `index` is a string):**

1. **Validate parameters**: Validate all input parameters according to [G.TOOLREQLIXD.5](#gtoolreqlixd5-parameter-validation).

2. **Parse index**: Extract category prefix, chapter prefix, and requirement number from the index (see [G.R.4](#gr4-index-format)).

3. **Find requirement**: Locate the requirement by its index (see [G.REQLIX_GET_REQUIREMENT.3](#greqlix_get_requirement3-index-parsing-and-file-lookup)). If not found, return error "Requirement not found".

4. **Delete requirement**: Remove the requirement heading and body from the category file. The requirement boundaries are determined according to [G.R.5](#gr5-requirement-parsing-boundaries).

5. **Delete empty chapter**: If the chapter becomes empty after deleting the requirement (no more requirements in the chapter), remove the chapter heading from the category file.

6. **Return result**: Return the deleted requirement metadata (index, title, category, chapter).

**Batch delete (when `index` is an array):**

1. **Validate batch size**: Ensure array length does not exceed 100 (see [G.TOOLREQLIXD.6](#gtoolreqlixd6-batch-delete-limit)).

2. **Process all indices**: For each index in the array, execute steps 1-5 from single delete algorithm.

3. For each index, return either success result or error object.

4. **Return results**: Return array of results in the same order as input indices (each element is either success data or error object).

## G.TOOLREQLIXD.4: Response format

**Single delete success:**

```json
{
  "success": true,
  "data": {
    "index": "G.G.1",
    "title": "Deleted requirement title",
    "category": "general",
    "chapter": "General Requirements"
  }
}
```

**Batch delete (always returns array, each element has its own success/error):**

```json
{
  "success": true,
  "data": [
    {
      "success": true,
      "data": {
        "index": "G.G.1",
        "title": "Deleted title 1",
        "category": "general",
        "chapter": "General Requirements"
      }
    },
    {
      "success": false,
      "error": "Requirement not found"
    }
  ]
}
```

**Single delete error** (requirement not found, file system error, validation error): Use error format from [G.C.6](#gc6-error-response-format).

## G.TOOLREQLIXD.5: Parameter validation

Before executing the deletion algorithm, the tool must validate all input parameters according to the constraints defined in [G.P.1](#gp1-parameter-constraints). If any parameter violates these constraints, the tool must return an error as specified in [G.P.2](#gp2-constraint-violation-error).

This validation must occur before any file system operations or requirement processing.

## G.TOOLREQLIXD.6: Batch delete limit

When `index` parameter is an array, the maximum number of indices allowed is **100**.

If more than 100 indices are provided, return error: "Batch delete exceeds maximum limit of 100 indices".

# Configuration

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
