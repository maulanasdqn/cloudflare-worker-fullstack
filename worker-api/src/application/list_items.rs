use crate::domain::{DynItemRepository, PaginatedResult, Item};
use crate::errors::AppError;
use crate::types::PaginationParams;

pub struct ListItems {
    repo: DynItemRepository,
}

impl ListItems {
    pub fn new(repo: DynItemRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, params: PaginationParams) -> Result<PaginatedResult<Item>, AppError> {
        let limit = params.per_page();
        let offset = params.offset();
        self.repo.find_all(limit, offset).await
    }
}
