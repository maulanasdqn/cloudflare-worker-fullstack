use gloo_net::http::Request;

use crate::types::{
    CreateItemRequest, Item, ListResponse, PaginationMeta, SingleResponse, UpdateItemRequest,
};

const API_BASE: &str = "/api/v1";

#[derive(Clone, Default)]
pub struct FetchParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

pub struct FetchResult {
    pub items: Vec<Item>,
    pub pagination: PaginationMeta,
}

pub async fn fetch_items(params: FetchParams) -> Result<FetchResult, String> {
    let mut url = format!("{}/items", API_BASE);
    let mut query_parts = Vec::new();

    if let Some(page) = params.page {
        query_parts.push(format!("page={}", page));
    }
    if let Some(per_page) = params.per_page {
        query_parts.push(format!("per_page={}", per_page));
    }
    if let Some(search) = &params.search {
        if !search.is_empty() {
            query_parts.push(format!("search={}", search));
        }
    }
    if let Some(sort_by) = &params.sort_by {
        query_parts.push(format!("sort_by={}", sort_by));
    }
    if let Some(sort_order) = &params.sort_order {
        query_parts.push(format!("sort_order={}", sort_order));
    }

    if !query_parts.is_empty() {
        url = format!("{}?{}", url, query_parts.join("&"));
    }

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data: ListResponse<Item> = response.json().await.map_err(|e| e.to_string())?;

    Ok(FetchResult {
        items: data.data,
        pagination: data.meta,
    })
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
