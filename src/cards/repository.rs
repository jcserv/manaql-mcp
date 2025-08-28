use super::model::{CardFilters, CardModel, CardType};
use crate::error::Error;
use sqlx::{PgPool, Row};
use pgvector::Vector;

const MAX_LIMIT: i64 = 1000;

#[derive(Clone)]
pub struct CardRepository {
    pool: PgPool,
}

impl CardRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn row_to_card_model(row: &sqlx::postgres::PgRow) -> Result<CardModel, Error> {
        let id: i32 = row.try_get("id").map_err(|_| Error::InternalServerError)?;
        let name: String = row
            .try_get("name")
            .map_err(|_| Error::InternalServerError)?;
        let main_type_str: String = row
            .try_get("main_type")
            .map_err(|_| Error::InternalServerError)?;
        let main_type = CardType::from_str(&main_type_str);

        // Optional fields
        let type_line: Option<String> = row.try_get("type_line").ok();
        let oracle_text: Option<String> = row.try_get("oracle_text").ok();
        let keywords: Option<Vec<String>> = row.try_get("keywords").ok();
        let cmc: Option<f64> = row.try_get("cmc").ok();
        let mana_cost: Option<String> = row.try_get("mana_cost").ok();
        let colors: Option<Vec<String>> = row.try_get("colors").ok();
        let color_identity: Option<Vec<String>> = row.try_get("color_identity").ok();
        let power: Option<String> = row.try_get("power").ok();
        let toughness: Option<String> = row.try_get("toughness").ok();
        let games: Option<Vec<String>> = row.try_get("games").ok();
        let legalities: Option<serde_json::Value> = row.try_get("legalities").ok();
        let reserved: Option<bool> = row.try_get("reserved").ok();
        let game_changer: Option<bool> = row.try_get("game_changer").ok();
        let embedding: Option<Vector> = row.try_get("embedding").ok();

        Ok(CardModel {
            id,
            name,
            main_type,
            type_line,
            oracle_text,
            keywords,
            cmc,
            mana_cost,
            colors,
            color_identity,
            power,
            toughness,
            games,
            legalities,
            reserved,
            game_changer,
            embedding,
        })
    }

    pub async fn get(&self, id: i32) -> Result<CardModel, Error> {
        let row = sqlx::query("SELECT * FROM card WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::NotFound(format!("Card {}", id)))?;
        
        Self::row_to_card_model(&row)
    }

    pub async fn get_by_name(&self, name: &str) -> Result<CardModel, Error> {
        let row = sqlx::query("SELECT * FROM card WHERE name = $1")
            .bind(name)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::NotFound(format!("Card with name {}", name)))?;
        
        Self::row_to_card_model(&row)
    }

    fn build_where_conditions(
        &self,
        filters: &CardFilters,
        query: Option<&str>,
    ) -> (String, Vec<String>) {
        let mut params = Vec::new();

        let where_clause = format!(
            "1=1{}",
            if let Some(query_str) = query {
                // If fields are specified, search across those fields
                if let Some(fields) = &filters.fields {
                    let field_conditions: Vec<String> = fields
                        .iter()
                        .map(|field| {
                            params.push(format!("%{}%", query_str));
                            match field.to_lowercase().as_str() {
                                "name" => format!("name ILIKE ${}", params.len()),
                                "type" => format!("main_type ILIKE ${}", params.len()),
                                _ => format!("name ILIKE ${}", params.len()), // fallback to name
                            }
                        })
                        .collect();
                    format!(" AND ({})", field_conditions.join(" OR "))
                } else {
                    params.push(format!("%{}%", query_str));
                    format!(" AND name ILIKE ${}", params.len())
                }
            } else {
                String::new()
            }
        ) + &format!(
            "{}",
            if let Some(main_type) = &filters.main_type {
                params.push(main_type.as_str().to_string());
                format!(" AND main_type = ${}", params.len())
            } else {
                String::new()
            }
        );

        (where_clause, params)
    }

    pub async fn search(
        &self,
        filters: Option<CardFilters>,
        query: Option<&str>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<CardModel>, Error> {
        let filters = filters.unwrap_or_default();
        let limit = limit.unwrap_or(MAX_LIMIT);
        let offset = offset.unwrap_or(0);

        let (where_clause, params) = self.build_where_conditions(&filters, query);

        let query = format!(
            "WITH results AS (
                SELECT * 
                FROM card
                WHERE {}
            )
            SELECT * 
            FROM results
            ORDER BY name
            LIMIT ${} OFFSET ${}",
            where_clause,
            params.len() + 1,
            params.len() + 2
        );

        let mut query_builder = sqlx::query(&query);

        for param in params {
            query_builder = query_builder.bind(param);
        }

        query_builder = query_builder.bind(limit);
        query_builder = query_builder.bind(offset);

        let rows = query_builder.fetch_all(&self.pool).await.map_err(|e| {
            tracing::error!("Database query error: {:?}", e);
            Error::InternalServerError
        })?;

        let cards: Result<Vec<CardModel>, Error> =
            rows.iter().map(Self::row_to_card_model).collect();

        cards
    }

    pub async fn count(&self) -> Result<i64, Error> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM card")
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::InternalServerError)?;

        Ok(result.count.unwrap_or(0))
    }

    pub async fn find_similar_cards(
        &self,
        card_name: &str,
        limit: Option<i64>,
    ) -> Result<Vec<CardModel>, Error> {
        let limit = limit.unwrap_or(10);

        // First, get the target card's embedding
        let target_card = self.get_by_name(card_name).await?;
        
        let embedding = target_card.embedding.ok_or_else(|| {
            Error::NotFound(format!("Card '{}' does not have an embedding", card_name))
        })?;

        // Use pgvector's cosine distance to find similar cards
        // We exclude the target card itself and order by similarity
        let query = r#"
            SELECT * FROM card 
            WHERE embedding IS NOT NULL 
            AND name != $1
            ORDER BY embedding <=> $2
            LIMIT $3
        "#;

        let rows = sqlx::query(query)
            .bind(card_name)
            .bind(embedding)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Database query error: {:?}", e);
                Error::InternalServerError
            })?;

        let cards: Result<Vec<CardModel>, Error> =
            rows.iter().map(Self::row_to_card_model).collect();

        cards
    }
}
