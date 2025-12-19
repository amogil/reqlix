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
    - Array of strings for batch operations (max 100 elements) in `reqlix_get_requirement` and
      `reqlix_delete_requirement`
- `text` - required, max 10000 characters
- `title` - required for `reqlix_insert_requirement`, optional for `reqlix_update_requirement`, max 100 characters
- `items` - array of update objects for batch `reqlix_update_requirement` (max 100 elements). Each object must satisfy
  constraints for `index`, `text`, and `title`.
- `keywords` - required for `reqlix_search_requirements`, max 200 characters per keyword. Can be:
    - Single string (e.g., "auth")
    - Array of strings (max 100 elements)
- `query` - required for `reqlix_fuzzy_search_requirements`, max 10000 characters
- `limit` - optional for `reqlix_fuzzy_search_requirements`, integer between 1 and 1000, default: 10

## G.P.2: Constraint violation error

If any parameter does not satisfy the constraints, the tool must return an error.

## G.P.3: Name validation

Category and chapter names must satisfy the following validation rules:

**Category name validation:**

- Must not be empty (enforced by [G.P.1](#gp1-parameter-constraints) max length constraint)
- Must contain only lowercase English letters (a-z) and underscore (_)
- Must be a valid filename (cannot contain characters that are invalid in filenames: `/`, `\`, `:`, `*`, `?`, `"`, `<`,
  `>`, `|`)
- Must not be `AGENTS` (reserved name)
- Must not start or end with whitespace
- Must not contain consecutive dots (`.`)
- Must not be `.` or `..`

**Chapter name validation:**

- Must not be empty (enforced by [G.P.1](#gp1-parameter-constraints) max length constraint)
- Must contain only uppercase and lowercase English letters (A-Z, a-z), spaces, colons (:), hyphens (-), and
  underscores (_)
- Must not start or end with whitespace
- Must not contain newline characters (would break markdown heading structure)
- Must be a valid markdown heading content

If validation fails, the tool must return an error in the format specified in [C.C.6](#cc6-error-response-format) with a
descriptive message indicating which validation rule was violated.

## G.P.4: Empty array handling

When a batch parameter (`index` as array or `items`) is an empty array `[]`, the tool must return success with an empty
data array:

```json
{
  "success": true,
  "data": []
}
```

This is not considered an error.

# Requirements Storage Format

## G.R.1: Category definition

Category: a file `{category}.md` in the requirements directory (e.g., `general.md`, `testing.md`). The file format is
markdown.

## G.R.2: Chapter definition

Chapter: a level-1 ATX-style heading in markdown within a category file.

A chapter heading is an ATX-style heading with heading level 1 (one `#` character).
The heading content (text after the `#` and required space) is the chapter name. Example: `# Chapter Name` gives
`Chapter Name`

## G.R.3: Requirement definition

Requirement: a level-2 ATX-style heading in markdown. Format: `## {index}: {title}`

The requirement body is the content of this section, excluding the heading.

Example: `## G.G.1: Requirement title`

## G.R.4: Index format

Requirement index format: `{category_prefix}.{chapter_prefix}.{number}`

The dot (`.`) is the delimiter between parts. Each part is parsed by splitting the index on dots.

- `{category_prefix}` - First letter(s) of the category name (uppercase). Algorithm: if the category file already
  contains requirements, extract the prefix from an existing requirement index; otherwise, calculate a unique prefix
  that does not conflict with other category files by taking the first letter(s) and adding more letters until unique. *
  *Only ASCII letters (A-Z, a-z) are considered for prefix calculation; all other characters (spaces, underscores,
  hyphens, colons, numbers, etc.) are ignored.**
- `{chapter_prefix}` - First letter(s) of the chapter name (uppercase). Algorithm: if the chapter already contains
  requirements, extract the prefix from an existing requirement index; otherwise, calculate a unique prefix that does
  not conflict with other chapters in the same category by taking the first letter(s) of the chapter name (using
  uppercase) and adding more letters until unique. **Only ASCII letters (A-Z, a-z) are considered for prefix
  calculation; all other characters (spaces, colons, hyphens, numbers, etc.) are ignored.**
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
- Category `general`, chapter `Chapter: Sub-Chapter Name` → G.G.1, G.G.2, ... (only letters "ChapterSubChapterName" are
  considered)

## G.R.5: Requirement parsing boundaries

When parsing requirements from markdown files, tools must correctly identify requirement boundaries:

A requirement starts with a level-2 ATX-style heading (see [G.R.3](#gr3-requirement-definition)). The requirement body
is the content of this markdown section. The markdown parser automatically determines section boundaries (section ends
at the next heading of the same or higher level, or at end of file).
## G.R.6: File encoding

All requirement files must be encoded in UTF-8. All tools must read and write files using UTF-8 encoding. If a file
cannot be read as UTF-8, the tool must return an error indicating encoding issues.

## G.R.7: File system error handling

All tools must handle file system errors gracefully. Common errors include:

- Permission denied: Return error "Permission denied: {path}"
- File not found: For read operations, return appropriate error (e.g., "Category not found", "Requirement not found").
  For write operations, create files/directories as needed (see [C.C.2](#cc2-directory-creation))
- Disk full: Return error "Disk full: cannot write to {path}"
- Invalid path: Return error "Invalid path: {path}"
- Encoding errors: Return error "Encoding error: file is not valid UTF-8"

All file system errors must be returned in the JSON error format specified in [C.C.6](#cc6-error-response-format).

## G.R.8: Empty file handling

Empty files must be handled as follows:

- **Empty category file**: An empty category file (containing only whitespace or no content) is considered valid. It has
  no chapters and no requirements. Tools must return empty arrays for chapters and requirements when querying an empty
  category file.

- **Category file with only whitespace**: Files containing only whitespace (spaces, tabs, newlines) are treated as empty
  files.

- **Chapter with no requirements**: A chapter that exists but contains no requirements (only the level-1 heading) is
  valid. Tools must return an empty requirements array for such chapters.

- **File creation**: When creating a new category file, it must be created as an empty file (or with only the initial
  chapter heading if a chapter is being added).

## G.R.9: Blank line before headings

When writing requirements to files, there must always be a blank line between the requirement text and the next
heading (level-1 or level-2).

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

## G.R.10: Exact heading match

When searching for chapters or requirements by name/index, tools must use exact heading match after proper markdown
parsing, not substring search.

**Correct approach:**

1. Parse file line by line
2. Identify headings using markdown parser
3. Extract heading text and compare exactly

**Incorrect approach:**

- Using `content.find("# ChapterName")` which may match `# ChapterName` as substring of `# ChapterNameExtended`

This prevents bugs where chapter "Foo" is incorrectly matched when searching in a file containing both "# Foobar" and "#
Foo".

## G.R.11: Embedding storage format

Requirements may contain embedding vectors for fuzzy search functionality. Embeddings are stored as HTML comments immediately after the requirement heading (level-2 heading). Format: `<!--embedding:<model_name>:<base64_encoded_vector>-->`

The embedding comment must be placed on a separate line right after the requirement heading line. Example:

```markdown
## G.G.1: Requirement title
<!--embedding:paraphrase-MiniLM-L3-v2:<base64_vector>-->

Requirement text content.
```

- The model name is stored in the comment for informational purposes, but is always ignored during fuzzy search. All embeddings are treated as if they were generated with paraphrase-MiniLM-L3-v2, regardless of the model name stored in the comment.
- The vector is base64-encoded
- This comment must not be included in requirement text when tools return requirement content
- This comment must not be searched when performing keyword-based search
- Only tools that insert or update requirements may modify this comment
- Only the fuzzy search tool may read and use this comment for similarity search

## G.R.12: Ignoring embedding comments in requirement content

All tools that return requirement content (title and/or text) must ignore embedding comments when extracting requirement text. The embedding comment format is defined in G.R.11.

When parsing requirements:
- Tools must skip lines matching the embedding comment pattern `<!--embedding:...-->`
- These lines must not appear in the `text` field of returned requirement objects
- These lines must not be included when calculating requirement boundaries
- The requirement text starts after the heading and any embedding comments, and ends at the next requirement heading or chapter heading

This ensures that embedding metadata remains invisible to clients and does not interfere with requirement content display.

## G.R.13: Embedding model requirements

The embedding model (paraphrase-MiniLM-L3-v2) must be embedded in the binary executable. The model files must be included at compile time using Rust's `include_bytes!` or `include_str!` macros, or similar mechanisms.

At runtime, the model must be loaded from the embedded data, not from external files. This ensures the MCP server is self-contained and does not require external model files.

The model must be loaded lazily (on first use) and reused for all subsequent embedding calculations. After the first load, the same model instance must be reused for the entire application lifetime. The model must be loaded only once per application run.
