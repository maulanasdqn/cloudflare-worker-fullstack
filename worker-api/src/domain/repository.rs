use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::{CreateItemInput, Item, UpdateItemInput};
use crate::errors::AppError;

#[async_trait(?Send)]
pub trait ItemRepository {
    async fn create(&self, input: CreateItemInput) -> Result<Item, AppError>;
    async fn find_by_id(&self, id: i64) -> Result<Option<Item>, AppError>;
    async fn find_all(&self) -> Result<Vec<Item>, AppError>;
    async fn update(&self, id: i64, input: UpdateItemInput) -> Result<Item, AppError>;
    async fn delete(&self, id: i64) -> Result<(), AppError>;
}

pub type DynItemRepository = Arc<dyn ItemRepository>;
