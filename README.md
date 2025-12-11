# reqlix

[![Version](https://img.shields.io/badge/version-0.1.7-blue.svg)](https://github.com/yourusername/reqlix)
[![License](https://img.shields.io/badge/license-BSL--1.1-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

> MCP (Model Context Protocol) server for managing project requirements through structured markdown files.

**reqlix** provides LLM assistants with structured access to project requirements via the standardized MCP protocol, enabling automated requirement management and maintaining synchronization between code and documentation.

## 📋 Table of Contents

- [Why](#-why)
- [How We Solve It](#-how-we-solve-it)
- [How to Use](#-how-to-use)
  - [Installation](#installation)
  - [Configuration](#configuration)
  - [Usage Examples](#usage-examples)
- [Requirements File Format](#-requirements-file-format)
- [Environment Variables](#-environment-variables)
- [Troubleshooting](#-troubleshooting)
- [License](#-license)
- [Contributing](#-contributing)

## 🎯 Why

When developing software with LLM assistants, managing requirements becomes a challenge:

- **No single source of truth**: Requirements are scattered across different documents, code comments, and notes, making them difficult to track and maintain
- **Automation difficulties**: LLM assistants cannot effectively work with requirements without a structured API
- **Synchronization risk**: Code and requirements can diverge without a mechanism to keep them in sync
- **No versioning**: Requirement changes are not tracked systematically
- **Navigation difficulties**: It's hard to find and understand relationships between requirements in large projects

**reqlix** solves these problems by providing LLM assistants with structured access to requirements through the standardized MCP protocol.

## 🔧 How We Solve It

**reqlix** is an MCP server that:

### 📁 Structured Requirements Storage

Requirements are organized in a hierarchical structure:
- **Categories** — groups of related requirements (e.g., `general`, `testing`)
- **Chapters** — sections within categories (e.g., "General Requirements", "Unit Tests")
- **Requirements** — individual atomic requirements with unique indices

**Index format**: `{CATEGORY}.{CHAPTER}.{NUMBER}` (e.g., `G.G.1`, `T.U.2`)

### 📝 Markdown as Storage Format

Requirements are stored in plain markdown files, which ensures:
- **Readability**: Requirements can be viewed and edited in any text editor
- **Versioning**: Standard Git tools work with markdown files
- **Transparency**: The format is open and understandable, with no proprietary formats

### 🔌 Programmatic Access via MCP

The server provides a set of tools for:
- **Reading**: Getting instructions, lists of categories, chapters, and requirements
- **Creating**: Adding new requirements with automatic index generation
- **Updating**: Modifying existing requirements (batch updates supported)
- **Deleting**: Removing requirements with automatic cleanup of empty chapters
- **Searching**: Finding requirements by keywords

### ⚙️ Automation and Validation

- Automatic generation of unique indices for new requirements
- Parameter and data format validation
- Error handling with clear messages
- Support for batch operations for efficient work

### 🤖 Integration with LLM Assistants

The server follows the MCP protocol, allowing LLM assistants to:
- Automatically receive instructions on working with requirements
- Read and modify requirements during development
- Reference requirements in code comments
- Maintain synchronization between code and requirements

## 🚀 How to Use

### Installation

#### Requirements

- **Rust** 1.70+ (for building from source)
- **Cargo** (Rust package manager)

#### Download Pre-built Binaries (Recommended)

Pre-built binaries are available for all major platforms in the [Releases](https://github.com/amogil/reqlix/releases) section:

- **Linux**: x64 and ARM64
- **macOS**: Intel (x86_64) and Apple Silicon (ARM64)
- **Windows**: x64 and ARM64

1. Download the appropriate ZIP archive for your platform
2. Extract the binary from the archive
3. Add it to your PATH or use it directly in your MCP client configuration

**Note for macOS users**: If you see a security warning, see the [macOS Security Warning](#macos-security-warning) section in Troubleshooting.

#### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/reqlix.git
cd reqlix

# Build the project
cargo build --release

# The executable will be in target/release/reqlix
```

#### Installing via Cargo (if published on crates.io)

```bash
cargo install reqlix
```

### Configuration

#### 1. MCP Client Configuration

Add **reqlix** to your MCP client configuration (e.g., in Cursor or another IDE with MCP support).

**Example configuration for Cursor** (`~/.cursor/mcp.json` or similar file):

```json
{
  "mcpServers": {
    "reqlix": {
      "command": "path/to/reqlix",
      "args": []
    }
  }
}
```

If you built from source:

```json
{
  "mcpServers": {
    "reqlix": {
      "command": "/path/to/reqlix/target/release/reqlix"
    }
  }
}
```

#### 2. Project Structure Setup

**reqlix** will automatically create the requirements structure on first use. By default, requirements are stored in:

```
docs/development/requirements/
├── AGENTS.md          # Instructions for LLM (created automatically)
├── general.md         # Category "general"
├── testing.md         # Category "testing"
└── ...
```

#### 3. Custom Requirements Path (Optional)

You can change the path to the requirements directory via an environment variable:

```bash
export REQLIX_REQ_REL_PATH="custom/path/to/requirements"
```

The server will look for `AGENTS.md` in the following locations (in priority order):
1. `{project_root}/{REQLIX_REQ_REL_PATH}/AGENTS.md` (if the variable is set)
2. `{project_root}/docs/development/requirements/AGENTS.md`
3. `{project_root}/docs/dev/req/AGENTS.md`

#### 4. Using in Your Project

After configuring the MCP client, the LLM assistant can use the following tools:

**Get instructions** before starting work:
```json
{
  "project_root": "/path/to/project",
  "operation_description": "Starting code review"
}
```

**Browse categories**:
```json
{
  "project_root": "/path/to/project",
  "operation_description": "Browsing requirements"
}
```

**Create a new requirement**:
```json
{
  "project_root": "/path/to/project",
  "operation_description": "Adding new requirement",
  "category": "general",
  "chapter": "General Requirements",
  "title": "Requirement Title",
  "text": "Requirement description..."
}
```

**Update a requirement**:
```json
{
  "project_root": "/path/to/project",
  "operation_description": "Updating requirement",
  "index": "G.G.1",
  "text": "Updated requirement text"
}
```

### Usage Examples

#### Creating Requirements Structure

On the first call to `reqlix_get_instructions`, the server will automatically create the `AGENTS.md` file with instructions for the LLM.

#### Adding a Requirement

**Request:**
```json
{
  "project_root": "/Users/user/myproject",
  "operation_description": "Adding new requirement for user authentication",
  "category": "general",
  "chapter": "Authentication",
  "title": "User Login",
  "text": "The system must support user login with email and password."
}
```

**Result**: A requirement is created with an index, e.g., `G.A.1` (where `G` is the prefix for category "general", `A` is the prefix for chapter "Authentication", `1` is the requirement number).

#### Searching Requirements

**Request:**
```json
{
  "project_root": "/Users/user/myproject",
  "operation_description": "Searching for authentication-related requirements",
  "keywords": ["authentication", "login", "user"]
}
```

## 📄 Requirements File Format

Requirements are stored in markdown files with the following format:

```markdown
# Chapter Name

## G.C.1: Requirement Title

Requirement text goes here. It can span multiple lines
and include code blocks, lists, and other markdown elements.

## G.C.2: Another Requirement

Another requirement description.
```

## 🔐 Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `REQLIX_REQ_REL_PATH` | Relative path to requirements directory (from project root) | `custom/path/to/requirements` |
| `RUST_LOG` | Logging level | `info`, `debug`, `trace` |

## 🐛 Troubleshooting

### Server Won't Start

- Make sure the executable has execute permissions: `chmod +x reqlix`
- Check logs via `RUST_LOG=debug reqlix`

### Requirements Not Found

- Verify that the project path is specified correctly
- Ensure the directory structure is created
- Check the `REQLIX_REQ_REL_PATH` environment variable if used

### Validation Errors

- **Category names** must contain only lowercase letters (a-z) and underscores (_)
- **Chapter names** can contain letters, spaces, colons, and hyphens
- **Indices** must be in the format `CATEGORY.CHAPTER.NUMBER`

### macOS Security Warning

If you see a warning that macOS "could not verify reqlix is free of malware" when downloading the binary:

**Option 1: Remove quarantine attribute (Recommended)**
```bash
xattr -d com.apple.quarantine /path/to/reqlix
```

**Option 2: Open via Finder**
1. Right-click on the `reqlix` binary
2. Select "Open" from the context menu
3. Click "Open" in the security dialog

This warning appears because the binary is not code-signed with an Apple Developer certificate. The binary is safe to use - it's built from open source code and the warning is a standard macOS security feature for unsigned applications.

## 📜 License

This project is licensed under the [Business Source License 1.1](LICENSE).

### License Summary

You are granted the right to use, modify, and run this Software for any purpose, including within commercial organizations, as long as:

1. You do **NOT** integrate, embed, or distribute this Software or any part of it as a component of another commercial or non-commercial software product without obtaining a separate commercial license from the Licensor.

2. You do **NOT** offer this Software as part of a hosted, cloud, or software-as-a-service (SaaS) offering to third parties without obtaining a separate commercial license from the Licensor.

3. You may use this Software in the development of your own commercial or non-commercial applications, provided that the Software itself is not included, embedded, linked, or distributed as part of your application.

**Change Date**: December 31, 2027  
**Change License**: GNU General Public License, version 2.0 (GPL-2.0)

Until the Change Date, any use of the Software outside the Additional Use Grant requires a separate commercial license from the Licensor. After the Change Date, the Software will be made available under the Change License.

For full license terms, see the [LICENSE](LICENSE) file.

## 🤝 Contributing

Contributions are welcome! Please:

1. Create an [issue](https://github.com/yourusername/reqlix/issues) to discuss changes
2. Fork the repository
3. Create a feature branch (`git checkout -b feature/amazing-feature`)
4. Commit your changes (`git commit -m 'Add some amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a [Pull Request](https://github.com/yourusername/reqlix/pulls)

---

⭐ If this project was helpful, please give it a star!
