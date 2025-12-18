
# Configuration

## C.C.1: Requirements directory location

All tools must locate the requirements directory using the same algorithm as defined in
[T.R.3](#tr3-requirements-file-search-order) for `reqlix_get_instructions`. If AGENTS.md is not
found,
it must be created as defined in [T.R.4](#tr4-requirements-file-creation).

## C.C.2: Directory creation

If the requirements directory does not exist, it must be created automatically (including parent directories).

## C.C.3: AGENTS.md exclusive access

The file AGENTS.md is used exclusively by `reqlix_get_instructions`. Other tools must not read or modify
AGENTS.md. The LLM/model cannot modify AGENTS.md directly.

## C.C.4: AGENTS.md protection

The file AGENTS.md must never be deleted.

## C.C.5: JSON response format

All tool responses must be in JSON format. This includes both successful responses and errors.

## C.C.6: Error response format

Error responses must use the following JSON format:

```json
{
  "success": false,
  "error": "Human-readable error message"
}
```

## C.C.7: Category lookup by prefix

To find a category file by prefix:

1. List all `*.md` files in the requirements directory (excluding AGENTS.md)
2. For each file, extract category name from filename (without `.md`)
3. Calculate what prefix this category would have using the algorithm in [G.R.4](#gr4-index-format)
4. Return the category whose calculated prefix matches the search prefix
5. If no category matches the prefix, return an error "Category not found"
