use anyhow::Result;
use once_cell::sync::Lazy;
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
use std::collections::HashMap;

// Embed YAML file into binary at compile time
const TOOLS_YAML: &str = include_str!("tools.yaml");

/// Tool configuration from YAML
#[derive(Debug, Clone, Deserialize)]
struct ToolConfig {
    description: String,
    instructions: Option<String>,
}

/// Root YAML config
#[derive(Debug, Clone, Deserialize)]
struct Config {
    tools: HashMap<String, ToolConfig>,
}

/// Lazily parsed config from embedded YAML
static CONFIG: Lazy<Config> = Lazy::new(|| {
    serde_yaml::from_str(TOOLS_YAML).expect("Failed to parse embedded tools.yaml")
});

/// Parameters for getting instructions
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetInstructionsParams {
    /// Project root directory path
    #[schemars(description = "Project root directory path")]
    pub project_root: String,
}

#[derive(Debug, Clone, Default)]
pub struct RequirementsServer;

impl RequirementsServer {
    pub fn new() -> Self {
        Self
    }

    fn get_tool_description(tool_name: &str) -> Cow<'static, str> {
        CONFIG
            .tools
            .get(tool_name)
            .map(|t| Cow::Owned(t.description.clone()))
            .unwrap_or_else(|| Cow::Owned(format!("Tool {}", tool_name)))
    }

    fn get_instructions() -> String {
        CONFIG
            .tools
            .get("reqlix_get_instructions")
            .and_then(|t| t.instructions.clone())
            .unwrap_or_else(|| "No instructions available".to_string())
    }

    fn build_tool_schema<T: JsonSchema>(name: &str) -> Tool {
        let schema = schemars::schema_for!(T);
        let input_schema: serde_json::Value = serde_json::to_value(&schema).unwrap_or_default();

        Tool {
            name: name.to_string().into(),
            description: Some(Self::get_tool_description(name)),
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
                Self::build_tool_schema::<GetInstructionsParams>("reqlix_get_instructions"),
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
                "reqlix_get_instructions" => Self::get_instructions(),
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
