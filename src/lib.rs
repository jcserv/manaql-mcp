pub mod cards;
pub mod error;
pub mod mcp;

use cards::repository::CardRepository;

#[derive(Clone)]
pub struct AppState {
    pub card_repo: CardRepository,
}
