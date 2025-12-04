use anyhow::Result;
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

const TOOL_DESCRIPTION: &str = r#"CALL THIS BEFORE ANY CODE OPERATION (reading or writing). Returns instructions on how to work with requirements. This MCP server is the single source of truth for everything related to requirements."#;

const PLACEHOLDER_CONTENT: &str = r#"# Requirements Documentation

## General Rules

**R.1.** All requirements must be written in English.

**R.2.** Requirement text must be formatted so that each line does not exceed 120 characters.

## Requirements

<!-- Add your requirements here -->
"#;

/// Parameters for getting instructions
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetInstructionsParams {
    /// Path to the project root directory
    #[schemars(description = "Path to the project root directory")]
    pub project_root: String,
    /// Brief description of the operation that LLM intends to perform
    #[schemars(description = "Brief description of the operation that LLM intends to perform")]
    pub operation_description: String,
}

#[derive(Debug, Clone, Default)]
pub struct RequirementsServer;

impl RequirementsServer {
    pub fn new() -> Self {
        Self
    }

    fn get_search_paths(project_root: &str) -> Vec<PathBuf> {
        let root = PathBuf::from(project_root);
        let mut paths = Vec::new();

        // 1. Check REQLIX_REQ_REL_PATH environment variable
        if let Ok(rel_path) = env::var("REQLIX_REQ_REL_PATH") {
            paths.push(root.join(&rel_path).join("AGENTS.md"));
        }

        // 2. Default paths
        paths.push(root.join("docs/development/requirements/AGENTS.md"));
        paths.push(root.join("docs/dev/req/AGENTS.md"));

        paths
    }

    fn get_create_path(project_root: &str) -> PathBuf {
        let root = PathBuf::from(project_root);

        // Use REQLIX_REQ_REL_PATH if set, otherwise default
        if let Ok(rel_path) = env::var("REQLIX_REQ_REL_PATH") {
            root.join(&rel_path).join("AGENTS.md")
        } else {
            root.join("docs/development/requirements/AGENTS.md")
        }
    }

    fn find_or_create_requirements_file(project_root: &str) -> Result<PathBuf, String> {
        // Search for existing file
        for path in Self::get_search_paths(project_root) {
            if path.exists() {
                return Ok(path);
            }
        }

        // File not found, create with placeholder
        let create_path = Self::get_create_path(project_root);

        // Create parent directories if needed
        if let Some(parent) = create_path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create directories: {}", e))?;
        }

        // Create file with placeholder content
        fs::write(&create_path, PLACEHOLDER_CONTENT)
            .map_err(|e| format!("Failed to create requirements file: {}", e))?;

        Ok(create_path)
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
}

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
    ) -> impl std::future::Future<Output = std::result::Result<ListToolsResult, rmcp::model::ErrorData>> + Send + '_ {
        async move {
            let tools = vec![
                Self::build_tool_schema::<GetInstructionsParams>(
                    "reqlix_get_instructions",
                    TOOL_DESCRIPTION,
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
    ) -> impl std::future::Future<Output = std::result::Result<CallToolResult, rmcp::model::ErrorData>> + Send + '_ {
        async move {
            let result = match request.name.as_ref() {
                "reqlix_get_instructions" => {
                    let params: GetInstructionsParams = serde_json::from_value(
                        request.arguments.unwrap_or_default().into(),
                    )
                    .map_err(|e| rmcp::model::ErrorData::invalid_params(e.to_string(), None))?;

                    // Find or create requirements file
                    let path = Self::find_or_create_requirements_file(&params.project_root)
                        .map_err(|e| rmcp::model::ErrorData::internal_error(e, None))?;

                    // Read and return file content
                    fs::read_to_string(&path)
                        .map_err(|e| rmcp::model::ErrorData::internal_error(
                            format!("Failed to read requirements file: {}", e),
                            None,
                        ))?
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
