use anyhow::Result;
use regex::Regex;
use rmcp::{
    ServerHandler, ServiceExt,
    model::{
        CallToolRequestParam, CallToolResult, Content, ListToolsResult, PaginatedRequestParam,
        ServerCapabilities, Tool,
    },
    service::RequestContext,
    service::RoleServer,
    transport::stdio,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::env;
use std::fs;
use std::path::PathBuf;

// Tool descriptions (T.1, G.1, S.1, D.1)
const GET_INSTRUCTIONS_DESC: &str = "CALL THIS BEFORE ANY CODE OPERATION (reading or writing). \
Returns instructions on how to work with requirements. \
This MCP server is the single source of truth for everything related to requirements.";

const GET_REQUIREMENTS_DESC: &str = "Returns all requirements in the specified section.";

const SET_REQUIREMENTS_DESC: &str = "Modifies a requirement in a section. \
To add a new requirement, specify a new index number. \
Returns the added/modified requirement (text or index may be adjusted according to policies).";

const DELETE_REQUIREMENTS_DESC: &str = "Deletes a requirement with the specified index from a section. \
Returns the deleted requirement text, or an error if not found.";

// Placeholder content (T.3.4)
const PLACEHOLDER_CONTENT: &str = r#"# Requirements

## General Rules

**R.1.** All requirements must be written in English.

## Sections

Requirements are organized into the following sections:

- General requirements (key: general)
- Requirements change management (key: requirements_change_management)
- Testing requirements (key: testing)
- Code quality requirements (key: code_quality)
- Code writing requirements (key: code_style)
- Change validation requirements (key: change_validation)

"#;

// Parameter constraints (P.1)
const MAX_PROJECT_ROOT_LEN: usize = 1000;
const MAX_OPERATION_DESC_LEN: usize = 10000;
const MAX_SECTION_LEN: usize = 100;
const MAX_INDEX_LEN: usize = 10;
const MAX_TEXT_LEN: usize = 10000;

/// Parameters for getting instructions (T.2)
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetInstructionsParams {
    /// Path to the project root directory. Used to locate requirements and project source code.
    pub project_root: String,
    /// Brief description of the operation that LLM intends to perform.
    pub operation_description: String,
}

/// Parameters for getting requirements (G.2)
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetRequirementsParams {
    /// Path to the project root directory. Used to locate requirements and project source code.
    pub project_root: String,
    /// Brief description of the operation that LLM intends to perform.
    pub operation_description: String,
    /// Section key (e.g., "general", "testing", "code_quality").
    pub section: String,
}

/// Parameters for setting requirements (S.2)
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetRequirementsParams {
    /// Path to the project root directory. Used to locate requirements and project source code.
    pub project_root: String,
    /// Brief description of the operation that LLM intends to perform.
    pub operation_description: String,
    /// Section key (e.g., "general", "testing", "code_quality").
    pub section: String,
    /// Requirement index (e.g., "1", "2.1", "3").
    pub index: String,
    /// Requirement text.
    pub text: String,
}

/// Parameters for deleting requirements (D.2)
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteRequirementsParams {
    /// Path to the project root directory. Used to locate requirements and project source code.
    pub project_root: String,
    /// Brief description of the operation that LLM intends to perform.
    pub operation_description: String,
    /// Section key (e.g., "general", "testing", "code_quality").
    pub section: String,
    /// Requirement index to delete (e.g., "1", "2.1", "3").
    pub index: String,
}

/// A parsed requirement
#[derive(Debug, Clone)]
struct Requirement {
    index: String,
    text: String,
}

#[derive(Debug, Clone, Default)]
pub struct RequirementsServer;

impl RequirementsServer {
    pub fn new() -> Self {
        Self
    }

    // Parameter validation (P.2)
    fn validate_project_root(value: &str) -> Result<(), String> {
        if value.is_empty() {
            return Err("project_root is required".to_string());
        }
        if value.len() > MAX_PROJECT_ROOT_LEN {
            return Err(format!(
                "project_root exceeds maximum length of {} characters",
                MAX_PROJECT_ROOT_LEN
            ));
        }
        Ok(())
    }

    fn validate_operation_description(value: &str) -> Result<(), String> {
        if value.is_empty() {
            return Err("operation_description is required".to_string());
        }
        if value.len() > MAX_OPERATION_DESC_LEN {
            return Err(format!(
                "operation_description exceeds maximum length of {} characters",
                MAX_OPERATION_DESC_LEN
            ));
        }
        Ok(())
    }

    fn validate_section(value: &str) -> Result<(), String> {
        if value.is_empty() {
            return Err("section is required".to_string());
        }
        if value.len() > MAX_SECTION_LEN {
            return Err(format!(
                "section exceeds maximum length of {} characters",
                MAX_SECTION_LEN
            ));
        }
        Ok(())
    }

    fn validate_index(value: &str) -> Result<(), String> {
        if value.is_empty() {
            return Err("index is required".to_string());
        }
        if value.len() > MAX_INDEX_LEN {
            return Err(format!(
                "index exceeds maximum length of {} characters",
                MAX_INDEX_LEN
            ));
        }
        Ok(())
    }

    fn validate_text(value: &str) -> Result<(), String> {
        if value.is_empty() {
            return Err("text is required".to_string());
        }
        if value.len() > MAX_TEXT_LEN {
            return Err(format!(
                "text exceeds maximum length of {} characters",
                MAX_TEXT_LEN
            ));
        }
        Ok(())
    }

    // T.3.1: Search paths for AGENTS.md
    fn get_search_paths(project_root: &str) -> Vec<PathBuf> {
        let root = PathBuf::from(project_root);
        let mut paths = Vec::new();

        if let Ok(rel_path) = env::var("REQLIX_REQ_REL_PATH") {
            paths.push(root.join(&rel_path).join("AGENTS.md"));
        }

        paths.push(root.join("docs/development/requirements/AGENTS.md"));
        paths.push(root.join("docs/dev/req/AGENTS.md"));

        paths
    }

    // T.3.2: Path for creating AGENTS.md
    fn get_create_path(project_root: &str) -> PathBuf {
        let root = PathBuf::from(project_root);

        if let Ok(rel_path) = env::var("REQLIX_REQ_REL_PATH") {
            root.join(&rel_path).join("AGENTS.md")
        } else {
            root.join("docs/development/requirements/AGENTS.md")
        }
    }

    // C.1, T.3.1-T.3.3: Find or create requirements file
    fn find_or_create_requirements_file(project_root: &str) -> Result<PathBuf, String> {
        for path in Self::get_search_paths(project_root) {
            if path.exists() {
                return Ok(path);
            }
        }

        let create_path = Self::get_create_path(project_root);

        if let Some(parent) = create_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directories: {}", e))?;
        }

        fs::write(&create_path, PLACEHOLDER_CONTENT)
            .map_err(|e| format!("Failed to create requirements file: {}", e))?;

        Ok(create_path)
    }

    // G.3.1: Get requirements directory
    fn get_requirements_dir(project_root: &str) -> Result<PathBuf, String> {
        let agents_path = Self::find_or_create_requirements_file(project_root)?;
        agents_path
            .parent()
            .map(|p| p.to_path_buf())
            .ok_or_else(|| "Could not determine requirements directory".to_string())
    }

    // C.2: Parse requirements from section file content
    fn parse_requirements(content: &str) -> Vec<Requirement> {
        let re = Regex::new(r"\*\*([^*]+)\.\*\*\s*(.+)").unwrap();
        let mut requirements = Vec::new();

        for line in content.lines() {
            if let Some(caps) = re.captures(line) {
                requirements.push(Requirement {
                    index: caps[1].to_string(),
                    text: caps[2].to_string(),
                });
            }
        }

        requirements
    }

    // C.2: Format requirements to section file content
    fn format_requirements(requirements: &[Requirement]) -> String {
        requirements
            .iter()
            .map(|r| format!("**{}.** {}", r.index, r.text))
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    // S.3.4: Compare indices for sorting
    fn compare_indices(a: &str, b: &str) -> std::cmp::Ordering {
        let a_parts: Vec<u32> = a.split('.').filter_map(|s| s.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|s| s.parse().ok()).collect();

        for (ap, bp) in a_parts.iter().zip(b_parts.iter()) {
            match ap.cmp(bp) {
                std::cmp::Ordering::Equal => continue,
                other => return other,
            }
        }

        a_parts.len().cmp(&b_parts.len())
    }

    // S.3.5, D.3.5: Rebuild sections list in AGENTS.md
    fn rebuild_sections(requirements_dir: &PathBuf) -> Result<(), String> {
        let agents_path = requirements_dir.join("AGENTS.md");

        // Find all .md files except AGENTS.md
        let mut sections: Vec<String> = Vec::new();
        let entries = fs::read_dir(requirements_dir)
            .map_err(|e| format!("Failed to read requirements directory: {}", e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    if let Some(stem) = path.file_stem() {
                        let name = stem.to_string_lossy().to_string();
                        if name != "AGENTS" {
                            sections.push(name);
                        }
                    }
                }
            }
        }

        sections.sort();

        // Read current AGENTS.md
        let content = fs::read_to_string(&agents_path)
            .map_err(|e| format!("Failed to read AGENTS.md: {}", e))?;

        // Build new sections list
        let sections_text = if sections.is_empty() {
            "No sections defined yet.".to_string()
        } else {
            sections
                .iter()
                .map(|s| format!("- {} (key: {})", Self::section_key_to_name(s), s))
                .collect::<Vec<_>>()
                .join("\n")
        };

        // Replace sections in AGENTS.md
        let new_content = Self::replace_sections_in_agents_md(&content, &sections_text);

        fs::write(&agents_path, new_content)
            .map_err(|e| format!("Failed to write AGENTS.md: {}", e))?;

        Ok(())
    }

    // Helper: Convert section key to human-readable name
    fn section_key_to_name(key: &str) -> String {
        key.split('_')
            .map(|word| {
                let mut chars: Vec<char> = word.chars().collect();
                if let Some(first) = chars.first_mut() {
                    *first = first.to_uppercase().next().unwrap_or(*first);
                }
                chars.into_iter().collect::<String>()
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    // Helper: Replace ## Sections content in AGENTS.md
    fn replace_sections_in_agents_md(content: &str, new_sections: &str) -> String {
        let re = Regex::new(r"(?ms)(## Sections\s*\n\s*Requirements are organized into the following sections:\s*\n).*?(\n\n|$)").unwrap();
        
        if re.is_match(content) {
            re.replace(
                content,
                format!("$1\n{}\n\n", new_sections).as_str(),
            )
            .to_string()
        } else {
            // If ## Sections doesn't exist, append it
            format!(
                "{}\n## Sections\n\nRequirements are organized into the following sections:\n\n{}\n",
                content.trim_end(),
                new_sections
            )
        }
    }

    fn build_tool_schema<T: JsonSchema>(name: &str, description: &'static str) -> Tool {
        let schema = schemars::schema_for!(T);
        let input_schema: serde_json::Value = serde_json::to_value(&schema).unwrap_or_default();

        Tool {
            name: name.to_string().into(),
            description: Some(Cow::Borrowed(description)),
            input_schema: serde_json::from_value(input_schema).unwrap_or_default(),
            annotations: None,
            icons: None,
            meta: None,
            output_schema: None,
            title: None,
        }
    }

    // Tool implementations

    // T.3: reqlix_get_instructions
    fn handle_get_instructions(params: GetInstructionsParams) -> Result<String, String> {
        Self::validate_project_root(&params.project_root)?;
        Self::validate_operation_description(&params.operation_description)?;

        let path = Self::find_or_create_requirements_file(&params.project_root)?;

        fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read requirements file: {}", e))
    }

    // G.3: reqlix_get_requirements
    fn handle_get_requirements(params: GetRequirementsParams) -> Result<String, String> {
        Self::validate_project_root(&params.project_root)?;
        Self::validate_operation_description(&params.operation_description)?;
        Self::validate_section(&params.section)?;

        let requirements_dir = Self::get_requirements_dir(&params.project_root)?;
        let section_path = requirements_dir.join(format!("{}.md", params.section));

        if section_path.exists() {
            fs::read_to_string(&section_path)
                .map_err(|e| format!("Failed to read section file: {}", e))
        } else {
            Ok("No requirements in this section.".to_string())
        }
    }

    // S.3: reqlix_set_requirements
    fn handle_set_requirements(params: SetRequirementsParams) -> Result<String, String> {
        Self::validate_project_root(&params.project_root)?;
        Self::validate_operation_description(&params.operation_description)?;
        Self::validate_section(&params.section)?;
        Self::validate_index(&params.index)?;
        Self::validate_text(&params.text)?;

        let requirements_dir = Self::get_requirements_dir(&params.project_root)?;
        let section_path = requirements_dir.join(format!("{}.md", params.section));

        // S.3.1: Read or create section file
        let mut requirements = if section_path.exists() {
            let content = fs::read_to_string(&section_path)
                .map_err(|e| format!("Failed to read section file: {}", e))?;
            Self::parse_requirements(&content)
        } else {
            Vec::new()
        };

        // S.3.2, S.3.3: Update or add requirement
        let mut found = false;
        for req in &mut requirements {
            if req.index == params.index {
                req.text = params.text.clone();
                found = true;
                break;
            }
        }

        if !found {
            requirements.push(Requirement {
                index: params.index.clone(),
                text: params.text.clone(),
            });
        }

        // S.3.4: Sort by index
        requirements.sort_by(|a, b| Self::compare_indices(&a.index, &b.index));

        // Write section file
        let content = Self::format_requirements(&requirements);
        fs::write(&section_path, &content)
            .map_err(|e| format!("Failed to write section file: {}", e))?;

        // S.3.5: Rebuild sections in AGENTS.md
        Self::rebuild_sections(&requirements_dir)?;

        // S.3.6: Return final requirement
        let final_req = requirements
            .iter()
            .find(|r| r.index == params.index)
            .map(|r| format!("**{}.** {}", r.index, r.text))
            .unwrap_or_else(|| format!("**{}.** {}", params.index, params.text));

        Ok(final_req)
    }

    // D.3: reqlix_delete_requirements
    fn handle_delete_requirements(params: DeleteRequirementsParams) -> Result<String, String> {
        Self::validate_project_root(&params.project_root)?;
        Self::validate_operation_description(&params.operation_description)?;
        Self::validate_section(&params.section)?;
        Self::validate_index(&params.index)?;

        let requirements_dir = Self::get_requirements_dir(&params.project_root)?;
        let section_path = requirements_dir.join(format!("{}.md", params.section));

        // D.3.1: Check if section exists
        if !section_path.exists() {
            return Err("Section not found.".to_string());
        }

        let content = fs::read_to_string(&section_path)
            .map_err(|e| format!("Failed to read section file: {}", e))?;

        let mut requirements = Self::parse_requirements(&content);

        // D.3.2: Find requirement
        let position = requirements.iter().position(|r| r.index == params.index);

        let deleted_req = match position {
            Some(pos) => requirements.remove(pos),
            None => return Err("Requirement not found.".to_string()),
        };

        // D.3.3: Return deleted content
        let deleted_text = format!("**{}.** {}", deleted_req.index, deleted_req.text);

        // D.3.4: Delete file if empty, otherwise write updated content
        if requirements.is_empty() {
            fs::remove_file(&section_path)
                .map_err(|e| format!("Failed to delete section file: {}", e))?;
        } else {
            let new_content = Self::format_requirements(&requirements);
            fs::write(&section_path, &new_content)
                .map_err(|e| format!("Failed to write section file: {}", e))?;
        }

        // D.3.5: Rebuild sections in AGENTS.md
        Self::rebuild_sections(&requirements_dir)?;

        Ok(deleted_text)
    }
}

#[allow(clippy::manual_async_fn)]
impl ServerHandler for RequirementsServer {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        rmcp::model::ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: rmcp::model::Implementation {
                name: "reqlix".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                title: None,
                icons: None,
                website_url: None,
            },
            ..Default::default()
        }
    }

    fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = std::result::Result<ListToolsResult, rmcp::model::ErrorData>>
           + Send
           + '_ {
        async move {
            let tools = vec![
                Self::build_tool_schema::<GetInstructionsParams>(
                    "reqlix_get_instructions",
                    GET_INSTRUCTIONS_DESC,
                ),
                Self::build_tool_schema::<GetRequirementsParams>(
                    "reqlix_get_requirements",
                    GET_REQUIREMENTS_DESC,
                ),
                Self::build_tool_schema::<SetRequirementsParams>(
                    "reqlix_set_requirements",
                    SET_REQUIREMENTS_DESC,
                ),
                Self::build_tool_schema::<DeleteRequirementsParams>(
                    "reqlix_delete_requirements",
                    DELETE_REQUIREMENTS_DESC,
                ),
            ];

            Ok(ListToolsResult {
                tools,
                next_cursor: None,
                meta: None,
            })
        }
    }

    fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = std::result::Result<CallToolResult, rmcp::model::ErrorData>>
           + Send
           + '_ {
        async move {
            let result = match request.name.as_ref() {
                "reqlix_get_instructions" => {
                    let params: GetInstructionsParams = serde_json::from_value(
                        request.arguments.unwrap_or_default().into(),
                    )
                    .map_err(|e| rmcp::model::ErrorData::invalid_params(e.to_string(), None))?;

                    Self::handle_get_instructions(params)
                        .map_err(|e| rmcp::model::ErrorData::internal_error(e, None))?
                }
                "reqlix_get_requirements" => {
                    let params: GetRequirementsParams = serde_json::from_value(
                        request.arguments.unwrap_or_default().into(),
                    )
                    .map_err(|e| rmcp::model::ErrorData::invalid_params(e.to_string(), None))?;

                    Self::handle_get_requirements(params)
                        .map_err(|e| rmcp::model::ErrorData::internal_error(e, None))?
                }
                "reqlix_set_requirements" => {
                    let params: SetRequirementsParams = serde_json::from_value(
                        request.arguments.unwrap_or_default().into(),
                    )
                    .map_err(|e| rmcp::model::ErrorData::invalid_params(e.to_string(), None))?;

                    Self::handle_set_requirements(params)
                        .map_err(|e| rmcp::model::ErrorData::internal_error(e, None))?
                }
                "reqlix_delete_requirements" => {
                    let params: DeleteRequirementsParams = serde_json::from_value(
                        request.arguments.unwrap_or_default().into(),
                    )
                    .map_err(|e| rmcp::model::ErrorData::invalid_params(e.to_string(), None))?;

                    Self::handle_delete_requirements(params)
                        .map_err(|e| rmcp::model::ErrorData::internal_error(e, None))?
                }
                _ => {
                    return Err(rmcp::model::ErrorData::invalid_params(
                        format!("Unknown tool: {}", request.name),
                        None,
                    ));
                }
            };

            Ok(CallToolResult {
                content: vec![Content::text(result)],
                is_error: None,
                meta: None,
                structured_content: None,
            })
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("Starting Reqlix MCP server");

    let service = RequirementsServer::new();
    let server = service.serve(stdio()).await?;

    tracing::info!("Reqlix MCP server started successfully");

    server.waiting().await?;

    Ok(())
}
