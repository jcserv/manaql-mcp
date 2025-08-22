mod model;
pub mod repository;
mod routes;

pub use model::{BookModel, CreateBookSchema, UpdateBookSchema};
pub use routes::config;