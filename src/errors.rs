use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, strum_macros::AsRefStr, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Model errors
    InvalidPayload { message: String },

    // -- User Errors
    UserNotFound { message: String },
    UserAlreadyExists { message: String },

    // -- Password Errors
    InvalidPassword { message: String },
    ResetPasswordLinkExpired { message: String },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("{:?}", self);

        // Create a placeholder Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the error message into the response body
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::InvalidPayload { message: _ } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            Self::UserNotFound { message: _ } => {
                (StatusCode::NOT_FOUND, ClientError::USER_NOT_FOUND)
            }

            Self::UserAlreadyExists { message: _ } => {
                (StatusCode::FOUND, ClientError::USER_ALREADY_EXISTS)
            }

            Self::InvalidPassword { message: _ } => {
                (StatusCode::UNAUTHORIZED, ClientError::INVALID_PASSWORD)
            }

            Self::ResetPasswordLinkExpired { message: _ } => {
                (StatusCode::UNAUTHORIZED, ClientError::RESET_PASSWORD_LINK_EXPIRED)
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
    USER_NOT_FOUND,
    INVALID_PARAMS,
    SERVICE_ERROR,
    USER_ALREADY_EXISTS,
    INVALID_PASSWORD,
    RESET_PASSWORD_LINK_EXPIRED,
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// end region: --- Error Boilerplate
