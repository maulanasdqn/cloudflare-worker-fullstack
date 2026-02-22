use serde::Serialize;
use worker::Response;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    ValidationError(String),
    InternalError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl From<AppError> for worker::Error {
    fn from(err: AppError) -> Self {
        worker::Error::from(err.to_string())
    }
}

impl From<AppError> for Response {
    fn from(err: AppError) -> Self {
        let (status, message) = match &err {
            AppError::NotFound(msg) => (404, msg.clone()),
            AppError::BadRequest(msg) => (400, msg.clone()),
            AppError::ValidationError(msg) => (400, msg.clone()),
            AppError::InternalError(msg) => (500, msg.clone()),
        };

        Response::from_json(&ErrorResponse { message })
            .unwrap_or_else(|_| Response::error("Internal Error", 500).unwrap())
            .with_status(status)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal Error: {}", msg),
        }
    }
}

impl From<worker::Error> for AppError {
    fn from(err: worker::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}
