use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use tracing::error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, thiserror::Error, strum_macros::AsRefStr)]
pub enum Error {
    #[error("login failed")]
    LoginFail,
    #[error("Authentication - No token in cookies")]
    AuthFailNoAuthTokenCookie,
    #[error("Authentication - Wrong token format")]
    AuthFailTokenWrongFormat,
    #[error("Ticket delete - ID not found")]
    TicketDeleteFailIdNotFound { id: u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("{:<12} - {self:?}", "   ");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            Self::AuthFailTokenWrongFormat | Self::AuthFailNoAuthTokenCookie => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }
            Self::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
