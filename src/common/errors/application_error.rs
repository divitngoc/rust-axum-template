use axum::response::{IntoResponse, Response};
use diesel::result::DatabaseErrorKind;
use http::StatusCode;
use serde::Serialize;
use tracing::error;
use crate::common::errors::db_error::DbError;
use crate::common::errors::api_error_response::{ErrorResponse};
use diesel::result::{Error::DatabaseError};

#[derive(Debug, Serialize)]
pub enum ApplicationError {
    ConflictError,
    NotFound,
    InternalServerError,
    LoginError,
    Unauthorized,
    Forbidden,
    LimitReached(String),
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        match self {
            Self::ConflictError => StatusCode::CONFLICT.into_response(),
            Self::NotFound => StatusCode::NOT_FOUND.into_response(),
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Self::LoginError => StatusCode::BAD_REQUEST.into_response(),
            Self::Unauthorized => StatusCode::UNAUTHORIZED.into_response(),
            Self::Forbidden => StatusCode::FORBIDDEN.into_response(),
            Self::LimitReached(message) => ErrorResponse::build(StatusCode::FORBIDDEN, message).into_response(),
        }
    }
}

impl From<DbError> for ApplicationError {
    fn from(error: DbError) -> Self {
        match error {
            DbError::PoolError(e) => {
                error!("Error with connection pool {:?}", e);
                Self::InternalServerError
            },
            DbError::DieselError(e) => {
                match e {
                    DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                        Self::ConflictError
                    },
                    // Handle other kinds of database errors here
                    _ => {
                        error!("Error with query {:?}", e);
                        Self::InternalServerError
                    },
                }
            },
        }
    }
}

// For DB Transactions errors
impl From<diesel::result::Error> for ApplicationError {
    fn from(e: diesel::result::Error) -> Self {
        error!("Error with query {:?}", e);
        Self::InternalServerError
    }
}

impl From<headers::Error> for ApplicationError {
    fn from(e: headers::Error) -> Self {
        error!("Error with query {:?}", e);
        Self::InternalServerError
    }
}