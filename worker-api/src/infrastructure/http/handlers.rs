use worker::{Request, Response, RouteContext};

use crate::application::{CreateItem, DeleteItem, GetItem, ListItems, UpdateItem};
use crate::domain::DynItemRepository;
use crate::errors::AppError;
use crate::infrastructure::http::dto::{CreateItemRequest, ItemResponse, UpdateItemRequest};
use crate::types::{ListResponse, SingleResponse};

fn handle_error(err: AppError) -> worker::Result<Response> {
    Ok(Response::from(err))
}

pub async fn list_items_handler(
    _req: Request,
    ctx: RouteContext<DynItemRepository>,
) -> worker::Result<Response> {
    let repo = ctx.data;
    let use_case = ListItems::new(repo);

    match use_case.execute().await {
        Ok(items) => {
            let response: Vec<ItemResponse> = items.into_iter().map(Into::into).collect();
            Response::from_json(&ListResponse::new(response))
        }
        Err(e) => handle_error(e),
    }
}

pub async fn get_item_handler(
    _req: Request,
    ctx: RouteContext<DynItemRepository>,
) -> worker::Result<Response> {
    let id: i64 = match ctx.param("id") {
        Some(id_str) => match id_str.parse() {
            Ok(id) => id,
            Err(_) => return handle_error(AppError::BadRequest("Invalid id parameter".into())),
        },
        None => return handle_error(AppError::BadRequest("Missing id parameter".into())),
    };

    let repo = ctx.data;
    let use_case = GetItem::new(repo);

    match use_case.execute(id).await {
        Ok(item) => {
            Response::from_json(&SingleResponse::new(ItemResponse::from(item), "Item retrieved"))
        }
        Err(e) => handle_error(e),
    }
}

pub async fn create_item_handler(
    mut req: Request,
    ctx: RouteContext<DynItemRepository>,
) -> worker::Result<Response> {
    let payload: CreateItemRequest = match req.json().await {
        Ok(p) => p,
        Err(e) => return handle_error(AppError::BadRequest(e.to_string())),
    };

    let repo = ctx.data;
    let use_case = CreateItem::new(repo);

    match use_case.execute(payload.into()).await {
        Ok(item) => {
            Response::from_json(&SingleResponse::new(ItemResponse::from(item), "Item created"))
        }
        Err(e) => handle_error(e),
    }
}

pub async fn update_item_handler(
    mut req: Request,
    ctx: RouteContext<DynItemRepository>,
) -> worker::Result<Response> {
    let id: i64 = match ctx.param("id") {
        Some(id_str) => match id_str.parse() {
            Ok(id) => id,
            Err(_) => return handle_error(AppError::BadRequest("Invalid id parameter".into())),
        },
        None => return handle_error(AppError::BadRequest("Missing id parameter".into())),
    };

    let payload: UpdateItemRequest = match req.json().await {
        Ok(p) => p,
        Err(e) => return handle_error(AppError::BadRequest(e.to_string())),
    };

    let repo = ctx.data;
    let use_case = UpdateItem::new(repo);

    match use_case.execute(id, payload.into()).await {
        Ok(item) => {
            Response::from_json(&SingleResponse::new(ItemResponse::from(item), "Item updated"))
        }
        Err(e) => handle_error(e),
    }
}

pub async fn delete_item_handler(
    _req: Request,
    ctx: RouteContext<DynItemRepository>,
) -> worker::Result<Response> {
    let id: i64 = match ctx.param("id") {
        Some(id_str) => match id_str.parse() {
            Ok(id) => id,
            Err(_) => return handle_error(AppError::BadRequest("Invalid id parameter".into())),
        },
        None => return handle_error(AppError::BadRequest("Missing id parameter".into())),
    };

    let repo = ctx.data;
    let use_case = DeleteItem::new(repo);

    match use_case.execute(id).await {
        Ok(()) => Response::from_json(&SingleResponse::new((), "Item deleted")),
        Err(e) => handle_error(e),
    }
}
