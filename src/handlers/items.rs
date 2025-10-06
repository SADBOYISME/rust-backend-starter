use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::{AppError, AppResult},
    models::{CreateItem, Item, ItemResponse, UpdateItem},
    AppState,
};

pub async fn create_item(
    State(state): State<AppState>,
    user_id: axum::Extension<String>,
    Json(payload): Json<CreateItem>,
) -> AppResult<(StatusCode, Json<ItemResponse>)> {
    // Validate input
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let user_uuid: Uuid = user_id
        .0
        .parse()
        .map_err(|_| AppError::Internal("Invalid user ID format".to_string()))?;

    let item = sqlx::query_as::<_, Item>(
        "INSERT INTO items (user_id, title, description) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(user_uuid)
    .bind(&payload.title)
    .bind(&payload.description)
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(item.into())))
}

pub async fn get_items(
    State(state): State<AppState>,
    user_id: axum::Extension<String>,
) -> AppResult<Json<Vec<ItemResponse>>> {
    let user_uuid: Uuid = user_id
        .0
        .parse()
        .map_err(|_| AppError::Internal("Invalid user ID format".to_string()))?;

    let items = sqlx::query_as::<_, Item>(
        "SELECT * FROM items WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(user_uuid)
    .fetch_all(&state.db)
    .await?;

    let responses: Vec<ItemResponse> = items.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn get_item(
    State(state): State<AppState>,
    user_id: axum::Extension<String>,
    Path(item_id): Path<Uuid>,
) -> AppResult<Json<ItemResponse>> {
    let user_uuid: Uuid = user_id
        .0
        .parse()
        .map_err(|_| AppError::Internal("Invalid user ID format".to_string()))?;

    let item = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE id = $1 AND user_id = $2")
        .bind(item_id)
        .bind(user_uuid)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Item not found".to_string()))?;

    Ok(Json(item.into()))
}

pub async fn update_item(
    State(state): State<AppState>,
    user_id: axum::Extension<String>,
    Path(item_id): Path<Uuid>,
    Json(payload): Json<UpdateItem>,
) -> AppResult<Json<ItemResponse>> {
    // Validate input
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let user_uuid: Uuid = user_id
        .0
        .parse()
        .map_err(|_| AppError::Internal("Invalid user ID format".to_string()))?;

    // Check if item exists and belongs to user
    let _existing_item =
        sqlx::query_as::<_, Item>("SELECT * FROM items WHERE id = $1 AND user_id = $2")
            .bind(item_id)
            .bind(user_uuid)
            .fetch_optional(&state.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Item not found".to_string()))?;

    // Update item
    let item = sqlx::query_as::<_, Item>(
        r#"
        UPDATE items 
        SET title = COALESCE($1, title),
            description = COALESCE($2, description),
            status = COALESCE($3, status)
        WHERE id = $4 AND user_id = $5
        RETURNING *
        "#,
    )
    .bind(payload.title)
    .bind(payload.description)
    .bind(payload.status)
    .bind(item_id)
    .bind(user_uuid)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(item.into()))
}

pub async fn delete_item(
    State(state): State<AppState>,
    user_id: axum::Extension<String>,
    Path(item_id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let user_uuid: Uuid = user_id
        .0
        .parse()
        .map_err(|_| AppError::Internal("Invalid user ID format".to_string()))?;

    let result = sqlx::query("DELETE FROM items WHERE id = $1 AND user_id = $2")
        .bind(item_id)
        .bind(user_uuid)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Item not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
