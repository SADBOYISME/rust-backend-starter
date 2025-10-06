# Rust Backend Starter Template 🦀

A production-ready, scalable REST API backend built with Rust, featuring JWT authentication, PostgreSQL database, and comprehensive CRUD operations.

## ✨ Features

- **🔐 JWT Authentication** - Secure signup/login with bcrypt password hashing
- **🗄️ PostgreSQL Database** - SQLx for type-safe database queries
- **⚡ High Performance** - Built with Axum web framework
- **🔄 CRUD Operations** - Complete RESTful API endpoints
- **✅ Input Validation** - Request validation with validator
- **🐳 Docker Support** - Ready-to-use Docker and Docker Compose setup
- **📝 Migrations** - Database migrations with SQLx
- **🛡️ Error Handling** - Comprehensive error handling and logging
- **🔧 Easy Configuration** - Environment-based configuration

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+ ([Install Rust](https://rustup.rs/))
- PostgreSQL 14+ ([Install PostgreSQL](https://www.postgresql.org/download/))
- Docker & Docker Compose (optional)

### Option 1: Local Setup

1. **Clone and setup**
   ```bash
   cd rust-backend-starter
   cp .env.example .env
   ```

2. **Configure environment**
   Edit `.env` with your database credentials:
   ```env
   DATABASE_URL=postgresql://username:password@localhost:5432/rust_starter_db
   JWT_SECRET=your-super-secret-jwt-key-change-this
   ```

3. **Create database**
   ```bash
   # Using psql
   createdb rust_starter_db
   ```

4. **Run migrations and start server**
   ```bash
   cargo run
   ```

The server will start at `http://127.0.0.1:8000`

### Option 2: Docker Setup

```bash
# Start everything with Docker Compose
docker-compose up -d

# View logs
docker-compose logs -f app
```

## 📚 API Documentation

### Base URL
```
http://localhost:8000
```

### Public Endpoints

#### Health Check
```http
GET /health
```

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T12:00:00Z"
}
```

#### Signup
```http
POST /auth/signup
Content-Type: application/json

{
  "email": "user@example.com",
  "username": "johndoe",
  "password": "securepassword123"
}
```

**Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIs...",
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "johndoe",
    "created_at": "2024-01-01T12:00:00Z"
  }
}
```

#### Login
```http
POST /auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

### Protected Endpoints
*Include JWT token in Authorization header:* `Authorization: Bearer <token>`

#### Get Current User
```http
GET /auth/me
Authorization: Bearer <token>
```

#### Create Item
```http
POST /items
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "My First Item",
  "description": "This is a test item"
}
```

#### Get All Items
```http
GET /items
Authorization: Bearer <token>
```

#### Get Single Item
```http
GET /items/:id
Authorization: Bearer <token>
```

#### Update Item
```http
PUT /items/:id
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "Updated Title",
  "description": "Updated description",
  "status": "completed"
}
```

#### Delete Item
```http
DELETE /items/:id
Authorization: Bearer <token>
```

## 🏗️ Project Structure

```
rust-backend-starter/
├── src/
│   ├── main.rs              # Application entry point
│   ├── config.rs            # Configuration management
│   ├── db.rs                # Database connection & migrations
│   ├── error.rs             # Error types and handling
│   ├── routes.rs            # Route definitions
│   ├── handlers/            # Request handlers
│   │   ├── auth.rs          # Authentication handlers
│   │   ├── items.rs         # CRUD handlers
│   │   └── health.rs        # Health check
│   ├── middleware/          # Custom middleware
│   │   └── auth.rs          # JWT authentication middleware
│   ├── models/              # Data models
│   │   ├── user.rs          # User model & DTOs
│   │   └── item.rs          # Item model & DTOs
│   └── utils/               # Utility functions
│       └── auth.rs          # JWT & password hashing
├── migrations/              # SQL migrations
│   ├── 20240101000001_create_users_table.sql
│   └── 20240101000002_create_items_table.sql
├── Cargo.toml              # Dependencies
├── Dockerfile              # Docker image definition
├── docker-compose.yml      # Docker Compose setup
├── .env.example            # Example environment variables
└── README.md
```

## 🔧 Configuration

Environment variables (`.env`):

| Variable | Description | Default |
|----------|-------------|---------|
| `HOST` | Server host | `127.0.0.1` |
| `PORT` | Server port | `8000` |
| `DATABASE_URL` | PostgreSQL connection string | Required |
| `JWT_SECRET` | Secret key for JWT signing | Required |
| `JWT_EXPIRATION` | Token expiration in seconds | `86400` (24h) |
| `RUST_LOG` | Logging level | `debug` |
| `APP_ENV` | Environment (development/production) | `development` |

## 🛠️ Development

### Run Tests
```bash
cargo test
```

### Format Code
```bash
cargo fmt
```

### Lint Code
```bash
cargo clippy
```

### Database Migrations

Create a new migration:
```bash
sqlx migrate add <migration_name>
```

Run migrations:
```bash
sqlx migrate run
```

Revert last migration:
```bash
sqlx migrate revert
```

## 📦 Building for Production

```bash
# Build optimized binary
cargo build --release

# Binary will be at target/release/rust-backend-starter
./target/release/rust-backend-starter
```

## 🐳 Docker Commands

```bash
# Build image
docker build -t rust-backend-starter .

# Run container
docker run -p 8000:8000 --env-file .env rust-backend-starter

# Using Docker Compose
docker-compose up -d        # Start services
docker-compose down         # Stop services
docker-compose logs -f      # View logs
docker-compose restart app  # Restart app
```

## 🔒 Security Best Practices

- ✅ Passwords are hashed with bcrypt
- ✅ JWT tokens for stateless authentication
- ✅ SQL injection protection via SQLx
- ✅ Input validation on all endpoints
- ✅ CORS configured (customize in production)
- ⚠️ Change `JWT_SECRET` in production
- ⚠️ Use HTTPS in production
- ⚠️ Set strong database passwords

## 🚀 Scaling Tips

1. **Database Connection Pool** - Adjust `max_connections` in `src/db.rs`
2. **Horizontal Scaling** - Stateless design allows multiple instances
3. **Caching** - Add Redis for session/data caching
4. **Database Indexing** - Migrations include essential indexes
5. **Load Balancing** - Use nginx or cloud load balancers
6. **Monitoring** - Add Prometheus/Grafana for metrics

## 📝 Adding New Features

### Add a new CRUD resource:

1. Create model in `src/models/your_resource.rs`
2. Create migration in `migrations/`
3. Add handlers in `src/handlers/your_resource.rs`
4. Register routes in `src/routes.rs`

### Example: Adding "posts" resource

```rust
// migrations/xxx_create_posts.sql
CREATE TABLE posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    content TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License.

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [SQLx](https://github.com/launchbadge/sqlx) - Database toolkit
- [Tokio](https://tokio.rs/) - Async runtime

---

**Built with ❤️ using Rust**

For questions or issues, please open an issue on GitHub.
