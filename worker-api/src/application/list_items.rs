use crate::domain::{DynItemRepository, Item};
use crate::errors::AppError;

pub struct ListItems {
    repo: DynItemRepository,
}

impl ListItems {
    pub fn new(repo: DynItemRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<Item>, AppError> {
        self.repo.find_all().await
    }
}
