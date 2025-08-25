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
        query: &str,
        limit: Option<i32>,
    ) -> Result<Vec<CardModel>, Error> {
        // TODO: add search to the repository
        let cards = self
            .repository
            .list(None, Some(limit.unwrap_or(100) as i64), Some(0))
            .await?;

        let filtered_cards: Vec<CardModel> = cards
            .into_iter()
            .filter(|card| card.name.to_lowercase().contains(&query.to_lowercase()))
            .take(limit.unwrap_or(10) as usize)
            .collect();

        Ok(filtered_cards)
    }

    pub async fn get_card_by_id(&self, id: i32) -> Result<CardModel, Error> {
        self.repository.get(id).await
    }

    pub async fn get_cards_by_type(
        &self,
        card_type: &str,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<CardModel>, Error> {
        let card_type_enum = CardType::from_str(card_type);

        let filters = CardFilters {
            main_type: Some(card_type_enum),
        };

        self.repository
            .list(
                Some(filters),
                Some(limit.unwrap_or(100) as i64),
                Some(offset.unwrap_or(0) as i64),
            )
            .await
    }

    pub async fn get_card_count(&self) -> Result<i64, Error> {
        self.repository.count().await
    }
}
