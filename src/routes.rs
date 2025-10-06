use axum::{
    middleware,
    routing::{delete, get, post, put},
    Extension, Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::{
    config::Config,
    handlers,
    middleware::auth_middleware,
    AppState,
};

pub fn create_router(state: AppState, config: Config) -> Router {
    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/auth/signup", post(handlers::signup))
        .route("/auth/login", post(handlers::login));

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        .route("/auth/me", get(handlers::get_me))
        .route("/items", post(handlers::create_item))
        .route("/items", get(handlers::get_items))
        .route("/items/:id", get(handlers::get_item))
        .route("/items/:id", put(handlers::update_item))
        .route("/items/:id", delete(handlers::delete_item))
        .layer(middleware::from_fn(auth_middleware));

    // Combine routes
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(config))
        .with_state(state)
}
