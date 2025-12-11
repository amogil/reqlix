// ServerHandler implementation

use crate::descriptions::*;
use crate::handlers::*;
use crate::params::*;
use rmcp::{
    model::{
        CallToolRequestParam, CallToolResult, Content, ListToolsResult, PaginatedRequestParam,
        ServerCapabilities, Tool,
    },
    service::RequestContext,
    service::RoleServer,
    ServerHandler,
};
use schemars::JsonSchema;
use std::borrow::Cow;
use std::env;

/// Build tool schema from type and description
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

#[allow(clippy::manual_async_fn)]
impl ServerHandler for crate::RequirementsServer {
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
    ) -> impl std::future::Future<
        Output = std::result::Result<ListToolsResult, rmcp::model::ErrorData>,
    > + Send
           + '_ {
        async move {
            let tools = vec![
                build_tool_schema::<GetInstructionsParams>(
                    "reqlix_get_instructions",
                    GET_INSTRUCTIONS_DESC,
                ),
                build_tool_schema::<GetCategoriesParams>(
                    "reqlix_get_categories",
                    GET_CATEGORIES_DESC,
                ),
                build_tool_schema::<GetChaptersParams>("reqlix_get_chapters", GET_CHAPTERS_DESC),
                build_tool_schema::<GetRequirementsParams>(
                    "reqlix_get_requirements",
                    GET_REQUIREMENTS_DESC,
                ),
                build_tool_schema::<GetRequirementParams>(
                    "reqlix_get_requirement",
                    GET_REQUIREMENT_DESC,
                ),
                build_tool_schema::<InsertRequirementParams>(
                    "reqlix_insert_requirement",
                    INSERT_REQUIREMENT_DESC,
                ),
                build_tool_schema::<UpdateRequirementParams>(
                    "reqlix_update_requirement",
                    UPDATE_REQUIREMENT_DESC,
                ),
                build_tool_schema::<GetVersionParams>("reqlix_get_version", GET_VERSION_DESC),
                build_tool_schema::<DeleteRequirementParams>(
                    "reqlix_delete_requirement",
                    DELETE_REQUIREMENT_DESC,
                ),
                build_tool_schema::<SearchRequirementsParams>(
                    "reqlix_search_requirements",
                    SEARCH_REQUIREMENTS_DESC,
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
                    let params: GetInstructionsParams =
                        serde_json::from_value(request.arguments.unwrap_or_default().into())
                            .map_err(|e| {
                                rmcp::model::ErrorData::invalid_params(e.to_string(), None)
                            })?;
                    handle_get_instructions(params)
                }
                "reqlix_get_categories" => {
                    let params: GetCategoriesParams =
                        serde_json::from_value(request.arguments.unwrap_or_default().into())
                            .map_err(|e| {
                                rmcp::model::ErrorData::invalid_params(e.to_string(), None)
                            })?;
                    handle_get_categories(params)
                }
                "reqlix_get_chapters" => {
                    let params: GetChaptersParams =
                        serde_json::from_value(request.arguments.unwrap_or_default().into())
                            .map_err(|e| {
                                rmcp::model::ErrorData::invalid_params(e.to_string(), None)
                            })?;
                    handle_get_chapters(params)
                }
                "reqlix_get_requirements" => {
                    let params: GetRequirementsParams =
                        serde_json::from_value(request.arguments.unwrap_or_default().into())
                            .map_err(|e| {
                                rmcp::model::ErrorData::invalid_params(e.to_string(), None)
                            })?;
                    handle_get_requirements(params)
                }
                "reqlix_get_requirement" => {
                    let params: GetRequirementParams =
                        serde_json::from_value(request.arguments.unwrap_or_default().into())
                            .map_err(|e| {
                                rmcp::model::ErrorData::invalid_params(e.to_string(), None)
                            })?;
                    handle_get_requirement(params)
                }
                "reqlix_insert_requirement" => {
                    let params: InsertRequirementParams =
                        serde_json::from_value(request.arguments.unwrap_or_default().into())
                            .map_err(|e| {
                                rmcp::model::ErrorData::invalid_params(e.to_string(), None)
                            })?;
                    handle_insert_requirement(params)
                }
                "reqlix_update_requirement" => {
                    let params: UpdateRequirementParams =
                        serde_json::from_value(request.arguments.unwrap_or_default().into())
                            .map_err(|e| {
                                rmcp::model::ErrorData::invalid_params(e.to_string(), None)
                            })?;
                    handle_update_requirement(params)
                }
                "reqlix_get_version" => {
                    // G.TOOLREQLIXGETV.3: No parameters required
                    let params: GetVersionParams =
                        serde_json::from_value(request.arguments.unwrap_or_default().into())
                            .map_err(|e| {
                                rmcp::model::ErrorData::invalid_params(e.to_string(), None)
                            })?;
                    handle_get_version(params)
                }
                "reqlix_delete_requirement" => {
                    // G.TOOLREQLIXD.2: Parse parameters
                    let params: DeleteRequirementParams =
                        serde_json::from_value(request.arguments.unwrap_or_default().into())
                            .map_err(|e| {
                                rmcp::model::ErrorData::invalid_params(e.to_string(), None)
                            })?;
                    handle_delete_requirement(params)
                }
                "reqlix_search_requirements" => {
                    // G.TOOLREQLIXS.2: Parse parameters
                    let params: SearchRequirementsParams =
                        serde_json::from_value(request.arguments.unwrap_or_default().into())
                            .map_err(|e| {
                                rmcp::model::ErrorData::invalid_params(e.to_string(), None)
                            })?;
                    handle_search_requirements(params)
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
