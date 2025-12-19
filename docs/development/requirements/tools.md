# reqlix_get_instructions

## T.R.1: Description

Description (shown to LLM in tool list):

```
CALL THIS BEFORE ANY CODE OPERATION (reading or writing). 
Returns instructions on how to work with requirements, including a MANDATORY workflow that MUST be followed for all code modifications.
This MCP server is the single source of truth for everything related to requirements.

Returns JSON with "success": true and "data": {"content": "..."} containing instructions with a mandatory workflow.
On error, returns JSON with "success": false and "error": "error message".
```

## T.R.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.

## T.R.3: Requirements file search order

The tool must locate the requirements file using the following search order (check each path in order,
proceed to next if file not found):

1. `{project_root}/{REQLIX_REQ_REL_PATH}/AGENTS.md` (if environment variable `REQLIX_REQ_REL_PATH` is set)
2. `{project_root}/docs/development/requirements/AGENTS.md`
3. `{project_root}/docs/dev/req/AGENTS.md`

## T.R.4: Requirements file creation

If no file is found, the tool must create a file with placeholder content at:

1. `{project_root}/{REQLIX_REQ_REL_PATH}/AGENTS.md` (if environment variable `REQLIX_REQ_REL_PATH` is set)
2. `{project_root}/docs/development/requirements/AGENTS.md` (if `REQLIX_REQ_REL_PATH` is not set)

## T.R.5: Error handling

If file creation fails or a permission error occurs at any stage, the tool must return an error.

## T.R.6: Placeholder content

Placeholder content for new requirements file.

The placeholder `{requirements_directory}` must be replaced with the relative path to the requirements
directory from the project root at runtime (e.g., `docs/development/requirements` or `{REQLIX_REQ_REL_PATH}` if set):

```
# Instructions

These instructions are MANDATORY for all code operations. You MUST follow them strictly.

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

# MANDATORY WORKFLOW FOR CODE CHANGES

**⚠️ CRITICAL: You MUST follow this 11-step workflow for ALL code modifications. Skipping steps is NOT allowed.**

Before making any code changes, you MUST complete this entire workflow. Inform the user about each step and its results:

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

**Remember: This workflow is MANDATORY. Do not skip any steps.**

```

## T.R.7: Response format

```json
{
  "success": true,
  "data": {
    "content": "# Instructions\n\nThese instructions are mandatory..."
  }
}
```

# reqlix_get_categories

## T.REQLIXGETC.1: Description

Description (shown to LLM in tool list):

```
Returns a list of all available requirement categories.
Use this to discover what categories exist before querying chapters or requirements.

Returns JSON with "success": true and "data": {"categories": [...]} (alphabetically sorted).
If no categories exist, returns empty array: "categories": [].
On error, returns JSON with "success": false and "error": "error message".
```

## T.REQLIXGETC.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.

## T.REQLIXGETC.3: Response format

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

