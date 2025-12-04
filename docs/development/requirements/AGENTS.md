# Requirements Documentation

This document serves as the single source of truth for all requirements-related information for the project.

## General Solution Description

### Overview

**O.1.** This is an MCP server for managing requirements.

**O.2.** MCP server reads all requirements from the project root directory, which is passed as the first parameter
to each tool.

**O.3.** It supports the following list of tools:

- `reqlix_get_instructions` - returns instructions on how to work with requirements. Must be called before any
  code operation (reading or writing).

### Tool Descriptions

#### reqlix_get_instructions

**T.1.** Description (shown to LLM in tool list):

```
CALL THIS BEFORE ANY CODE OPERATION (reading or writing). 
Returns instructions on how to work with requirements.
```

**T.2.** Instructions (returned when tool is called):

```
# Reqlix MCP Server Instructions

IMPORTANT: Call reqlix_get_instructions before any code operation (reading or writing).

## Available Tools

### reqlix_get_instructions
Returns instructions on how to work with requirements.
- Parameter: project_root (string) - Project root directory path
- Call this tool BEFORE any code operation (reading or writing)

## Workflow

1. Always call reqlix_get_instructions before any code operation
2. Follow the requirements when implementing or modifying code
```

### General Requirements

**R.1.** All requirements must be written in English.

**R.2.** Requirement text must be formatted so that each line does not exceed 120 characters.

**R.3.** All tool descriptions must be stored in a separate YAML file. The file must be embedded into the binary
at build time, and tool descriptions must be loaded from it at runtime.

**R.4.** The YAML file content must be automatically generated from the requirements documentation (this file).
