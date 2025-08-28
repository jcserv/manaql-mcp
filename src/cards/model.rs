use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;
use pgvector::Vector;

#[derive(Debug, FromRow)]
pub struct CardModel {
    pub id: i32,
    pub name: String,
    pub main_type: CardType,
    pub type_line: Option<String>,
    pub oracle_text: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub cmc: Option<f64>,
    pub mana_cost: Option<String>,
    pub colors: Option<Vec<String>>,
    pub color_identity: Option<Vec<String>>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub games: Option<Vec<String>>,
    pub legalities: Option<serde_json::Value>,
    pub reserved: Option<bool>,
    pub game_changer: Option<bool>,
    pub embedding: Option<Vector>,
}

// A serializable version of CardModel for MCP responses
#[derive(Debug, Serialize)]
pub struct CardResponse {
    pub id: i32,
    pub name: String,
    pub main_type: String,
    pub type_line: Option<String>,
    pub oracle_text: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub cmc: Option<f64>,
    pub mana_cost: Option<String>,
    pub colors: Option<Vec<String>>,
    pub color_identity: Option<Vec<String>>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub games: Option<Vec<String>>,
    pub legalities: Option<serde_json::Value>,
    pub reserved: Option<bool>,
    pub game_changer: Option<bool>,
}

impl From<CardModel> for CardResponse {
    fn from(card: CardModel) -> Self {
        Self {
            id: card.id,
            name: card.name,
            main_type: card.main_type.to_string(),
            type_line: card.type_line,
            oracle_text: card.oracle_text,
            keywords: card.keywords,
            cmc: card.cmc,
            mana_cost: card.mana_cost,
            colors: card.colors,
            color_identity: card.color_identity,
            power: card.power,
            toughness: card.toughness,
            games: card.games,
            legalities: card.legalities,
            reserved: card.reserved,
            game_changer: card.game_changer,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum CardType {
    Artifact,
    Battle,
    Conspiracy,
    Creature,
    Dungeon,
    Enchantment,
    Instant,
    Kindred,
    Land,
    Phenomenon,
    Plane,
    Planeswalker,
    Scheme,
    Sorcery,
    Vanguard,
    Unknown,
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl CardType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CardType::Artifact => "Artifact",
            CardType::Battle => "Battle",
            CardType::Conspiracy => "Conspiracy",
            CardType::Creature => "Creature",
            CardType::Dungeon => "Dungeon",
            CardType::Enchantment => "Enchantment",
            CardType::Instant => "Instant",
            CardType::Kindred => "Kindred",
            CardType::Land => "Land",
            CardType::Phenomenon => "Phenomenon",
            CardType::Plane => "Plane",
            CardType::Planeswalker => "Planeswalker",
            CardType::Scheme => "Scheme",
            CardType::Sorcery => "Sorcery",
            CardType::Vanguard => "Vanguard",
            CardType::Unknown => "Unknown",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "artifact" => CardType::Artifact,
            "battle" => CardType::Battle,
            "conspiracy" => CardType::Conspiracy,
            "creature" => CardType::Creature,
            "dungeon" => CardType::Dungeon,
            "enchantment" => CardType::Enchantment,
            "instant" => CardType::Instant,
            "kindred" => CardType::Kindred,
            "land" => CardType::Land,
            "phenomenon" => CardType::Phenomenon,
            "plane" => CardType::Plane,
            "planeswalker" => CardType::Planeswalker,
            "scheme" => CardType::Scheme,
            "sorcery" => CardType::Sorcery,
            "vanguard" => CardType::Vanguard,
            _ => CardType::Unknown,
        }
    }
}

impl From<String> for CardType {
    fn from(s: String) -> Self {
        Self::from_str(&s)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CardFilters {
    /// Filter cards by main type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_type: Option<CardType>,
    /// Fields to search across when a query is provided - options are: name, type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
}

impl Default for CardFilters {
    fn default() -> Self {
        Self {
            main_type: None,
            fields: None,
        }
    }
}
