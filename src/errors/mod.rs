use rocket::serde::json::Json;
use rocket::http::Status;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl ErrorResponse {
    pub fn new(error: String) -> Self {
        ErrorResponse {
            error,
            details: None,
        }
    }

    pub fn with_details(error: String, details: String) -> Self {
        ErrorResponse {
            error,
            details: Some(details),
        }
    }
}

/// Helper function to create error responses
pub fn error_response(status: Status, message: String) -> (Status, Json<ErrorResponse>) {
    (status, Json(ErrorResponse::new(message)))
}

pub fn error_response_with_details(
    status: Status,
    message: String,
    details: String,
) -> (Status, Json<ErrorResponse>) {
    (status, Json(ErrorResponse::with_details(message, details)))
}
