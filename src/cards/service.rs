use super::{model::CardModel, repository::CardRepository};
use crate::{
    cards::model::{CardFilters, CardType},
    error::Error,
};

#[derive(Clone)]
pub struct CardService {
    repository: CardRepository,
}

impl CardService {
    pub fn new(repository: CardRepository) -> Self {
        Self { repository }
    }

    pub async fn search_cards(
        &self,
        filters: &crate::cards::mcp::SearchFilters,
        query: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<CardModel>, Error> {
        let mut card_filters = CardFilters {
            main_type: filters.card_type.as_ref().map(|t| CardType::from_str(t)),
            fields: filters.fields.clone(),
        };

        // If a query is provided but no fields specified, default to searching name
        if query.is_some() && card_filters.fields.is_none() {
            card_filters.fields = Some(vec!["name".to_string()]);
        }

        self.repository
            .search(
                Some(card_filters),
                query,
                limit.map(|l| l as i64),
                offset.map(|o| o as i64),
            )
            .await
    }

    pub async fn get_card_by_id(&self, id: i32) -> Result<CardModel, Error> {
        self.repository.get(id).await
    }

    pub async fn get_card_count(&self) -> Result<i64, Error> {
        self.repository.count().await
    }
}
