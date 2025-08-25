use super::model::{CardFilters, CardModel, CardType};
use crate::error::Error;
use sqlx::PgPool;

const MAX_LIMIT: i64 = 1000;

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
        filters: Option<CardFilters>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<CardModel>, Error> {
        let filters = filters.unwrap_or_default();

        if let Some(main_type) = filters.main_type {
            self.list_by_type(main_type, limit, offset).await
        } else {
            self.list_all(limit, offset).await
        }
    }

    pub async fn list_all(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<CardModel>, Error> {
        let limit = limit.unwrap_or(MAX_LIMIT);
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

    pub async fn list_by_type(
        &self,
        main_type: CardType,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<CardModel>, Error> {
        let limit = limit.unwrap_or(MAX_LIMIT);
        let offset = offset.unwrap_or(0);
        let main_type_str = main_type.as_str();

        sqlx::query_as!(
            CardModel,
            "SELECT * FROM card WHERE main_type = $1 ORDER BY name LIMIT $2 OFFSET $3",
            main_type_str,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| Error::InternalServerError)
    }

    pub async fn count(&self) -> Result<i64, Error> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM card")
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::InternalServerError)?;

        Ok(result.count.unwrap_or(0))
    }
}
