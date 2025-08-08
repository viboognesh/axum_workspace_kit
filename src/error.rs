use std::fmt;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    PasswordTooShort(usize),
    InvalidHashFormat,
    HashingError,
    InvalidToken,
    ServerError,
    WrongeCredentials,
    EmailExit,
    UserNoLongerExists,
    TokenNotProvided,
    PermissionDenied,
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl ErrorMessage {
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::EmptyPassword => "EmptyPassword".to_string(),
            ErrorMessage::ExceededMaxPasswordLength(_) => "ExceededMaxPasswordLength".to_string(),
            ErrorMessage::PasswordTooShort(_) => "PasswordTooShort".to_string(),
            ErrorMessage::InvalidHashFormat => "InvalidHashFormat".to_string(),
            ErrorMessage::HashingError => "HashingError".to_string(),
            ErrorMessage::InvalidToken => "InvalidToken".to_string(),
            ErrorMessage::ServerError => "ServerError".to_string(),
            ErrorMessage::WrongeCredentials => "WrongeCredentials".to_string(),
            ErrorMessage::EmailExit => "EmailExit".to_string(),
            ErrorMessage::UserNoLongerExists => "UserNoLongerExists".to_string(),
            ErrorMessage::TokenNotProvided => "TokenNotProvided".to_string(),
            ErrorMessage::PermissionDenied => "PermissionDenied".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub status: StatusCode,
    pub message: String,
}

impl HttpError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status: status,
            message: message.into(),
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, message)
    }

    pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, message)
    }

    pub fn into_http_response(self) -> Response {
        let json_response = Json(ErrorResponse {
            status: "failed".to_string(),
            message: self.message,
        });

        (self.status, json_response).into_response()
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "HTTPError: message: {} status: {}",
            self.message, self.status
        )
    }
}

impl std::error::Error for HttpError {}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        self.into_http_response()
    }
}
