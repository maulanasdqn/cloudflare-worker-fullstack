use crate::domain::DynItemRepository;
use crate::errors::AppError;

pub struct DeleteItem {
    repo: DynItemRepository,
}

impl DeleteItem {
    pub fn new(repo: DynItemRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: i64) -> Result<(), AppError> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Item with id {} not found", id)))?;

        self.repo.delete(id).await
    }
}
