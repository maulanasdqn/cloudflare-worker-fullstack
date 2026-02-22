use worker::{Request, Response};

pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub max_age: u32,
    pub allow_credentials: bool,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "OPTIONS".to_string(),
            ],
            allowed_headers: vec![
                "Content-Type".to_string(),
                "Authorization".to_string(),
                "X-Requested-With".to_string(),
            ],
            max_age: 86400,
            allow_credentials: false,
        }
    }
}

pub struct Cors {
    config: CorsConfig,
}

impl Cors {
    pub fn new(config: CorsConfig) -> Self {
        Self { config }
    }

    pub fn with_defaults() -> Self {
        Self::new(CorsConfig::default())
    }

    pub fn is_preflight(&self, req: &Request) -> bool {
        req.method() == worker::Method::Options
    }

    fn get_origin(&self, req: &Request) -> Option<String> {
        req.headers().get("Origin").ok().flatten()
    }

    fn is_origin_allowed(&self, origin: &str) -> bool {
        self.config.allowed_origins.contains(&"*".to_string())
            || self.config.allowed_origins.contains(&origin.to_string())
    }

    pub fn preflight_response(&self, req: &Request) -> worker::Result<Response> {
        let origin = self.get_origin(req).unwrap_or_default();
        let allowed_origin = if self.is_origin_allowed(&origin) {
            if self.config.allowed_origins.contains(&"*".to_string()) {
                "*".to_string()
            } else {
                origin
            }
        } else {
            return Response::error("Origin not allowed", 403);
        };

        let response = Response::empty()?;
        let headers = response.headers().clone();

        let _ = headers.set("Access-Control-Allow-Origin", &allowed_origin);
        let _ = headers.set(
            "Access-Control-Allow-Methods",
            &self.config.allowed_methods.join(", "),
        );
        let _ = headers.set(
            "Access-Control-Allow-Headers",
            &self.config.allowed_headers.join(", "),
        );
        let _ = headers.set("Access-Control-Max-Age", &self.config.max_age.to_string());

        if self.config.allow_credentials {
            let _ = headers.set("Access-Control-Allow-Credentials", "true");
        }

        Ok(response.with_headers(headers))
    }

    pub fn apply_headers(&self, req: &Request, response: Response) -> Response {
        let origin = self.get_origin(req).unwrap_or_default();
        let allowed_origin = if self.is_origin_allowed(&origin) {
            if self.config.allowed_origins.contains(&"*".to_string()) {
                "*".to_string()
            } else {
                origin
            }
        } else {
            return response;
        };

        let headers = response.headers().clone();
        let _ = headers.set("Access-Control-Allow-Origin", &allowed_origin);

        if self.config.allow_credentials {
            let _ = headers.set("Access-Control-Allow-Credentials", "true");
        }

        response.with_headers(headers)
    }
}
