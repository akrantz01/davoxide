use axum::{
    http::{
        header::{self, HeaderValue},
        method::InvalidMethod,
        StatusCode,
    },
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;
use std::{
    error::Error as StdError,
    fmt::{self, Display, Formatter},
};
use tracing::error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// For when the credentials are invalid
    Unauthorized,
    /// For when a user does not have the correct permissions
    InvalidPermissions,
    /// For when a requested resource was not found
    NotFound,
    /// For when the incoming request is invalid
    BadRequest,
    /// Used when an unexpected and unhandleable error occurs
    /// i.e. database or file system errors
    Unexpected(Box<dyn StdError + Send + Sync>),
}

impl Error {
    /// Check if the error resulted from not being able to find a record
    pub fn is_not_found(&self) -> bool {
        matches!(self, Error::NotFound)
    }
}

impl From<DbErr> for Error {
    fn from(e: DbErr) -> Self {
        match e {
            DbErr::RecordNotFound(_) => Error::NotFound,
            source => Error::Unexpected(Box::new(source)),
        }
    }
}

impl From<InvalidMethod> for Error {
    fn from(_: InvalidMethod) -> Self {
        Error::BadRequest
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unauthorized => write!(f, "unauthorized"),
            Self::InvalidPermissions => write!(f, "permission denied"),
            Self::NotFound => write!(f, "not found"),
            Self::BadRequest => write!(f, "bad request"),
            Self::Unexpected(e) => {
                error!(error = %e, source = ?e.source(), "an unexpected error occurred");
                write!(f, "an unexpected error occurred")
            }
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::Unauthorized => {
                let mut response = static_response("unauthorized", StatusCode::UNAUTHORIZED);
                response.headers_mut().insert(
                    header::WWW_AUTHENTICATE,
                    HeaderValue::from_static("Basic realm=\"davoxide\""),
                );
                response
            }
            Self::InvalidPermissions => static_response("permission denied", StatusCode::FORBIDDEN),
            Self::NotFound => static_response("not found", StatusCode::NOT_FOUND),
            Self::BadRequest => static_response("bad request", StatusCode::BAD_REQUEST),
            Self::Unexpected(e) => {
                error!(error = %e, source = ?e.source(), "an unexpected error occurred");
                static_response("internal server error", StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

fn static_response(message: &'static str, status: StatusCode) -> Response {
    (status, message).into_response()
}
