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
use crate::infrastructure::security::{
    Cors, RateLimitConfig, RateLimiter, RequestValidator, SecurityHeaders,
};

fn add_security_headers(response: Response, for_api: bool) -> Response {
    let headers = response.headers().clone();

    let _ = headers.set("X-Content-Type-Options", "nosniff");
    let _ = headers.set("X-Frame-Options", "DENY");
    let _ = headers.set("X-XSS-Protection", "1; mode=block");
    let _ = headers.set("Referrer-Policy", "strict-origin-when-cross-origin");
    let _ = headers.set(
        "Strict-Transport-Security",
        "max-age=31536000; includeSubDomains",
    );

    if !for_api {
        let _ = headers.set("Content-Security-Policy", "default-src 'self' https://cdn.tailwindcss.com 'unsafe-inline' 'unsafe-eval'; script-src 'self' https://cdn.tailwindcss.com 'unsafe-inline' 'unsafe-eval'; style-src 'self' https://cdn.tailwindcss.com 'unsafe-inline'");
    }

    response.with_headers(headers)
}

fn add_cors_headers(response: Response, origin: Option<String>) -> Response {
    let headers = response.headers().clone();
    let allowed_origin = origin.unwrap_or_else(|| "*".to_string());

    let _ = headers.set("Access-Control-Allow-Origin", &allowed_origin);
    let _ = headers.set(
        "Access-Control-Allow-Methods",
        "GET, POST, PUT, DELETE, OPTIONS",
    );
    let _ = headers.set(
        "Access-Control-Allow-Headers",
        "Content-Type, Authorization, X-Requested-With",
    );
    let _ = headers.set("Access-Control-Max-Age", "86400");

    response.with_headers(headers)
}

fn add_rate_limit_headers(response: Response, limit: u32, remaining: u32, reset: u64) -> Response {
    let headers = response.headers().clone();

    let _ = headers.set("X-RateLimit-Limit", &limit.to_string());
    let _ = headers.set("X-RateLimit-Remaining", &remaining.to_string());
    let _ = headers.set("X-RateLimit-Reset", &reset.to_string());

    response.with_headers(headers)
}

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

    let response = match env.kv("__STATIC_CONTENT") {
        Ok(kv) => {
            if is_binary {
                match kv.get(asset_path).bytes().await? {
                    Some(bytes) => {
                        let resp = Response::from_bytes(bytes)?;
                        let headers = resp.headers().clone();
                        let _ = headers.set("Content-Type", content_type);
                        resp.with_headers(headers)
                    }
                    None => Response::error("Not Found", 404)?,
                }
            } else {
                match kv.get(asset_path).text().await? {
                    Some(content) => {
                        let resp = Response::ok(content)?;
                        let headers = resp.headers().clone();
                        let _ = headers.set("Content-Type", content_type);
                        resp.with_headers(headers)
                    }
                    None => Response::error("Not Found", 404)?,
                }
            }
        }
        Err(_) => Response::error("Static content not available", 500)?,
    };

    Ok(add_security_headers(response, false))
}

async fn handle_api_request(req: Request, env: Env) -> worker::Result<Response> {
    let origin = req.headers().get("Origin").ok().flatten();
    let method = req.method();

    if method == worker::Method::Options {
        let response = Response::empty()?;
        let response = add_cors_headers(response, origin);
        return Ok(response);
    }

    let validator = RequestValidator::default();
    if let Err(e) = validator.validate(&req) {
        let response: Response = e.into();
        let response = add_security_headers(response, true);
        let response = add_cors_headers(response, origin);
        return Ok(response);
    }

    let (rate_limit, rate_remaining, rate_reset) = if let Ok(kv) = env.kv("RATE_LIMIT") {
        let config = RateLimitConfig {
            requests_per_window: 100,
            window_seconds: 60,
        };
        let limiter = RateLimiter::new(kv, config);

        match limiter.check(&req).await {
            Ok(result) => {
                if result.is_exceeded() {
                    let response = Response::error("Too Many Requests", 429)?;
                    let response = add_security_headers(response, true);
                    let response = add_cors_headers(response, origin);
                    let response = add_rate_limit_headers(response, 100, 0, 60);
                    return Ok(response);
                }
                match result {
                    crate::infrastructure::security::RateLimitResult::Allowed {
                        limit,
                        remaining,
                        reset_seconds,
                    } => (limit, remaining, reset_seconds),
                    _ => (100, 0, 60),
                }
            }
            Err(_) => (100, 100, 60),
        }
    } else {
        (100, 100, 60)
    };

    let db = env.d1("DB")?;
    let repo: Arc<dyn ItemRepository> = Arc::new(D1ItemRepository::new(db));

    let router = Router::with_data(repo)
        .get_async("/api/items", list_items_handler)
        .post_async("/api/items", create_item_handler)
        .get_async("/api/items/:id", get_item_handler)
        .put_async("/api/items/:id", update_item_handler)
        .delete_async("/api/items/:id", delete_item_handler);

    let response = router.run(req, env).await?;
    let response = add_security_headers(response, true);
    let response = add_cors_headers(response, origin);
    let response = add_rate_limit_headers(response, rate_limit, rate_remaining, rate_reset);

    Ok(response)
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    console_error_panic_hook::set_once();

    let path = req.path();

    if path.starts_with("/api/") {
        handle_api_request(req, env).await
    } else {
        serve_static(req, env).await
    }
}
