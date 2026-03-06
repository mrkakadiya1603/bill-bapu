use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

/// Standard API response wrapper
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ApiResponse<T: TS> {
    pub success: bool,
    pub data: Option<T>,
    pub errors: Option<Vec<ApiError>>,
    pub meta: Option<PaginationMeta>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub field: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PaginationMeta {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// Base fields shared by all entities
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct BaseEntity {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
