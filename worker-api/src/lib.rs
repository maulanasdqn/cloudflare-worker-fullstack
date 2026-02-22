mod application;
mod domain;
mod errors;
mod infrastructure;
mod types;

use std::sync::Arc;

use worker::{event, Context, Env, Request, Response, Router};

use crate::domain::ItemRepository;
use crate::infrastructure::http::{
    create_item_handler, delete_item_handler, get_item_handler, list_items_handler,
    update_item_handler,
};
use crate::infrastructure::persistence::D1ItemRepository;

async fn serve_static(req: Request, env: Env) -> worker::Result<Response> {
    let path = req.path();
    let asset_path = if path == "/" { "/index.html" } else { &path };

    let content_type = if asset_path.ends_with(".html") {
        "text/html"
    } else if asset_path.ends_with(".js") {
        "application/javascript"
    } else if asset_path.ends_with(".wasm") {
        "application/wasm"
    } else if asset_path.ends_with(".css") {
        "text/css"
    } else {
        "application/octet-stream"
    };

    let is_binary = asset_path.ends_with(".wasm");

    match env.kv("__STATIC_CONTENT") {
        Ok(kv) => {
            if is_binary {
                match kv.get(asset_path).bytes().await? {
                    Some(bytes) => Response::from_bytes(bytes).map(|r| {
                        let headers = r.headers().clone();
                        let _ = headers.set("Content-Type", content_type);
                        r.with_headers(headers)
                    }),
                    None => Response::error("Not Found", 404),
                }
            } else {
                match kv.get(asset_path).text().await? {
                    Some(content) => Response::ok(content).map(|r| {
                        let headers = r.headers().clone();
                        let _ = headers.set("Content-Type", content_type);
                        r.with_headers(headers)
                    }),
                    None => Response::error("Not Found", 404),
                }
            }
        }
        Err(_) => Response::error("Static content not available", 500),
    }
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    console_error_panic_hook::set_once();

    let path = req.path();

    if path.starts_with("/api/") {
        let db = env.d1("DB")?;
        let repo: Arc<dyn ItemRepository> = Arc::new(D1ItemRepository::new(db));

        let router = Router::with_data(repo)
            .get_async("/api/items", list_items_handler)
            .post_async("/api/items", create_item_handler)
            .get_async("/api/items/:id", get_item_handler)
            .put_async("/api/items/:id", update_item_handler)
            .delete_async("/api/items/:id", delete_item_handler);

        router.run(req, env).await
    } else {
        serve_static(req, env).await
    }
}
