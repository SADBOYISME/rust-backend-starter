use crate::{config::Config, error::AppError, utils::auth::verify_token};
use axum::{extract::Request, http::header, middleware::Next, response::Response};

pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid authorization header format".to_string()))?;

    let config = req
        .extensions()
        .get::<Config>()
        .ok_or_else(|| AppError::Internal("Config not found in request extensions".to_string()))?;

    let claims = verify_token(token, config)
        .map_err(|e| AppError::Authentication(format!("Invalid token: {}", e)))?;

    // Add user ID to request extensions for use in handlers
    req.extensions_mut().insert(claims.sub.clone());

    Ok(next.run(req).await)
}
