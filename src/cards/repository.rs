use super::model::CardModel;
use crate::error::Error;
use sqlx::PgPool;

#[derive(Clone)]
pub struct CardRepository {
    pool: PgPool,
}

impl CardRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get(&self, id: i32) -> Result<CardModel, Error> {
        sqlx::query_as!(CardModel, "SELECT * FROM card WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::NotFound(format!("Card {}", id)))
    }

    pub async fn get_by_name(&self, name: &str) -> Result<CardModel, Error> {
        sqlx::query_as!(CardModel, "SELECT * FROM card WHERE name = $1", name)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::NotFound(format!("Card with name {}", name)))
    }

    pub async fn list(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<CardModel>, Error> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);

        sqlx::query_as!(
            CardModel,
            "SELECT * FROM card ORDER BY name LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| Error::InternalServerError)
    }
}
