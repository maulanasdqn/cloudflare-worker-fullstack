use worker::Request;

use crate::errors::AppError;

pub struct RequestValidator {
    pub max_body_size: usize,
    pub allowed_content_types: Vec<String>,
}

impl Default for RequestValidator {
    fn default() -> Self {
        Self {
            max_body_size: 1024 * 1024,
            allowed_content_types: vec![
                "application/json".to_string(),
                "application/x-www-form-urlencoded".to_string(),
            ],
        }
    }
}

impl RequestValidator {
    pub fn new(max_body_size: usize) -> Self {
        Self {
            max_body_size,
            ..Default::default()
        }
    }

    pub fn validate_content_length(&self, req: &Request) -> Result<(), AppError> {
        if let Ok(Some(length_str)) = req.headers().get("Content-Length") {
            if let Ok(length) = length_str.parse::<usize>() {
                if length > self.max_body_size {
                    return Err(AppError::BadRequest(format!(
                        "Request body too large. Maximum size is {} bytes",
                        self.max_body_size
                    )));
                }
            }
        }
        Ok(())
    }

    pub fn validate_content_type(&self, req: &Request) -> Result<(), AppError> {
        let method = req.method();
        if method == worker::Method::Get || method == worker::Method::Delete {
            return Ok(());
        }

        let content_type = req
            .headers()
            .get("Content-Type")
            .ok()
            .flatten()
            .unwrap_or_default();

        if content_type.is_empty() {
            return Ok(());
        }

        let base_type = content_type.split(';').next().unwrap_or("").trim();

        if !self
            .allowed_content_types
            .iter()
            .any(|ct| ct == base_type)
        {
            return Err(AppError::BadRequest(format!(
                "Content-Type '{}' not allowed",
                base_type
            )));
        }

        Ok(())
    }

    pub fn validate(&self, req: &Request) -> Result<(), AppError> {
        self.validate_content_length(req)?;
        self.validate_content_type(req)?;
        Ok(())
    }
}

pub fn sanitize_string(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .take(10000)
        .collect()
}

pub fn is_valid_id(id: i64) -> bool {
    id > 0
}
