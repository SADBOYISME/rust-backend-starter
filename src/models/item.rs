use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateItem {
    #[validate(length(min = 1, max = 255, message = "Title must be between 1 and 255 characters"))]
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateItem {
    #[validate(length(min = 1, max = 255, message = "Title must be between 1 and 255 characters"))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ItemResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Item> for ItemResponse {
    fn from(item: Item) -> Self {
        Self {
            id: item.id,
            user_id: item.user_id,
            title: item.title,
            description: item.description,
            status: item.status,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}
