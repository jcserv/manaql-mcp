use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CardModel {
    pub id: i32,
    pub name: String,
    pub main_type: String,
}
