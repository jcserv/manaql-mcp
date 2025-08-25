use schemars;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SearchCardsRequest {
    /// Search filters to apply
    pub filters: SearchFilters,
    /// Text query for searching across specified fields (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Maximum number of results to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Number of results to skip for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetCardByIdRequest {
    /// Card ID to retrieve
    pub id: i32,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SearchFilters {
    /// Card type to filter by - options are: Artifact, Battle, Conspiracy, Creature, Dungeon, Enchantment, Instant, Kindred, Land, Phenomenon, Plane, Planeswalker, Scheme, Sorcery, Vanguard, Unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
    /// Fields to search across when a query is provided - options are: name, type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
}
