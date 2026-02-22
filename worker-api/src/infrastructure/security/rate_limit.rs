use worker::{kv::KvStore, Request, Response};

use crate::errors::AppError;

pub struct RateLimitConfig {
    pub requests_per_window: u32,
    pub window_seconds: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_window: 100,
            window_seconds: 60,
        }
    }
}

pub struct RateLimiter {
    kv: KvStore,
    config: RateLimitConfig,
}

impl RateLimiter {
    pub fn new(kv: KvStore, config: RateLimitConfig) -> Self {
        Self { kv, config }
    }

    pub fn with_defaults(kv: KvStore) -> Self {
        Self::new(kv, RateLimitConfig::default())
    }

    fn get_client_ip(&self, req: &Request) -> String {
        req.headers()
            .get("CF-Connecting-IP")
            .ok()
            .flatten()
            .or_else(|| req.headers().get("X-Forwarded-For").ok().flatten())
            .unwrap_or_else(|| "unknown".to_string())
    }

    pub async fn check(&self, req: &Request) -> Result<RateLimitResult, AppError> {
        let ip = self.get_client_ip(req);
        let key = format!("ratelimit:{}", ip);

        let current: u32 = self
            .kv
            .get(&key)
            .text()
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        if current >= self.config.requests_per_window {
            return Ok(RateLimitResult::Exceeded {
                limit: self.config.requests_per_window,
                remaining: 0,
                reset_seconds: self.config.window_seconds,
            });
        }

        let new_count = current + 1;
        self.kv
            .put(&key, new_count.to_string())
            .map_err(|e| AppError::InternalError(e.to_string()))?
            .expiration_ttl(self.config.window_seconds)
            .execute()
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(RateLimitResult::Allowed {
            limit: self.config.requests_per_window,
            remaining: self.config.requests_per_window - new_count,
            reset_seconds: self.config.window_seconds,
        })
    }
}

pub enum RateLimitResult {
    Allowed {
        limit: u32,
        remaining: u32,
        reset_seconds: u64,
    },
    Exceeded {
        limit: u32,
        remaining: u32,
        reset_seconds: u64,
    },
}

impl RateLimitResult {
    pub fn is_exceeded(&self) -> bool {
        matches!(self, RateLimitResult::Exceeded { .. })
    }

    pub fn apply_headers(&self, response: Response) -> Response {
        let (limit, remaining, reset) = match self {
            RateLimitResult::Allowed {
                limit,
                remaining,
                reset_seconds,
            } => (*limit, *remaining, *reset_seconds),
            RateLimitResult::Exceeded {
                limit,
                remaining,
                reset_seconds,
            } => (*limit, *remaining, *reset_seconds),
        };

        let headers = response.headers().clone();
        let _ = headers.set("X-RateLimit-Limit", &limit.to_string());
        let _ = headers.set("X-RateLimit-Remaining", &remaining.to_string());
        let _ = headers.set("X-RateLimit-Reset", &reset.to_string());

        response.with_headers(headers)
    }

    pub fn to_error_response(&self) -> worker::Result<Response> {
        let resp = Response::error("Too Many Requests", 429)?;
        Ok(self.apply_headers(resp))
    }
}
