use serde::{Deserialize, Serialize};

use crate::domain::{CreateItemInput, Item, UpdateItemInput};

#[derive(Debug, Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
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

#[derive(Debug, Deserialize)]
pub struct UpdateItemRequest {
    pub name: Option<String>,
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
