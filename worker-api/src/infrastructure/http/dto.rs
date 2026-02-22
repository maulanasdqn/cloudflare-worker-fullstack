use serde::{Deserialize, Serialize};
use zod_rs::prelude::*;

use crate::domain::{CreateItemInput, Item, UpdateItemInput};

#[derive(Debug, Deserialize, ZodSchema)]
pub struct CreateItemRequest {
    #[zod(min_length(1), max_length(255))]
    pub name: String,
    #[zod(max_length(1000))]
    pub description: Option<String>,
}

impl From<CreateItemRequest> for CreateItemInput {
    fn from(req: CreateItemRequest) -> Self {
        CreateItemInput {
            name: req.name,
            description: req.description,
        }
    }
}

#[derive(Debug, Deserialize, ZodSchema)]
pub struct UpdateItemRequest {
    #[zod(min_length(1), max_length(255))]
    pub name: Option<String>,
    #[zod(max_length(1000))]
    pub description: Option<String>,
}

impl From<UpdateItemRequest> for UpdateItemInput {
    fn from(req: UpdateItemRequest) -> Self {
        UpdateItemInput {
            name: req.name,
            description: req.description,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ItemResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
}

impl From<Item> for ItemResponse {
    fn from(item: Item) -> Self {
        ItemResponse {
            id: item.id,
            name: item.name,
            description: item.description,
            created_at: item.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ValidationErrorResponse {
    pub message: String,
    pub errors: Vec<String>,
}
