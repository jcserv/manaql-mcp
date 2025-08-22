use rust_api_template::books::repository::BookRepository;
use sqlx::postgres::PgPoolOptions;
use std::sync::Once;

static INIT: Once = Once::new();

pub async fn setup_test_db() -> BookRepository {
    INIT.call_once(|| {
        dotenv::from_filename(".env.test").ok();
    });

    let database_url = std::env::var("DATABASE_URL").expect("TEST DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate test database");

    BookRepository::new(pool)
}