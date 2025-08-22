use derive_more::Display;

#[derive(Debug, Display, Clone)]
pub enum Error {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    #[display(fmt = "NotFound: {}", _0)]
    NotFound(String),
}
