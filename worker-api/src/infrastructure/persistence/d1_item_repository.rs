use async_trait::async_trait;
use serde::Deserialize;
use worker::D1Database;

use crate::domain::{CreateItemInput, Item, ItemRepository, PaginatedResult, QueryOptions, UpdateItemInput};
use crate::errors::AppError;
use crate::types::SortOrder;

pub struct D1ItemRepository {
    db: D1Database,
}

impl D1ItemRepository {
    pub fn new(db: D1Database) -> Self {
        Self { db }
    }

    fn validate_sort_field(field: &str) -> &str {
        match field {
            "id" | "name" | "created_at" => field,
            _ => "created_at",
        }
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

    async fn find_all(&self, options: QueryOptions) -> Result<PaginatedResult<Item>, AppError> {
        let total = self.count(options.search.as_deref()).await?;

        let sort_field = Self::validate_sort_field(&options.sort_by);
        let sort_dir = match options.sort_order {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        };

        let (query, bindings) = if let Some(ref search) = options.search {
            let search_pattern = format!("%{}%", search);
            (
                format!(
                    "SELECT * FROM items WHERE name LIKE ?1 OR description LIKE ?1 ORDER BY {} {} LIMIT ?2 OFFSET ?3",
                    sort_field, sort_dir
                ),
                vec![
                    search_pattern.into(),
                    (options.limit as f64).into(),
                    (options.offset as f64).into(),
                ],
            )
        } else {
            (
                format!(
                    "SELECT * FROM items ORDER BY {} {} LIMIT ?1 OFFSET ?2",
                    sort_field, sort_dir
                ),
                vec![
                    (options.limit as f64).into(),
                    (options.offset as f64).into(),
                ],
            )
        };

        let stmt = self.db.prepare(&query);
        let stmt = stmt
            .bind(&bindings)
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

    async fn count(&self, search: Option<&str>) -> Result<u64, AppError> {
        let (query, bindings) = if let Some(search) = search {
            let search_pattern = format!("%{}%", search);
            (
                "SELECT COUNT(*) as count FROM items WHERE name LIKE ?1 OR description LIKE ?1",
                vec![search_pattern.into()],
            )
        } else {
            ("SELECT COUNT(*) as count FROM items", vec![])
        };

        let stmt = self.db.prepare(query);
        let stmt = if bindings.is_empty() {
            stmt
        } else {
            stmt.bind(&bindings)
                .map_err(|e| AppError::InternalError(e.to_string()))?
        };

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
