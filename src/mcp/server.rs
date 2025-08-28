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
    app_state: AppState,
}

#[tool_router]
impl McpServer {
    pub fn new(app_state: AppState) -> Self {
        Self {
            tool_router: Self::tool_router(),
            app_state,
        }
    }

    fn _create_resource_text(&self, uri: &str, name: &str) -> Resource {
        rmcp::model::RawResource::new(uri, name.to_string()).no_annotation()
    }

    // Tool implementations
    #[tool(
        description = "Search for cards using filters (name, type) and optional query for additional filtering across multiple fields with pagination support"
    )]
    pub async fn search_cards(
        &self,
        Parameters(request): Parameters<crate::cards::mcp::SearchCardsRequest>,
    ) -> Result<CallToolResult, McpError> {
        let limit = request.limit.unwrap_or(10);
        let offset = request.offset.unwrap_or(0);

        match self
            .app_state
            .card_service
            .search_cards(
                &request.filters,
                request.query.as_deref(),
                Some(limit),
                Some(offset),
            )
            .await
        {
            Ok(cards) => {
                let result = if cards.is_empty() {
                    let filter_desc = if request.query.is_some() {
                        format!(
                            "matching the specified filters and query (offset: {})",
                            offset
                        )
                    } else {
                        format!("matching the specified filters (offset: {})", offset)
                    };
                    format!("No cards found {}", filter_desc)
                } else {
                    let card_names: Vec<String> =
                        cards.iter().map(|card| card.name.clone()).collect();
                    let filter_desc = if request.query.is_some() {
                        "with filters and query"
                    } else {
                        "with filters"
                    };
                    format!(
                        "Found {} cards {} (offset: {}, limit: {}): {}",
                        cards.len(),
                        filter_desc,
                        offset,
                        limit,
                        card_names.join(", ")
                    )
                };
                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            Err(e) => {
                tracing::error!("Error searching cards: {:?}", e);
                Err(McpError::resource_not_found(
                    "internal_server_error",
                    Some(json!({ "error": e.to_string() })),
                ))
            }
        }
    }

    #[tool(description = "Get a specific card by ID")]
    pub async fn get_card_by_id(
        &self,
        Parameters(request): Parameters<crate::cards::mcp::GetCardByIdRequest>,
    ) -> Result<CallToolResult, McpError> {
        match self.app_state.card_service.get_card_by_id(request.id).await {
            Ok(card) => {
                let result = format!(
                    "Card: {} (ID: {}, Type: {})",
                    card.name, card.id, card.main_type
                );
                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            Err(e) => {
                tracing::error!("Error getting card by ID: {:?}", e);
                Err(McpError::resource_not_found(
                    "internal_server_error",
                    Some(json!({ "error": e.to_string() })),
                ))
            }
        }
    }

    #[tool(description = "Get total number of cards in database")]
    pub async fn get_card_count(&self) -> Result<CallToolResult, McpError> {
        match self.app_state.card_service.get_card_count().await {
            Ok(count) => {
                let result = format!("Total cards in database: {}", count);
                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            Err(e) => {
                tracing::error!("Error getting card count: {:?}", e);
                Err(McpError::resource_not_found(
                    "internal_server_error",
                    Some(json!({ "error": e.to_string() })),
                ))
            }
        }
    }

    #[tool(description = "Find similar cards using vector similarity search based on card characteristics like type, mana cost, function, etc.")]
    pub async fn find_similar_cards(
        &self,
        Parameters(request): Parameters<crate::cards::mcp::FindSimilarCardsRequest>,
    ) -> Result<CallToolResult, McpError> {
        let limit = request.limit.unwrap_or(10);

        match self
            .app_state
            .card_service
            .find_similar_cards(&request.card_name, Some(limit))
            .await
        {
            Ok(cards) => {
                let result = if cards.is_empty() {
                    format!("No similar cards found for '{}'", request.card_name)
                } else {
                    let card_details: Vec<String> = cards
                        .iter()
                        .map(|card| {
                            let mut details = vec![
                                format!("{} ({})", card.name, card.main_type)
                            ];
                            
                            if let Some(cmc) = card.cmc {
                                details.push(format!("CMC: {}", cmc));
                            }
                            if let Some(ref mana_cost) = card.mana_cost {
                                details.push(format!("Cost: {}", mana_cost));
                            }
                            if let Some(ref colors) = card.colors {
                                if !colors.is_empty() {
                                    details.push(format!("Colors: {}", colors.join(", ")));
                                }
                            }
                            if let Some(ref keywords) = card.keywords {
                                if !keywords.is_empty() {
                                    details.push(format!("Keywords: {}", keywords.join(", ")));
                                }
                            }
                            if let Some(ref power) = card.power {
                                if let Some(ref toughness) = card.toughness {
                                    details.push(format!("{}/{}", power, toughness));
                                }
                            }
                            if let Some(ref oracle_text) = card.oracle_text {
                                if !oracle_text.is_empty() {
                                    details.push(format!("Text: {}", oracle_text));
                                }
                            }
                            
                            format!("- {}", details.join(" | "))
                        })
                        .collect();

                    format!(
                        "Found {} similar cards to '{}':\n{}",
                        cards.len(),
                        request.card_name,
                        card_details.join("\n")
                    )
                };
                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            Err(e) => {
                tracing::error!("Error finding similar cards: {:?}", e);
                Err(McpError::resource_not_found(
                    "internal_server_error",
                    Some(json!({ "error": e.to_string() })),
                ))
            }
        }
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
            instructions: Some("ManaQL MCP Server - Provides tools and prompts for Magic: The Gathering card data. Tools: search_cards, get_card_by_id, get_card_count, find_similar_cards (vector similarity search).".to_string()),
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
