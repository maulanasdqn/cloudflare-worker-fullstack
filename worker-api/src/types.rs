use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SingleResponse<T> {
    pub data: T,
    pub message: String,
}

impl<T> SingleResponse<T> {
    pub fn new(data: T, message: impl Into<String>) -> Self {
        Self {
            data,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    Asc,
    Desc,
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::Desc
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ListQueryParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
}

impl ListQueryParams {
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn per_page(&self) -> u32 {
        self.per_page.unwrap_or(10).clamp(1, 100)
    }

    pub fn offset(&self) -> u32 {
        (self.page() - 1) * self.per_page()
    }

    pub fn search(&self) -> Option<&str> {
        self.search.as_deref().filter(|s| !s.is_empty())
    }

    pub fn sort_by(&self) -> &str {
        self.sort_by.as_deref().unwrap_or("created_at")
    }

    pub fn sort_order(&self) -> SortOrder {
        self.sort_order.unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

impl PaginationMeta {
    pub fn new(page: u32, per_page: u32, total: u64) -> Self {
        let total_pages = if total == 0 {
            1
        } else {
            ((total as f64) / (per_page as f64)).ceil() as u32
        };
        Self {
            page,
            per_page,
            total,
            total_pages,
            has_next: page < total_pages,
            has_prev: page > 1,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

impl<T> ListResponse<T> {
    pub fn new(data: Vec<T>, meta: PaginationMeta) -> Self {
        Self { data, meta }
    }
}
