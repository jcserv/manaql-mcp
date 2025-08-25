pub mod cards;
pub mod error;
pub mod mcp;

use cards::service::CardService;

#[derive(Clone)]
pub struct AppState {
    pub card_service: CardService,
}
