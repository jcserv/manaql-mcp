use sqlx::PgPool;
use crate::error::Error;
use super::model::{BookModel, CreateBookSchema, UpdateBookSchema};

#[derive(Clone)]
pub struct BookRepository {
    pool: PgPool,
}

impl BookRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new_book: CreateBookSchema) -> Result<BookModel, Error> {
        sqlx::query_as!(
            BookModel,
            "INSERT INTO Book (title, author_id) VALUES ($1, $2) RETURNING *",
            new_book.title,
            new_book.author_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| Error::InternalServerError)
    }

    pub async fn get(&self, id: i32) -> Result<BookModel, Error> {
        sqlx::query_as!(BookModel, "SELECT * FROM Book WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::NotFound(format!("Book {}", id)))
    }

    pub async fn update(&self, id: i32, update: UpdateBookSchema) -> Result<BookModel, Error> {
        let book = self.get(id).await?;
        let title = update.title.unwrap_or(book.title);
        
        sqlx::query_as!(
            BookModel,
            "UPDATE Book SET title = $1 WHERE id = $2 RETURNING *",
            title,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| Error::InternalServerError)
    }

    pub async fn delete(&self, id: i32) -> Result<(), Error> {
        sqlx::query!("DELETE FROM Book WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|_| Error::NotFound(format!("Book {}", id)))?;
        Ok(())
    }
}