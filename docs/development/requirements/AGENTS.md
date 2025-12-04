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

### General Requirements

**R.1.** All requirements must be written in English.

**R.2.** Requirement text must be formatted so that each line does not exceed 120 characters.

**R.3.** All tool descriptions must be stored in a separate YAML file. The file must be embedded into the binary
at build time, and tool descriptions must be loaded from it at runtime.
