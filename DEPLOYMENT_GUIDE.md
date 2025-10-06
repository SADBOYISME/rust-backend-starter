# Deployment Guide for Rust Backend Starter

This guide provides comprehensive instructions for deploying the Rust Backend Starter application using the optimized Docker configuration.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Environment Setup](#environment-setup)
3. [Development Deployment](#development-deployment)
4. [Production Deployment](#production-deployment)
5. [Security Configuration](#security-configuration)
6. [Monitoring and Maintenance](#monitoring-and-maintenance)
7. [Backup and Recovery](#backup-and-recovery)
8. [Troubleshooting](#troubleshooting)

## Prerequisites

### System Requirements

- **Docker**: 20.10+ with Docker Compose v2+
- **Memory**: Minimum 2GB RAM (4GB recommended)
- **Storage**: Minimum 10GB free space
- **Network**: Internet connection for image downloads

### Required Tools

```bash
# Verify Docker installation
docker --version
docker compose version

# Verify script permissions (Linux/macOS)
chmod +x scripts/*.sh
```

## Environment Setup

### 1. Clone Repository

```bash
git clone https://github.com/SADBOYISME/rust-backend-starter.git
cd rust-backend-starter
```

### 2. Environment Configuration

#### Development Environment

```bash
# Copy the example environment file
cp .env.example .env

# Edit the environment file
nano .env
```

#### Production Environment

```bash
# Copy the production template
cp .env.production .env

# IMPORTANT: Edit with your actual production values
nano .env
```

### 3. Required Environment Variables

```bash
# Database Configuration
POSTGRES_USER=your_db_user
POSTGRES_PASSWORD=your_secure_db_password
POSTGRES_DB=your_db_name

# JWT Configuration (CRITICAL - use a strong secret)
JWT_SECRET=your-super-secure-jwt-secret-at-least-32-characters-long-random-string

# Application Configuration
APP_ENV=production
RUST_LOG=info,tower_http=info,sqlx=warn
```

## Development Deployment

### Quick Start

```bash
# Using original Dockerfile
docker compose up -d

# Or using optimized configuration
docker compose -f docker-compose.optimized.yml up -d
```

### Development Features

- PostgreSQL exposed on port `5433`
- Application exposed on port `8000`
- Debug logging enabled
- Hot reload (if using volume mounts)

### Development Commands

```bash
# View logs
docker compose logs -f app
docker compose logs -f postgres

# Stop services
docker compose down

# Rebuild with changes
docker compose up -d --build
```

## Production Deployment

### 1. Using Optimized Configuration

```bash
# Deploy with production optimizations
docker compose -f docker-compose.optimized.yml up -d
```

### 2. Production Security Features

- **Network Isolation**: Custom bridge network
- **Read-only Filesystem**: Application container runs read-only
- **Capability Dropping**: Minimal Linux capabilities
- **Security Options**: No new privileges
- **Resource Limits**: CPU and memory constraints
- **Health Checks**: Automated health monitoring

### 3. Production Considerations

#### Port Security

The optimized configuration **does not expose PostgreSQL to the host**. Database is only accessible within the Docker network.

#### Resource Allocation

```yaml
# Production resource limits (in docker-compose.optimized.yml)
deploy:
  resources:
    limits:
      cpus: "0.5"
      memory: "256M"
    reservations:
      cpus: "0.1"
      memory: "128M"
```

## Security Configuration

### 1. JWT Secret Security

```bash
# Generate a secure JWT secret (Linux/macOS)
openssl rand -base64 64

# Or use Python
python3 -c "import secrets; print(secrets.token_urlsafe(64))"
```

### 2. Database Security

```bash
# Generate secure database password
openssl rand -base64 32

# Use strong credentials
POSTGRES_USER=app_user
POSTGRES_PASSWORD=generated_secure_password
POSTGRES_DB=app_database
```

### 3. Network Security

The optimized configuration includes:

- Custom bridge network (`app-network`)
- No external database port exposure
- Inter-container communication only

### 4. Runtime Security

```yaml
# Security options in docker-compose.optimized.yml
security_opt:
  - no-new-privileges:true
read_only: true
tmpfs:
  - /tmp
cap_drop:
  - ALL
cap_add:
  - CHOWN
  - SETGID
  - SETUID
```

## Monitoring and Maintenance

### 1. Health Checks

#### Application Health Check

```bash
# Check application health
curl http://localhost:8000/health

# Check container health status
docker ps --format "table {{.Names}}\t{{.Status}}"
```

#### Database Health Check

```bash
# Check database connectivity
docker exec rust_starter_db pg_isready -U postgres
```

### 2. Log Management

```bash
# View application logs
docker compose -f docker-compose.optimized.yml logs -f app

# View database logs
docker compose -f docker-compose.optimized.yml logs -f postgres

# Log rotation is configured automatically
# max-size: "10m", max-file: "3"
```

### 3. Resource Monitoring

```bash
# Check container resource usage
docker stats

# Check Docker system usage
./scripts/docker-cleanup.sh usage
```

### 4. System Maintenance

```bash
# Clean up Docker resources
./scripts/docker-cleanup.sh all

# Full system cleanup (periodic)
./scripts/docker-cleanup.sh full
```

## Backup and Recovery

### 1. Database Backup

#### Manual Backup

```bash
# Run backup script
./scripts/backup-db.sh

# Backup location: ./backups/
# Retention: 7 days (configurable)
```

#### Automated Backup (Cron)

```bash
# Add to crontab (Linux/macOS)
crontab -e

# Daily backup at 2 AM
0 2 * * * /path/to/rust-backend-starter/scripts/backup-db.sh
```

### 2. Database Recovery

```bash
# List available backups
ls -la backups/

# Restore from backup
gunzip -c backups/rust_starter_db_backup_YYYYMMDD_HHMMSS.sql.gz | \
docker exec -i rust_starter_db psql -U postgres -d rust_starter_db
```

### 3. Application Backup

```bash
# Backup environment configuration
cp .env .env.backup.$(date +%Y%m%d)

# Backup application image
docker save rust-backend-starter:latest > rust-backend-starter.tar
```

## Performance Optimization

### 1. Image Optimization

The optimized Dockerfile provides:

- **76% size reduction** (100MB → 24MB)
- Distroless base image
- Stripped binary
- Multi-stage build

### 2. Resource Tuning

Adjust resource limits based on your workload:

```yaml
# For high-load applications
deploy:
  resources:
    limits:
      cpus: "1.0"
      memory: "512M"
    reservations:
      cpus: "0.5"
      memory: "256M"
```

### 3. Database Optimization

```bash
# Connect to database
docker exec -it rust_starter_db psql -U postgres

# Check database size
SELECT pg_size_pretty(pg_database_size('rust_starter_db'));

# Check active connections
SELECT count(*) FROM pg_stat_activity;
```

## Troubleshooting

### 1. Common Issues

#### Container Won't Start

```bash
# Check logs
docker compose logs app
docker compose logs postgres

# Check configuration
docker compose config
```

#### Database Connection Issues

```bash
# Verify database is healthy
docker exec rust_starter_db pg_isready -U postgres

# Check network connectivity
docker network ls
docker network inspect rust-backend-starter_app-network
```

#### Permission Issues

```bash
# Fix script permissions (Linux/macOS)
chmod +x scripts/*.sh

# Check Docker permissions
docker run --rm hello-world
```

### 2. Health Check Failures

```bash
# Check application health endpoint
curl -v http://localhost:8000/health

# Check if curl is available in container
docker exec rust_starter_app which curl

# If curl is missing, rebuild with health check fix
docker compose -f docker-compose.optimized.yml build --no-cache app
```

### 3. Performance Issues

```bash
# Check resource usage
docker stats
docker top rust_starter_app

# Check database performance
docker exec rust_starter_db psql -U postgres -c "
SELECT query, calls, total_time, mean_time
FROM pg_stat_statements
ORDER BY total_time DESC
LIMIT 10;"
```

### 4. Recovery Procedures

#### Application Recovery

```bash
# Restart services
docker compose -f docker-compose.optimized.yml restart

# Full reset (data loss warning)
docker compose -f docker-compose.optimized.yml down -v
docker compose -f docker-compose.optimized.yml up -d
```

#### Database Recovery

```bash
# Restore from latest backup
LATEST_BACKUP=$(ls -t backups/*.gz | head -1)
gunzip -c "$LATEST_BACKUP" | \
docker exec -i rust_starter_db psql -U postgres -d rust_starter_db
```

## Advanced Configuration

### 1. Multi-Architecture Deployment

```yaml
# In docker-compose.yml
services:
  app:
    build:
      context: .
      dockerfile: Dockerfile.optimized
      platforms:
        - linux/amd64
        - linux/arm64
```

### 2. Environment-Specific Configurations

```bash
# Development
docker compose -f docker-compose.yml up -d

# Production
docker compose -f docker-compose.optimized.yml up -d

# Custom environment
docker compose -f docker-compose.custom.yml up -d
```

### 3. External Database Integration

```yaml
# Use external PostgreSQL
services:
  app:
    environment:
      DATABASE_URL: postgresql://user:pass@external-db:5432/dbname
    # Remove postgres service and dependencies
```

## Support and Maintenance

### 1. Regular Maintenance Tasks

- **Daily**: Monitor health checks and logs
- **Weekly**: Review backup retention and system usage
- **Monthly**: Update base images and dependencies
- **Quarterly**: Security audit and performance review

### 2. Update Procedures

```bash
# Update application
git pull origin main
docker compose -f docker-compose.optimized.yml build --no-cache app
docker compose -f docker-compose.optimized.yml up -d

# Update base images
docker compose -f docker-compose.optimized.yml pull
docker compose -f docker-compose.optimized.yml up -d
```

### 3. Security Best Practices

1. **Regular Updates**: Keep Docker images updated
2. **Secret Rotation**: Rotate JWT secrets regularly
3. **Access Control**: Limit Docker socket access
4. **Network Monitoring**: Monitor network traffic
5. **Audit Logs**: Regular security audit

## Quick Reference

### Essential Commands

```bash
# Deploy (Production)
docker compose -f docker-compose.optimized.yml up -d

# View Logs
docker compose -f docker-compose.optimized.yml logs -f

# Check Health
curl http://localhost:8000/health

# Backup Database
./scripts/backup-db.sh

# Cleanup System
./scripts/docker-cleanup.sh all

# Stop Services
docker compose -f docker-compose.optimized.yml down
```

### File Locations

- **Configuration**: `docker-compose.optimized.yml`
- **Environment**: `.env` (copy from `.env.production`)
- **Backups**: `./backups/`
- **Scripts**: `./scripts/`
- **Logs**: Docker logs (auto-rotated)

### Network Information

- **Application Port**: `8000` (host) → `8000` (container)
- **Database Port**: Internal only (not exposed)
- **Network Name**: `rust-backend-starter_app-network`

---

For additional support, refer to the [Docker Analysis Report](DOCKER_ANALYSIS_REPORT.md) and [Optimization Report](OPTIMIZATION_REPORT.md).
