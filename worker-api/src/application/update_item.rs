use crate::domain::{DynItemRepository, Item, UpdateItemInput};
use crate::errors::AppError;

pub struct UpdateItem {
    repo: DynItemRepository,
}

impl UpdateItem {
    pub fn new(repo: DynItemRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: i64, input: UpdateItemInput) -> Result<Item, AppError> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Item with id {} not found", id)))?;

        self.repo.update(id, input).await
    }
}
