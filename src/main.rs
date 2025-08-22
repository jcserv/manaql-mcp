use dotenv::dotenv;
use mtg_mcp::cards::repository::CardRepository;
use sqlx::postgres::PgPoolOptions;

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    card_repo: CardRepository,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connected to the database");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let card_repo = CardRepository::new(pool.clone());
    let _app_state = AppState { card_repo };

    // start_mcp_server(app_state).await?;

    Ok(())
}
