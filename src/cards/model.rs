use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct CardModel {
    pub id: i32,
    pub name: String,
    pub main_type: String,
}
