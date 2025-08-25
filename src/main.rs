use dotenv::dotenv;
use manaql_mcp::mcp::McpServer;
use manaql_mcp::{cards::repository::CardRepository, AppState};
use sqlx::postgres::PgPoolOptions;
use tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            tracing::info!("Connected to the database");
            pool
        }
        Err(err) => {
            tracing::error!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let card_repo = CardRepository::new(pool.clone());
    let app_state = AppState { card_repo };

    McpServer::start_stdio(app_state).await?;

    Ok(())
}
