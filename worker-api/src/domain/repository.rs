use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::{CreateItemInput, Item, UpdateItemInput};
use crate::errors::AppError;
use crate::types::SortOrder;

#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: u64,
}

#[derive(Debug, Clone, Default)]
pub struct QueryOptions {
    pub limit: u32,
    pub offset: u32,
    pub search: Option<String>,
    pub sort_by: String,
    pub sort_order: SortOrder,
}

impl QueryOptions {
    pub fn new(limit: u32, offset: u32) -> Self {
        Self {
            limit,
            offset,
            search: None,
            sort_by: "created_at".to_string(),
            sort_order: SortOrder::Desc,
        }
    }

    pub fn with_search(mut self, search: Option<String>) -> Self {
        self.search = search;
        self
    }

    pub fn with_sort(mut self, sort_by: &str, sort_order: SortOrder) -> Self {
        self.sort_by = sort_by.to_string();
        self.sort_order = sort_order;
        self
    }
}

#[async_trait(?Send)]
pub trait ItemRepository {
    async fn create(&self, input: CreateItemInput) -> Result<Item, AppError>;
    async fn find_by_id(&self, id: i64) -> Result<Option<Item>, AppError>;
    async fn find_all(&self, options: QueryOptions) -> Result<PaginatedResult<Item>, AppError>;
    async fn count(&self, search: Option<&str>) -> Result<u64, AppError>;
    async fn update(&self, id: i64, input: UpdateItemInput) -> Result<Item, AppError>;
    async fn delete(&self, id: i64) -> Result<(), AppError>;
}

pub type DynItemRepository = Arc<dyn ItemRepository>;
