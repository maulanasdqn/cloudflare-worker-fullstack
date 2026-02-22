use serde_json::Value;
use worker::{Request, Response, RouteContext};
use zod_rs::prelude::*;
use zod_rs_util::ValidationIssue;

use crate::application::{CreateItem, DeleteItem, GetItem, ListItems, UpdateItem};
use crate::domain::DynItemRepository;
use crate::errors::AppError;
use crate::infrastructure::http::dto::{
    CreateItemRequest, ItemResponse, UpdateItemRequest, ValidationErrorResponse,
};
use crate::types::{ListQueryParams, ListResponse, PaginationMeta, SingleResponse, SortOrder};

fn handle_error(err: AppError) -> worker::Result<Response> {
    Ok(Response::from(err))
}

fn handle_validation_error(errors: Vec<String>) -> worker::Result<Response> {
    let response = ValidationErrorResponse {
        message: "Validation failed".to_string(),
        errors,
    };
    Response::from_json(&response).map(|r| r.with_status(400))
}

fn parse_list_query_params(req: &Request) -> ListQueryParams {
    let url = match req.url().ok() {
        Some(u) => u,
        None => return ListQueryParams::default(),
    };

    let page = url
        .query_pairs()
        .find(|(k, _)| k == "page")
        .and_then(|(_, v)| v.parse().ok());

    let per_page = url
        .query_pairs()
        .find(|(k, _)| k == "per_page")
        .and_then(|(_, v)| v.parse().ok());

    let search = url
        .query_pairs()
        .find(|(k, _)| k == "search")
        .map(|(_, v)| v.to_string())
        .filter(|s| !s.is_empty());

    let sort_by = url
        .query_pairs()
        .find(|(k, _)| k == "sort_by")
        .map(|(_, v)| v.to_string());

    let sort_order = url
        .query_pairs()
        .find(|(k, _)| k == "sort_order")
        .and_then(|(_, v)| match v.as_ref() {
            "asc" => Some(SortOrder::Asc),
            "desc" => Some(SortOrder::Desc),
            _ => None,
        });

    ListQueryParams {
        page,
        per_page,
        search,
        sort_by,
        sort_order,
    }
}

pub async fn list_items_handler(
    req: Request,
    ctx: RouteContext<DynItemRepository>,
) -> worker::Result<Response> {
    let params = parse_list_query_params(&req);
    let repo = ctx.data;
    let use_case = ListItems::new(repo);

    match use_case.execute(params.clone()).await {
        Ok(result) => {
            let items: Vec<ItemResponse> = result.items.into_iter().map(Into::into).collect();
            let meta = PaginationMeta::new(params.page(), params.per_page(), result.total);
            Response::from_json(&ListResponse::new(items, meta))
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
            Ok(id) if id > 0 => id,
            Ok(_) => return handle_error(AppError::BadRequest("Id must be positive".into())),
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
    let json_value: Value = match req.json().await {
        Ok(v) => v,
        Err(e) => return handle_error(AppError::BadRequest(e.to_string())),
    };

    match CreateItemRequest::validate_and_parse(&json_value) {
        Ok(payload) => {
            let repo = ctx.data;
            let use_case = CreateItem::new(repo);

            match use_case.execute(payload.into()).await {
                Ok(item) => Response::from_json(&SingleResponse::new(
                    ItemResponse::from(item),
                    "Item created",
                )),
                Err(e) => handle_error(e),
            }
        }
        Err(validation_result) => {
            let errors: Vec<String> = validation_result
                .issues
                .iter()
                .map(|issue: &ValidationIssue| issue.to_string())
                .collect();
            handle_validation_error(errors)
        }
    }
}

pub async fn update_item_handler(
    mut req: Request,
    ctx: RouteContext<DynItemRepository>,
) -> worker::Result<Response> {
    let id: i64 = match ctx.param("id") {
        Some(id_str) => match id_str.parse() {
            Ok(id) if id > 0 => id,
            Ok(_) => return handle_error(AppError::BadRequest("Id must be positive".into())),
            Err(_) => return handle_error(AppError::BadRequest("Invalid id parameter".into())),
        },
        None => return handle_error(AppError::BadRequest("Missing id parameter".into())),
    };

    let json_value: Value = match req.json().await {
        Ok(v) => v,
        Err(e) => return handle_error(AppError::BadRequest(e.to_string())),
    };

    match UpdateItemRequest::validate_and_parse(&json_value) {
        Ok(payload) => {
            let repo = ctx.data;
            let use_case = UpdateItem::new(repo);

            match use_case.execute(id, payload.into()).await {
                Ok(item) => Response::from_json(&SingleResponse::new(
                    ItemResponse::from(item),
                    "Item updated",
                )),
                Err(e) => handle_error(e),
            }
        }
        Err(validation_result) => {
            let errors: Vec<String> = validation_result
                .issues
                .iter()
                .map(|issue: &ValidationIssue| issue.to_string())
                .collect();
            handle_validation_error(errors)
        }
    }
}

pub async fn delete_item_handler(
    _req: Request,
    ctx: RouteContext<DynItemRepository>,
) -> worker::Result<Response> {
    let id: i64 = match ctx.param("id") {
        Some(id_str) => match id_str.parse() {
            Ok(id) if id > 0 => id,
            Ok(_) => return handle_error(AppError::BadRequest("Id must be positive".into())),
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