Error: Use error format from [C.C.6](#cc6-error-response-format).

# reqlix_get_chapters

## T.REQLIXGETCH.1: Description

Description (shown to LLM in tool list):

```
Returns a list of all chapters in the specified category.
Use this to discover what chapters exist in a category before querying requirements.

Returns JSON with "success": true and "data": {"category": "...", "chapters": [...]}.
If category has no chapters, returns empty array: "chapters": [].
On error (category not found), returns JSON with "success": false and "error": "error message".
```

## T.REQLIXGETCH.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `category` (string, required) - Category key (e.g., "general", "testing").

## T.REQLIXGETCH.3: Implementation details

The tool must parse level-1 ATX-style headings according to [G.R.2](#gr2-chapter-definition).

## T.REQLIXGETCH.4: Response format

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

Error (category not found): Use error format from [C.C.6](#cc6-error-response-format).

# reqlix_get_requirements

## T.REQLIXGETR.1: Description

Description (shown to LLM in tool list):

```
Returns a list of all requirement titles (with indices) in the specified category and chapter.
Use this to browse requirements in a chapter. To get full requirement content, use reqlix_get_requirement.

Returns JSON with "success": true and "data": {"category": "...", "chapter": "...", "requirements": [{"index": "...", "title": "..."}, ...]}.
If chapter has no requirements, returns empty array: "requirements": [].
On error (category/chapter not found), returns JSON with "success": false and "error": "error message".
```

## T.REQLIXGETR.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `category` (string, required) - Category key (e.g., "general", "testing").
- `chapter` (string, required) - Chapter name (e.g., "General Requirements", "Unit Tests").

## T.REQLIXGETR.3: Implementation details

The tool must parse requirements according to [G.R.3](#gr3-requirement-definition) within the specified chapter (
see [G.R.2](#gr2-chapter-definition), [G.R.5](#gr5-requirement-parsing-boundaries)).

## T.REQLIXGETR.4: Response format

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

Errors (category/chapter not found): Use error format from [C.C.6](#cc6-error-response-format).

# reqlix_get_requirement

## T.REQLIXGETREQUIREMENT.1: Description

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

## T.REQLIXGETREQUIREMENT.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `index` (string | string[], required) - Requirement index or array of indices (max 100). Example: "G.G.1"
  or ["G.G.1", "G.G.2", "T.U.1"].

## T.REQLIXGETREQUIREMENT.3: Index parsing and file lookup

The tool must parse the index according to [G.R.4](#gr4-index-format) by splitting on dots (`.`).

**Single index (string):**

1. Use algorithm from [C.C.7](#cc7-category-lookup-by-prefix) to find category by prefix
2. Find the requirement by full index in the category file (
   see [G.R.3](#gr3-requirement-definition), [G.R.5](#gr5-requirement-parsing-boundaries)). Return both the title (
   extracted from the heading content) and body text.
3. If requirement not found, return error "Requirement not found"

**Batch request (array of strings):**

1. Validate array length does not exceed 100 (
   see [T.REQLIXGETREQUIREMENT.5](#treqlixgetrequirement5-batch-request-limit))
2. Process **all** indices in order using the single index algorithm
3. For each index, return either success result or error object
4. Return array of results in the same order as input indices (each element is either success data or error object)

## T.REQLIXGETREQUIREMENT.4: Response format

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

**Single request error** (requirement not found): Use error format from [C.C.6](#cc6-error-response-format).

## T.REQLIXGETREQUIREMENT.5: Batch request limit

When `index` parameter is an array, the maximum number of indices allowed is **100**.

If more than 100 indices are provided, return error: "Batch request exceeds maximum limit of 100 indices".

# reqlix_insert_requirement

## T.REQLIXI.1: Description

Description (shown to LLM in tool list):

```
Inserts a new requirement into the specified category and chapter.
The title must be generated by the LLM and provided as a parameter.
The title must be a concise name that reflects the essence of the requirement.

Returns JSON with "success": true and "data": {"index": "...", "title": "...", "text": "...", "category": "...", "chapter": "..."}.
On error (title already exists, file system error, validation error), returns JSON with "success": false and "error": "error message".
```

## T.REQLIXI.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `category` (string, required) - Category key (e.g., "general", "testing").
- `chapter` (string, required) - Chapter name (e.g., "General Requirements", "Unit Tests").
- `text` (string, required) - Requirement text (body content).
- `title` (string, required) - Requirement title. A concise name that reflects the essence of the requirement.

## T.REQLIXI.3: Algorithm

The tool must execute the following steps:

0. **Validate parameters**: Validate all input parameters according to [T.REQLIXI.5](#treqlixi5-parameter-validation).

1. **Find or create category**: Locate the category file `{category}.md`. If not found, create a new empty file.

2. **Find or create chapter**: Search for a chapter heading matching the chapter name (
   see [G.R.2](#gr2-chapter-definition)). If not found, append a chapter heading to the end of the file.

3. **Validate title uniqueness**: Check that the title is unique within the chapter (
   see [G.R.3](#gr3-requirement-definition)). If a requirement with the same title already exists, return an error "
   Title already exists in chapter".

4. **Generate index**: Create the requirement index according to [G.R.4](#gr4-index-format). Reuse existing prefixes
   when available, otherwise calculate unique prefixes.

5. **Insert requirement**: Append a requirement heading with content `{index}: {title}` followed by the requirement
   text (see [G.R.3](#gr3-requirement-definition)).

6. **Return result**: Return the full requirement data.

## T.REQLIXI.4: Response format

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

Errors (file system error, title already exists): Use error format from [C.C.6](#cc6-error-response-format).

## T.REQLIXI.5: Parameter validation

Before executing the insertion algorithm, the tool must validate all input parameters according to the constraints
defined in [G.P.1](#gp1-parameter-constraints). If any parameter violates these constraints, the tool must return an
error as specified in [G.P.2](#gp2-constraint-violation-error).

This validation must occur before any file system operations or requirement processing.

## T.REQLIXI.6: Embedding calculation and storage

When inserting a new requirement, the tool must calculate an embedding vector from the requirement text (title + text combined) using the paraphrase-MiniLM-L3-v2 model and store it as an embedding comment immediately after the requirement heading according to G.R.11.

The embedding must be calculated from the combined text: "{title}: {text}" (title, colon, space, then text).

The embedding comment must be inserted on a separate line right after the requirement heading line, before the requirement text.

If embedding calculation fails, the tool must return an error and abort the insertion operation.

# reqlix_update_requirement

## T.REQLIXU.1: Description

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

## T.REQLIXU.2: Parameters

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

## T.REQLIXU.3: Algorithm

The tool must execute the following steps:

**Single update (when `index` parameter is provided):**

1. **Validate parameters**: Validate all input parameters according to [T.REQLIXU.5](#treqlixu5-parameter-validation).

2. **Parse index**: Extract category prefix, chapter prefix, and requirement number from the index
   (see [G.R.4](#gr4-index-format)).

3. **Find requirement**: Locate the requirement by its index (
   see [T.REQLIXGETREQUIREMENT.3](#treqlixgetrequirement3-index-parsing-and-file-lookup)). If not found, return
   error.

4. **Determine new title**: If `title` parameter is provided, use it. Otherwise, keep the existing title.

5. **Validate title uniqueness**: If a new title was provided, check that it is unique within the chapter
   (excluding the current requirement) (see [G.R.3](#gr3-requirement-definition)). If a requirement with the same title
   already exists, return an error
   "Title already exists in chapter".

6. **Update requirement**: Replace the existing requirement heading and body with the new title (or keep existing)
   and new text (see [G.R.3](#gr3-requirement-definition)). Keep the same index.

7. **Return result**: Return the full updated requirement data.

**Batch update (when `items` parameter is provided):**

1. **Validate batch size**: Ensure `items` array length does not exceed 100 (
   see [T.REQLIXU.6](#treqlixu6-batch-update-limit)).

2. **Process all items**: For each item in the array, execute steps 1-6 from single update algorithm.

3. For each item, return either success result or error object.

4. **Return results**: Return array of results in the same order as input items (each element is either success data or
   error object).

## T.REQLIXU.4: Response format

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

**Single update error** (requirement not found, file system error, title already exists, validation error): Use error
format from [C.C.6](#cc6-error-response-format).

## T.REQLIXU.5: Parameter validation

Before executing the update algorithm, the tool must validate all input parameters according to the constraints defined
in [G.P.1](#gp1-parameter-constraints). If any parameter violates these constraints, the tool must return an error as
specified in [G.P.2](#gp2-constraint-violation-error).

This validation must occur before any file system operations or requirement processing.

## T.REQLIXU.6: Batch update limit

When `items` parameter is provided, the maximum number of items allowed is **100**.

If more than 100 items are provided, return error: "Batch update exceeds maximum limit of 100 items".

## T.REQLIXU.7: Embedding recalculation and update

When updating a requirement, the tool must recalculate the embedding vector from the updated requirement text (new title + new text combined) using the paraphrase-MiniLM-L3-v2 model and update the embedding comment according to G.R.11.

The embedding must be recalculated whenever the requirement is updated, even if only the title changes and the text remains unchanged.

If an embedding comment already exists, it must be replaced with the new embedding. If no embedding comment exists, one must be added.

The embedding must be calculated from the combined text: "{title}: {text}" (title, colon, space, then text).

If embedding calculation fails, the tool must return an error and abort the update operation.

# reqlix_delete_requirement

## T.REQLIXD.1: Description

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

## T.REQLIXD.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `index` (string | string[], required) - Requirement index or array of indices to delete (max 100). Example: "G.G.1"
  or ["G.G.1", "G.G.2", "T.U.1"].

## T.REQLIXD.3: Algorithm

The tool must execute the following steps:

**Single delete (when `index` is a string):**

1. **Validate parameters**: Validate all input parameters according
   to [T.REQLIXD.5](#treqlixd5-parameter-validation).

2. **Parse index**: Extract category prefix, chapter prefix, and requirement number from the index (
   see [G.R.4](#gr4-index-format)).

3. **Find requirement**: Locate the requirement by its index (
   see [T.REQLIXGETREQUIREMENT.3](#treqlixgetrequirement3-index-parsing-and-file-lookup)). If not found, return
   error "Requirement not found".

4. **Delete requirement**: Remove the requirement heading and body from the category file. The requirement boundaries
   are determined according to [G.R.5](#gr5-requirement-parsing-boundaries).

5. **Delete empty chapter**: If the chapter becomes empty after deleting the requirement (no more requirements in the
   chapter), remove the chapter heading from the category file.

6. **Return result**: Return the deleted requirement metadata (index, title, category, chapter).

**Batch delete (when `index` is an array):**

1. **Validate batch size**: Ensure array length does not exceed 100 (
   see [T.REQLIXD.6](#treqlixd6-batch-delete-limit)).

2. **Process all indices**: For each index in the array, execute steps 1-5 from single delete algorithm.

3. For each index, return either success result or error object.

4. **Return results**: Return array of results in the same order as input indices (each element is either success data
   or error object).

## T.REQLIXD.4: Response format

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

**Single delete error** (requirement not found, file system error, validation error): Use error format
from [C.C.6](#cc6-error-response-format).

## T.REQLIXD.5: Parameter validation

Before executing the deletion algorithm, the tool must validate all input parameters according to the constraints
defined in [G.P.1](#gp1-parameter-constraints). If any parameter violates these constraints, the tool must return an
error as specified in [G.P.2](#gp2-constraint-violation-error).

This validation must occur before any file system operations or requirement processing.

## T.REQLIXD.6: Batch delete limit

When `index` parameter is an array, the maximum number of indices allowed is **100**.

If more than 100 indices are provided, return error: "Batch delete exceeds maximum limit of 100 indices".

# reqlix_search_requirements

## T.REQLIXS.1: Description

Description (shown to LLM in tool list):

```
Searches for requirements by keywords across all categories.
Accepts from 0 to 100 keywords. Each keyword max 200 characters.
Returns all requirements where the title or text contains at least one of the specified keywords.
Search is case-insensitive.

Returns JSON with "success": true and "data": {"keywords": [...], "results": [...]}.
If keywords array is empty, returns success with empty results array.
On error, returns JSON with "success": false and "error": "error message".
```

## T.REQLIXS.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `keywords` (string | string[], required) - Single keyword (max 200 characters) or array of keywords (0 to 100
  elements, each max 200 characters). Example: "auth" or ["auth", "user", "login"].

## T.REQLIXS.3: Search logic

Search algorithm:

1. Iterate over all categories in the requirements directory.
2. For each category, iterate over all chapters.
3. For each chapter, iterate over all requirements.
4. For each requirement, check if the title or text contains at least one of the keywords.
5. Search is case-insensitive (convert both keyword and content to lowercase before comparison).
6. A requirement matches if any keyword is found as a substring in the title OR text.
7. Collect all matching requirements and return them in the results array.

**Note:** The order of results is undefined and may change between calls. Do not rely on any specific ordering.

## T.REQLIXS.4: Response format

**Success response:**

```json
{
  "success": true,
  "data": {
    "keywords": [
      "auth",
      "user"
    ],
    "results": [
      {
        "index": "G.G.1",
        "title": "User authentication",
        "text": "All users must authenticate before accessing the system.",
        "category": "general",
        "chapter": "Security"
      },
      {
        "index": "G.G.2",
        "title": "Auth token format",
        "text": "Authentication tokens must be JWT format.",
        "category": "general",
        "chapter": "Security"
      }
    ]
  }
}
```

**No matches found (still success, empty results):**

```json
{
  "success": true,
  "data": {
    "keywords": [
      "nonexistent"
    ],
    "results": []
  }
}
```

**Empty keywords (returns success with empty results per G.P.4):**

```json
{
  "success": true,
  "data": {
    "keywords": [],
    "results": []
  }
}
```

**Error response** (validation error, file system error): Use error format from C.C.6.

## T.REQLIXS.5: Keywords limit

The tool accepts from 0 to 100 keywords, each max 200 characters:

- If keywords is an empty array `[]`, return success with empty results (per G.P.4)
- If keywords is an empty string `""`, treat as empty array and return success with empty results
- Maximum: 100 keywords (exceeding returns error: "Keywords count exceeds maximum limit of 100")
- Maximum keyword length: 200 characters (exceeding returns error)

Empty strings within the keywords array are filtered out before search. If after filtering all keywords are empty, treat
as empty array.

## T.REQLIXS.6: Parameter validation

Before executing the search algorithm, the tool must validate all input parameters according to the constraints defined
in G.P.1 and T.REQLIXS.5. If any parameter violates these constraints, the tool must return an error as specified in
G.P.2.

Validation order:

1. Validate `project_root` (required, max 1000 characters)
2. Validate `operation_description` (required, max 10000 characters)
3. Validate `keywords` (max 100 elements, each max 200 characters)

This validation must occur before any file system operations or requirement processing.

## T.REQLIXS.7: Ignoring embedding comments in keyword search

The keyword search tool must ignore embedding comments when searching for keywords in requirement content. Embedding comments (format defined in G.R.11) must not be included in the searchable text.

When checking if a requirement matches keywords:
- Extract requirement text according to G.R.12 (which excludes embedding comments)
- Search only in the title and text fields, excluding any embedding comments
- Embedding comments must not be matched by keyword search

# reqlix_get_version

## T.REQLIXGETV.1: Description

Description (shown to LLM in tool list):

```
Returns the version of the reqlix MCP server.
Use this to check which version of the server is running.
This tool has no parameters.

Returns JSON with "success": true and "data": {"version": "x.y.z"}.
```

## T.REQLIXGETV.2: Response format

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

## T.REQLIXGETV.3: Implementation details

The tool must return the version string from `Cargo.toml` using the `env!("CARGO_PKG_VERSION")` macro at compile time.

This tool has no parameters and does not require validation.

# reqlix_fuzzy_search_requirements

## T.REQLIXF.1: Description

Description (shown to LLM in tool list):

```
Searches for requirements using semantic similarity (fuzzy search) across all categories.
Uses embedding vectors to find requirements semantically similar to the query text.
Returns requirements ordered by similarity score (most similar first).
Accepts a query string (max 10000 characters) and optional limit parameter (default: 10, max: 1000).

Returns JSON with "success": true and "data": {"query": "...", "results": [...]}.
Each result includes a similarity score (0.0 to 1.0, higher is more similar).
On error, returns JSON with "success": false and "error": "error message".
```

## T.REQLIXF.2: Parameters

Parameters:

- `project_root` (string, required) - Path to the project root directory.
- `operation_description` (string, required) - Brief description of the operation that LLM intends to perform.
- `query` (string, required) - Search query text (max 10000 characters). The tool will find requirements semantically similar to this query.
- `limit` (integer, optional) - Maximum number of results to return. Default: 10. Must be between 1 and 1000.

## T.REQLIXF.3: Search algorithm

Search algorithm:

1. Collect all embedding comments from all requirement files efficiently without parsing markdown structure. Use regex or string matching to find lines matching the pattern `<!--embedding:<model_name>:<base64_vector>-->`.

2. For each embedding comment found:
   - Extract the base64-encoded vector (ignore the model name from the comment, as per G.R.11)
   - Decode the vector
   - If decoding fails (invalid base64, wrong length, etc.), silently ignore the error and treat the requirement as if it has no embedding (skip it, do not include in search results)
   - If decoding succeeds, store mapping: vector -> requirement index (extracted from the requirement heading immediately preceding the embedding comment)
   
   **Note:** Requirements without embedding comments are excluded from search results. Requirements with invalid or unparseable embedding vectors are also excluded (parsing errors are silently ignored).

3. Calculate embedding vector for the query text using the paraphrase-MiniLM-L3-v2 model. The model name stored in embedding comments is ignored - always use paraphrase-MiniLM-L3-v2 for query embedding calculation (see G.R.11).

4. Calculate cosine similarity between query embedding and each requirement embedding.

5. Sort requirements by similarity score (highest first).

6. Apply limit parameter (default: 10, max: 1000) to restrict the number of results returned.

7. Return matching requirements with their similarity scores (up to the specified limit).

**Note:** The model must be embedded in the binary and loaded lazily on first use, then reused for all subsequent operations (see G.R.13).

## T.REQLIXF.4: Response format

**Success response:**

```json
{
  "success": true,
  "data": {
    "query": "user authentication",
    "results": [
      {
        "index": "G.G.1",
        "title": "User authentication",
        "text": "All users must authenticate before accessing the system.",
        "category": "general",
        "chapter": "Security",
        "similarity": 0.95
      },
      {
        "index": "G.G.2",
        "title": "Auth token format",
        "text": "Authentication tokens must be JWT format.",
        "category": "general",
        "chapter": "Security",
        "similarity": 0.87
      }
    ]
  }
}
```

**No matches found (still success, empty results):**

```json
{
  "success": true,
  "data": {
    "query": "nonexistent concept",
    "results": []
  }
}
```

**Error response** (validation error, file system error, embedding model error): Use error format from C.C.6.

Each result includes a `similarity` field (float, 0.0 to 1.0) indicating how similar the requirement is to the query. Higher values indicate greater similarity. Results are ordered by similarity (highest first).

## T.REQLIXF.5: Parameter validation

Before executing the search algorithm, the tool must validate all input parameters according to the constraints defined in G.P.1. If any parameter violates these constraints, the tool must return an error as specified in G.P.2.

Validation order:

1. Validate `project_root` (required, max 1000 characters)
2. Validate `operation_description` (required, max 10000 characters)
3. Validate `query` (required, max 10000 characters)
4. Validate `limit` (optional, default: 10, must be between 1 and 1000)

This validation must occur before any file system operations or requirement processing.
