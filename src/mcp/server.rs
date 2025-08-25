use rmcp::{
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::ErrorData as McpError,
    model::*,
    service::RequestContext,
    tool, tool_handler, tool_router, RoleServer, ServerHandler,
};
use serde_json::json;
use std::future::Future;

use crate::AppState;

pub struct McpServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl McpServer {
    pub fn new(_app_state: AppState) -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    fn _create_resource_text(&self, uri: &str, name: &str) -> Resource {
        rmcp::model::RawResource::new(uri, name.to_string()).no_annotation()
    }

    // Tool implementations
    #[tool(description = "Search for cards by name")]
    pub async fn search_cards(
        &self,
        Parameters(request): Parameters<crate::cards::mcp::SearchCardsRequest>,
    ) -> Result<CallToolResult, McpError> {
        let limit = request.limit.unwrap_or(10);

        // TODO: Implement actual search using card_repo
        let result = format!(
            "Searching for cards with query '{}' (limit: {})",
            request.query, limit
        );

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Get a specific card by ID")]
    pub async fn get_card_by_id(
        &self,
        Parameters(request): Parameters<crate::cards::mcp::GetCardByIdRequest>,
    ) -> Result<CallToolResult, McpError> {
        // TODO: Implement actual card retrieval using card_repo
        let result = format!("Retrieving card with ID: {}", request.id);

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Get cards by type")]
    pub async fn get_cards_by_type(
        &self,
        Parameters(request): Parameters<crate::cards::mcp::GetCardsByTypeRequest>,
    ) -> Result<CallToolResult, McpError> {
        let limit = request.limit.unwrap_or(20);

        // TODO: Implement actual type filtering using card_repo
        let result = format!(
            "Getting cards of type '{}' (limit: {})",
            request.card_type, limit
        );

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Get total number of cards in database")]
    pub async fn get_card_count(&self) -> Result<CallToolResult, McpError> {
        // TODO: Implement actual count using card_repo
        let result = "Total cards in database: [count to be implemented]";

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }
}

#[tool_handler]
impl ServerHandler for McpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("ManaQL MCP Server - Provides tools and prompts for Magic: The Gathering card data.".to_string()),
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            resources: vec![self._create_resource_text("manaql://cards", "mtg-cards")],
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        ReadResourceRequestParam { uri }: ReadResourceRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        match uri.as_str() {
            "manaql://cards" => {
                let cards_info = "ManaQL Cards Database\n\nAccess to card information, search, and management tools.";
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(cards_info, uri)],
                })
            }
            _ => Err(McpError::resource_not_found(
                "resource_not_found",
                Some(json!({
                    "uri": uri
                })),
            )),
        }
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(ListResourceTemplatesResult {
            next_cursor: None,
            resource_templates: Vec::new(),
        })
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(self.get_info())
    }
}

impl McpServer {
    /// Start the MCP server with stdio transport
    pub async fn start_stdio(app_state: AppState) -> Result<(), Box<dyn std::error::Error>> {
        use rmcp::{transport::stdio, ServiceExt};
        use tracing_subscriber::{self, EnvFilter};

        tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()),
            )
            .with_writer(std::io::stderr)
            .with_ansi(false)
            .init();

        tracing::info!("Starting MTG MCP server with stdio transport");

        let service = Self::new(app_state).serve(stdio()).await.inspect_err(|e| {
            tracing::error!("serving error: {:?}", e);
        })?;

        service.waiting().await?;
        Ok(())
    }
}
