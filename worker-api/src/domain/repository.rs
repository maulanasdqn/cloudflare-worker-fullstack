use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::{CreateItemInput, Item, UpdateItemInput};
use crate::errors::AppError;

#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: u64,
}

#[async_trait(?Send)]
pub trait ItemRepository {
    async fn create(&self, input: CreateItemInput) -> Result<Item, AppError>;
    async fn find_by_id(&self, id: i64) -> Result<Option<Item>, AppError>;
    async fn find_all(&self, limit: u32, offset: u32) -> Result<PaginatedResult<Item>, AppError>;
    async fn count(&self) -> Result<u64, AppError>;
    async fn update(&self, id: i64, input: UpdateItemInput) -> Result<Item, AppError>;
    async fn delete(&self, id: i64) -> Result<(), AppError>;
}

pub type DynItemRepository = Arc<dyn ItemRepository>;
