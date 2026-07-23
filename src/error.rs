use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use tracing::error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, thiserror::Error, strum_macros::AsRefStr)]
pub enum Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("{self:?}");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        (StatusCode::INTERNAL_SERVER_ERROR, ClientError::Unknown)
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    Unknown,
}
