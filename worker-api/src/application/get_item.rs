use crate::domain::{DynItemRepository, Item};
use crate::errors::AppError;

pub struct GetItem {
    repo: DynItemRepository,
}

impl GetItem {
    pub fn new(repo: DynItemRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: i64) -> Result<Item, AppError> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Item with id {} not found", id)))
    }
}
