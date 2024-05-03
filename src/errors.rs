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

    // -- Session Errors
    InvalidToken { message: String },
    RefreshTokenCreationError { message: String },
    IdTokenCreationError { message: String },
    PublicKeyLoadError { message: String },
    PrivateKeyLoadError { message: String },
    SignatureVerificationError { message: String },
    ExpiredSignature { message: String },
    SessionExpired { message: String },

    
    // -- Encryption Errors
    KeyNotFound { message: String },

    ServerError { message: String },
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
            // -- Model errors
            Self::InvalidPayload { message: _ } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            // -- User Errors
            Self::UserNotFound { message: _ } => {
                (StatusCode::NOT_FOUND, ClientError::USER_NOT_FOUND)
            }

            Self::UserAlreadyExists { message: _ } => {
                (StatusCode::FOUND, ClientError::USER_ALREADY_EXISTS)
            }

            // -- Password Errors
            Self::InvalidPassword { message: _ } => {
                (StatusCode::UNAUTHORIZED, ClientError::INVALID_PASSWORD)
            }

            Self::KeyNotFound { message: _ } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),

            Self::ResetPasswordLinkExpired { message: _ } => (
                StatusCode::UNAUTHORIZED,
                ClientError::RESET_PASSWORD_LINK_EXPIRED,
            ),

            // -- Session Errors
            Self::PublicKeyLoadError { message: _ } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),

            Self::PrivateKeyLoadError { message: _ } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),

            Self::SignatureVerificationError { message: _ } => (
                StatusCode::UNAUTHORIZED,
                ClientError::SIGNATURE_VERIFICATION_ERROR,
            ),

            Self::ExpiredSignature { message: _ } => {
                (StatusCode::UNAUTHORIZED, ClientError::EXPIRED_SIGNATURE)
            }

            Self::InvalidToken { message: _ } => {
                (StatusCode::UNAUTHORIZED, ClientError::INVALID_TOKEN)
            }
            Self::IdTokenCreationError { message: _ } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),

            Self::RefreshTokenCreationError { message: _ } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
            
            Self::SessionExpired { message: _ } => (
                StatusCode::UNAUTHORIZED,
                ClientError::SESSION_EXPIRED,
            ),

            Self::ServerError { message: _ } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),

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
    INVALID_TOKEN,
    SIGNATURE_VERIFICATION_ERROR,
    EXPIRED_SIGNATURE,
    SESSION_EXPIRED
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// end region: --- Error Boilerplate
