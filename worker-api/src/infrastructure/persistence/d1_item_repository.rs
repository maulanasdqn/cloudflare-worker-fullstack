use async_trait::async_trait;
use serde::Deserialize;
use worker::D1Database;

use crate::domain::{CreateItemInput, Item, ItemRepository, PaginatedResult, UpdateItemInput};
use crate::errors::AppError;

pub struct D1ItemRepository {
    db: D1Database,
}

impl D1ItemRepository {
    pub fn new(db: D1Database) -> Self {
        Self { db }
    }
}

#[derive(Debug, Deserialize)]
struct CountResult {
    count: i64,
}

#[async_trait(?Send)]
impl ItemRepository for D1ItemRepository {
    async fn create(&self, input: CreateItemInput) -> Result<Item, AppError> {
        let stmt = self
            .db
            .prepare("INSERT INTO items (name, description) VALUES (?1, ?2) RETURNING *");

        let stmt = stmt
            .bind(&[input.name.into(), input.description.into()])
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        let result = stmt
            .first::<Item>(None)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        result.ok_or_else(|| AppError::InternalError("Failed to create item".into()))
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Item>, AppError> {
        let stmt = self.db.prepare("SELECT * FROM items WHERE id = ?1");

        let stmt = stmt
            .bind(&[(id as f64).into()])
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        stmt.first::<Item>(None)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))
    }

    async fn find_all(&self, limit: u32, offset: u32) -> Result<PaginatedResult<Item>, AppError> {
        let total = self.count().await?;

        let stmt = self
            .db
            .prepare("SELECT * FROM items ORDER BY created_at DESC LIMIT ?1 OFFSET ?2");

        let stmt = stmt
            .bind(&[(limit as f64).into(), (offset as f64).into()])
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        let result = stmt
            .all()
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        let items = result
            .results::<Item>()
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(PaginatedResult { items, total })
    }

    async fn count(&self) -> Result<u64, AppError> {
        let stmt = self.db.prepare("SELECT COUNT(*) as count FROM items");

        let result = stmt
            .first::<CountResult>(None)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        match result {
            Some(r) => Ok(r.count as u64),
            None => Ok(0),
        }
    }

    async fn update(&self, id: i64, input: UpdateItemInput) -> Result<Item, AppError> {
        let current = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Item {} not found", id)))?;

        let name = input.name.unwrap_or(current.name);
        let description = input.description.or(current.description);

        let stmt = self
            .db
            .prepare("UPDATE items SET name = ?1, description = ?2 WHERE id = ?3 RETURNING *");

        let stmt = stmt
            .bind(&[name.into(), description.into(), (id as f64).into()])
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        let result = stmt
            .first::<Item>(None)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        result.ok_or_else(|| AppError::InternalError("Failed to update item".into()))
    }

    async fn delete(&self, id: i64) -> Result<(), AppError> {
        let stmt = self.db.prepare("DELETE FROM items WHERE id = ?1");

        let stmt = stmt
            .bind(&[(id as f64).into()])
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        stmt.run()
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(())
    }
}
