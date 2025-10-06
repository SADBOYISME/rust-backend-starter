# Rust Backend Starter - AI Coding Instructions

## Architecture Overview

This is a production-ready Rust REST API using **Axum** web framework with JWT authentication, PostgreSQL, and a layered architecture pattern:

- **State Management**: `AppState` struct contains shared `PgPool` and `Config` - passed via `State<AppState>` extractors
- **Route Organization**: Public routes (health, auth) vs protected routes (items) with middleware layering in `src/routes.rs`
- **Error Handling**: Centralized `AppError` enum in `src/error.rs` that implements `IntoResponse` for HTTP responses
- **Authentication Flow**: JWT tokens in Authorization headers → middleware extracts user ID → handlers access via request extensions

## Key Patterns & Conventions

### Handler Structure

All handlers follow this pattern:

```rust
pub async fn handler_name(
    State(state): State<AppState>,
    Json(payload): Json<RequestModel>,
) -> AppResult<Json<ResponseModel>> {
    // Input validation with validator crate
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    // Database operations with sqlx
    let result = sqlx::query_as::<_, Model>("SQL").fetch_one(&state.db).await?;

    Ok(Json(result.into()))
}
```

### Database Conventions

- Use `sqlx::query_as` with typed structs, not raw queries
- Models implement `FromRow` for database mapping and `Validate` for input validation
- UUID primary keys generated with `gen_random_uuid()`
- Separate request/response DTOs from database models (e.g., `CreateUser` → `User` → `UserResponse`)

### Authentication Middleware

The `auth_middleware` extracts JWT tokens, validates them, and injects user ID into request extensions:

```rust
req.extensions_mut().insert(claims.sub.clone());
```

Protected handlers access the user ID via request extensions.

## Development Workflow

### Essential Commands

```bash
# Local development (requires PostgreSQL running)
cargo run

# Development with auto-reload
cargo watch -x run  # or make dev

# Database migrations (auto-run on startup)
sqlx migrate run

# Docker development stack
docker-compose up -d
```

### Configuration

Environment variables in `.env` (copy from `.env.example`):

- `DATABASE_URL`: PostgreSQL connection string
- `JWT_SECRET`: Secret key for JWT signing
- `JWT_EXPIRATION`: Token expiration in seconds

### Project Structure

- `src/handlers/`: Route handlers organized by domain (auth, items, health)
- `src/middleware/`: Authentication and other middleware
- `src/models/`: Database models and DTOs with validation
- `src/utils/`: Utility functions (JWT, password hashing)
- `migrations/`: SQLx database migrations

## Critical Integration Points

### Database Layer (`src/db.rs`)

- Connection pooling via `PgPoolOptions` with max 5 connections
- Auto-migration on startup using `sqlx::migrate!`
- All database errors automatically convert to `AppError::Database`

### Route Architecture (`src/routes.rs`)

- CORS enabled for all origins (configure for production)
- Tracing middleware for request logging
- Config injected via `Extension` layer, state via `with_state`
- Protected routes use `auth_middleware` layer

### Error Handling Pattern

Always use `AppError` enum variants:

- `AppError::Validation` for input validation failures
- `AppError::Authentication` for login failures
- `AppError::Unauthorized` for missing/invalid tokens
- `AppError::NotFound` for missing resources

### Testing Strategy

Use `reqwest` client in tests. See `API_EXAMPLES.md` for curl/PowerShell examples.
Authentication flow: signup → login (get token) → use Bearer token in Authorization header.

## Common Gotchas

- Password hashing uses bcrypt with `DEFAULT_COST` (currently 12 rounds)
- JWT tokens include user ID as `sub` claim and email
- All timestamps use UTC with `DateTime<Utc>`
- Request validation happens before database operations
- Use `#[serde(skip_serializing)]` to hide sensitive fields like `password_hash`
