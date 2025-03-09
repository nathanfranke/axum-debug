use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("env error: {0}")]
    Env(#[from] dotenvy::Error),
    #[error("log error: {0}")]
    Log(#[from] log::SetLoggerError),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            _ => (StatusCode::INTERNAL_SERVER_ERROR,).into_response()
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
