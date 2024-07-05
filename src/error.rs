/// Result type of this crate.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to load database configuration")]
    DatabaseConfig,

    #[error("failed to connect to the database")]
    DatabaseConnection,

    #[error("sqlx error: {0}")] Sqlx(#[from] sqlx::Error),

    #[error("failed to authorize user")]
    Unauthorized,

    #[error("invalid token")]
    InvalidToken,

    #[error("got an invalid signature, the JWT token may be expired")]
    InvalidSignature,

    #[error("jsonwebtoken error: {source}")] Jsonwebtoken {
        #[source]
        source: jsonwebtoken::errors::Error,
    },
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        match error.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => Self::InvalidToken,
            jsonwebtoken::errors::ErrorKind::InvalidSignature => Self::InvalidSignature,
            _ => Self::Jsonwebtoken { source: error },
        }
    }
}

impl actix_web::error::ResponseError for Error {}
