use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub struct CustomError {
    pub message: String,
    pub code: super::graphql_handler::ErrorCode,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{code}: {message}",
            message = self.message,
            code = self.code.to_string()
        )
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("TypeError: {0:#}")]
    InvalidTypeError(String),
    #[error("NotFoundError: {0:#}")]
    NotFoundError(anyhow::Error),
    #[error("Unauthorized: {0:#}")]
    UnauthorizedError(anyhow::Error),
    #[error("DuplicatedError: {0:#}")]
    InternalServerError(anyhow::Error),
    #[error("DatabaseConnectionError: {0:#}")]
    DatabaseConnectionError(String),
    #[error("{0:#}")]
    CustomError(CustomError),
    #[error("ListCountLimitExceededError: {0:#}")]
    ListCountLimitExceededError(anyhow::Error),
}

impl AppError {
    pub fn get_inner_error_ref(&self) -> Option<&anyhow::Error> {
        match self {
            AppError::InternalServerError(e)
            | AppError::NotFoundError(e)
            | AppError::UnauthorizedError(e)
            | AppError::ListCountLimitExceededError(e) => Some(e),

            _ => None,
        }
    }
    pub fn type_error(s: &str) -> AppError {
        AppError::InvalidTypeError(s.to_string())
    }
    pub fn database_connection_error(s: &str) -> AppError {
        AppError::DatabaseConnectionError(s.to_string())
    }
}

impl ResponseError for AppError {}
