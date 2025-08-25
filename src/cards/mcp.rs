use schemars;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SearchCardsRequest {
    /// Search query for card name
    pub query: String,
    /// Maximum number of results to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetCardByIdRequest {
    /// Card ID to retrieve
    pub id: i32,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetCardsByTypeRequest {
    /// Card type to filter by
    pub card_type: String,
    /// Maximum number of results to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}
