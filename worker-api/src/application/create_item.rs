use crate::domain::{CreateItemInput, DynItemRepository, Item};
use crate::errors::AppError;

pub struct CreateItem {
    repo: DynItemRepository,
}

impl CreateItem {
    pub fn new(repo: DynItemRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, input: CreateItemInput) -> Result<Item, AppError> {
        if input.name.is_empty() {
            return Err(AppError::ValidationError("Name is required".into()));
        }
        self.repo.create(input).await
    }
}
