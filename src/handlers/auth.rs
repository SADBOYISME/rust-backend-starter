use axum::{extract::State, http::StatusCode, Json};
use validator::Validate;

use crate::{
    config::Config,
    error::{AppError, AppResult},
    models::{AuthResponse, CreateUser, LoginUser, User, UserResponse},
    utils::auth::{create_token, hash_password, verify_password},
    AppState,
};

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> AppResult<(StatusCode, Json<AuthResponse>)> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Check if user already exists
    let existing_user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1 OR username = $2"
    )
    .bind(&payload.email)
    .bind(&payload.username)
    .fetch_optional(&state.db)
    .await?;

    if existing_user.is_some() {
        return Err(AppError::BadRequest("User with this email or username already exists".to_string()));
    }

    // Hash password
    let password_hash = hash_password(&payload.password)
        .map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))?;

    // Create user
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, username, password_hash) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.email)
    .bind(&payload.username)
    .bind(&password_hash)
    .fetch_one(&state.db)
    .await?;

    // Generate JWT token
    let token = create_token(user.id, user.email.clone(), &state.config)
        .map_err(|e| AppError::Internal(format!("Failed to create token: {}", e)))?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            user: user.into(),
        }),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginUser>,
) -> AppResult<Json<AuthResponse>> {
    // Validate input
    payload.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Find user by email
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Authentication("Invalid email or password".to_string()))?;

    // Verify password
    let is_valid = verify_password(&payload.password, &user.password_hash)
        .map_err(|e| AppError::Internal(format!("Failed to verify password: {}", e)))?;

    if !is_valid {
        return Err(AppError::Authentication("Invalid email or password".to_string()));
    }

    // Generate JWT token
    let token = create_token(user.id, user.email.clone(), &state.config)
        .map_err(|e| AppError::Internal(format!("Failed to create token: {}", e)))?;

    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}

pub async fn get_me(
    State(state): State<AppState>,
    user_id: axum::Extension<String>,
) -> AppResult<Json<UserResponse>> {
    let user_uuid = user_id.0.parse()
        .map_err(|_| AppError::Internal("Invalid user ID format".to_string()))?;

    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(user_uuid)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(user.into()))
}
