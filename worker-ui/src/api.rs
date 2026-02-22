use gloo_net::http::Request;

use crate::types::{CreateItemRequest, Item, ListResponse, SingleResponse, UpdateItemRequest};

const API_BASE: &str = "/api/v1";

pub async fn fetch_items() -> Result<Vec<Item>, String> {
    let response = Request::get(&format!("{}/items", API_BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data: ListResponse<Item> = response.json().await.map_err(|e| e.to_string())?;

    Ok(data.data)
}

pub async fn fetch_item(id: i64) -> Result<Item, String> {
    let response = Request::get(&format!("{}/items/{}", API_BASE, id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data: SingleResponse<Item> = response.json().await.map_err(|e| e.to_string())?;

    Ok(data.data)
}

pub async fn create_item(req: CreateItemRequest) -> Result<Item, String> {
    let response = Request::post(&format!("{}/items", API_BASE))
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data: SingleResponse<Item> = response.json().await.map_err(|e| e.to_string())?;

    Ok(data.data)
}

pub async fn update_item(id: i64, req: UpdateItemRequest) -> Result<Item, String> {
    let response = Request::put(&format!("{}/items/{}", API_BASE, id))
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data: SingleResponse<Item> = response.json().await.map_err(|e| e.to_string())?;

    Ok(data.data)
}

pub async fn delete_item(id: i64) -> Result<(), String> {
    Request::delete(&format!("{}/items/{}", API_BASE, id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
