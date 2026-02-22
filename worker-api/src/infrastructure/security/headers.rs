use worker::Response;

pub struct SecurityHeaders {
    pub content_type_options: bool,
    pub frame_options: FrameOptions,
    pub xss_protection: bool,
    pub referrer_policy: String,
    pub content_security_policy: Option<String>,
    pub strict_transport_security: Option<HstsConfig>,
    pub permissions_policy: Option<String>,
}

pub enum FrameOptions {
    Deny,
    SameOrigin,
    None,
}

pub struct HstsConfig {
    pub max_age: u64,
    pub include_subdomains: bool,
    pub preload: bool,
}

impl Default for HstsConfig {
    fn default() -> Self {
        Self {
            max_age: 31536000,
            include_subdomains: true,
            preload: false,
        }
    }
}

impl Default for SecurityHeaders {
    fn default() -> Self {
        Self {
            content_type_options: true,
            frame_options: FrameOptions::Deny,
            xss_protection: true,
            referrer_policy: "strict-origin-when-cross-origin".to_string(),
            content_security_policy: Some("default-src 'self'".to_string()),
            strict_transport_security: Some(HstsConfig::default()),
            permissions_policy: Some("geolocation=(), microphone=(), camera=()".to_string()),
        }
    }
}

impl SecurityHeaders {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn api_defaults() -> Self {
        Self {
            content_type_options: true,
            frame_options: FrameOptions::Deny,
            xss_protection: true,
            referrer_policy: "strict-origin-when-cross-origin".to_string(),
            content_security_policy: None,
            strict_transport_security: Some(HstsConfig::default()),
            permissions_policy: None,
        }
    }

    pub fn apply(&self, response: Response) -> Response {
        let headers = response.headers().clone();

        if self.content_type_options {
            let _ = headers.set("X-Content-Type-Options", "nosniff");
        }

        match &self.frame_options {
            FrameOptions::Deny => {
                let _ = headers.set("X-Frame-Options", "DENY");
            }
            FrameOptions::SameOrigin => {
                let _ = headers.set("X-Frame-Options", "SAMEORIGIN");
            }
            FrameOptions::None => {}
        }

        if self.xss_protection {
            let _ = headers.set("X-XSS-Protection", "1; mode=block");
        }

        let _ = headers.set("Referrer-Policy", &self.referrer_policy);

        if let Some(csp) = &self.content_security_policy {
            let _ = headers.set("Content-Security-Policy", csp);
        }

        if let Some(hsts) = &self.strict_transport_security {
            let mut value = format!("max-age={}", hsts.max_age);
            if hsts.include_subdomains {
                value.push_str("; includeSubDomains");
            }
            if hsts.preload {
                value.push_str("; preload");
            }
            let _ = headers.set("Strict-Transport-Security", &value);
        }

        if let Some(pp) = &self.permissions_policy {
            let _ = headers.set("Permissions-Policy", pp);
        }

        response.with_headers(headers)
    }
}
