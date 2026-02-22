use crate::domain::{DynItemRepository, PaginatedResult, Item, QueryOptions};
use crate::errors::AppError;
use crate::types::ListQueryParams;

pub struct ListItems {
    repo: DynItemRepository,
}

impl ListItems {
    pub fn new(repo: DynItemRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, params: ListQueryParams) -> Result<PaginatedResult<Item>, AppError> {
        let options = QueryOptions::new(params.per_page(), params.offset())
            .with_search(params.search.clone())
            .with_sort(params.sort_by(), params.sort_order());

        self.repo.find_all(options).await
    }
}
