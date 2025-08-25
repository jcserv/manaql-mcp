use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct CardModel {
    pub id: i32,
    pub name: String,
    pub main_type: CardType,
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
}

impl Default for CardFilters {
    fn default() -> Self {
        Self { main_type: None }
    }
}
