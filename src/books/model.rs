use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BookModel {
    pub id: i32,
    pub title: String,
    pub author_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateBookSchema {
    pub title: String,
    pub author_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateBookSchema {
    pub title: Option<String>,
}
