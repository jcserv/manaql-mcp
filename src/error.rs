use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    #[display(fmt = "NotFound: {}", _0)]
    NotFound(String),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::InternalServerError => HttpResponse::InternalServerError().json("Internal Server Error"),
            Error::NotFound(msg) => HttpResponse::NotFound().json(msg),
        }
    }
}
