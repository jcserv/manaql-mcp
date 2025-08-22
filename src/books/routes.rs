use actix_web::{get, post, patch, delete, web, HttpResponse};
use super::{model::*, repository::BookRepository};
use crate::error::Error;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/v1/books")
        .service(create_book)
        .service(read_book)
        .service(update_book)
        .service(delete_book);

    conf.service(scope);
}

#[post("")]
async fn create_book(
    repo: web::Data<BookRepository>,
    book: web::Json<CreateBookSchema>,
) -> Result<HttpResponse, Error> {
    let book = repo.create(book.into_inner()).await?;
    Ok(HttpResponse::Created().json(book))
}

#[get("/{id}")]
async fn read_book(
    repo: web::Data<BookRepository>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let book = repo.get(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(book))
}

#[patch("/{id}")]
async fn update_book(
    repo: web::Data<BookRepository>,
    id: web::Path<i32>,
    update: web::Json<UpdateBookSchema>,
) -> Result<HttpResponse, Error> {
    let book = repo.update(id.into_inner(), update.into_inner()).await?;
    Ok(HttpResponse::Ok().json(book))
}

#[delete("/{id}")]
async fn delete_book(
    repo: web::Data<BookRepository>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    repo.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}