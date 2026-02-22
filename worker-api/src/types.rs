use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SingleResponse<T> {
    pub data: T,
    pub message: String,
}

impl<T> SingleResponse<T> {
    pub fn new(data: T, message: impl Into<String>) -> Self {
        Self {
            data,
            message: message.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub count: usize,
}

impl<T> ListResponse<T> {
    pub fn new(data: Vec<T>) -> Self {
        let count = data.len();
        Self { data, count }
    }
}
