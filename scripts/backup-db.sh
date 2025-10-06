#!/bin/bash

# Database Backup Script for Rust Backend Starter
# This script creates automated backups of the PostgreSQL database

set -euo pipefail

# Configuration
CONTAINER_NAME="rust_starter_db"
BACKUP_DIR="/backups"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
BACKUP_FILE="${BACKUP_DIR}/rust_starter_db_backup_${TIMESTAMP}.sql"
RETENTION_DAYS=7

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Logging function
log() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}"
}

error() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ERROR: $1${NC}" >&2
}

warn() {
    echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] WARNING: $1${NC}"
}

# Check if container is running
if ! docker ps --format "table {{.Names}}" | grep -q "^${CONTAINER_NAME}$"; then
    error "Container ${CONTAINER_NAME} is not running"
    exit 1
fi

# Create backup directory if it doesn't exist
if ! docker exec "${CONTAINER_NAME}" test -d "${BACKUP_DIR}"; then
    log "Creating backup directory ${BACKUP_DIR}"
    docker exec "${CONTAINER_NAME}" mkdir -p "${BACKUP_DIR}"
fi

# Get database credentials from environment
DB_USER=$(docker exec "${CONTAINER_NAME}" env | grep POSTGRES_USER | cut -d'=' -f2)
DB_NAME=$(docker exec "${CONTAINER_NAME}" env | grep POSTGRES_DB | cut -d'=' -f2)

if [[ -z "$DB_USER" || -z "$DB_NAME" ]]; then
    error "Could not retrieve database credentials from container"
    exit 1
fi

log "Starting database backup for ${DB_NAME}..."

# Create backup
if docker exec "${CONTAINER_NAME}" pg_dump -U "${DB_USER}" "${DB_NAME}" > "${BACKUP_FILE}"; then
    log "Backup created successfully: ${BACKUP_FILE}"
    
    # Compress the backup
    log "Compressing backup..."
    gzip "${BACKUP_FILE}"
    COMPRESSED_FILE="${BACKUP_FILE}.gz"
    
    # Get file size
    FILE_SIZE=$(du -h "${COMPRESSED_FILE}" | cut -f1)
    log "Backup compressed: ${COMPRESSED_FILE} (${FILE_SIZE})"
    
    # Clean up old backups
    log "Cleaning up backups older than ${RETENTION_DAYS} days..."
    find "${BACKUP_DIR}" -name "rust_starter_db_backup_*.sql.gz" -type f -mtime +${RETENTION_DAYS} -delete
    
    # List remaining backups
    BACKUP_COUNT=$(find "${BACKUP_DIR}" -name "rust_starter_db_backup_*.sql.gz" -type f | wc -l)
    log "Backup completed. ${BACKUP_COUNT} backups retained."
    
else
    error "Backup failed"
    exit 1
fi

# Verify backup integrity
log "Verifying backup integrity..."
if gzip -t "${COMPRESSED_FILE}"; then
    log "Backup integrity verified"
else
    error "Backup integrity check failed"
    exit 1
fi

log "Backup process completed successfully"
