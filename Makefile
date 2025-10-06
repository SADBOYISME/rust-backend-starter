.PHONY: help build run test clean docker-build docker-run migrate

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build the project
	cargo build --release

run: ## Run the application
	cargo run

dev: ## Run in development mode with auto-reload
	cargo watch -x run

test: ## Run tests
	cargo test

fmt: ## Format code
	cargo fmt

lint: ## Run clippy linter
	cargo clippy -- -D warnings

clean: ## Clean build artifacts
	cargo clean

migrate: ## Run database migrations
	sqlx migrate run

migrate-revert: ## Revert last migration
	sqlx migrate revert

docker-build: ## Build Docker image
	docker build -t rust-backend-starter .

docker-run: ## Run Docker container
	docker run -p 8000:8000 --env-file .env rust-backend-starter

docker-compose-up: ## Start services with Docker Compose
	docker-compose up -d

docker-compose-down: ## Stop services
	docker-compose down

docker-logs: ## View Docker logs
	docker-compose logs -f app

db-setup: ## Setup local database
	createdb rust_starter_db || true
	sqlx migrate run

check: fmt lint test ## Run all checks (format, lint, test)
