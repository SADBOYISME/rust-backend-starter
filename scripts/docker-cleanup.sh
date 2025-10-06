#!/bin/bash

# Docker Cleanup Script for Rust Backend Starter
# This script helps maintain Docker system hygiene

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}"
}

error() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ERROR: $1${NC}" >&2
}

warn() {
    echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] WARNING: $1${NC}"
}

info() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')] INFO: $1${NC}"
}

# Function to show Docker system usage
show_usage() {
    log "Current Docker System Usage:"
    echo "----------------------------------------"
    docker system df
    echo "----------------------------------------"
}

# Function to cleanup images
cleanup_images() {
    log "Cleaning up unused Docker images..."
    docker image prune -a -f
    log "Image cleanup completed"
}

# Function to cleanup volumes
cleanup_volumes() {
    warn "Volume cleanup will remove all unused volumes."
    read -p "Are you sure you want to continue? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        log "Cleaning up unused Docker volumes..."
        docker volume prune -f
        log "Volume cleanup completed"
    else
        info "Volume cleanup skipped"
    fi
}

# Function to cleanup build cache
cleanup_build_cache() {
    log "Cleaning up Docker build cache..."
    docker builder prune -a -f
    log "Build cache cleanup completed"
}

# Function to cleanup containers
cleanup_containers() {
    log "Cleaning up stopped Docker containers..."
    docker container prune -f
    log "Container cleanup completed"
}

# Function to full system cleanup
full_cleanup() {
    warn "Full system cleanup will remove all unused Docker resources."
    read -p "Are you sure you want to continue? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        log "Performing full Docker system cleanup..."
        docker system prune -a -f --volumes
        log "Full system cleanup completed"
    else
        info "Full system cleanup skipped"
    fi
}

# Function to show project-specific containers
show_project_containers() {
    log "Project-specific containers:"
    echo "----------------------------------------"
    docker ps -a --filter "name=rust_starter" --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
    echo "----------------------------------------"
}

# Function to cleanup project-specific resources
cleanup_project() {
    log "Cleaning up project-specific resources..."
    
    # Stop and remove project containers
    if docker ps -a --filter "name=rust_starter" --quiet | grep -q .; then
        log "Stopping project containers..."
        docker stop $(docker ps -a --filter "name=rust_starter" --quiet) || true
        log "Removing project containers..."
        docker rm $(docker ps -a --filter "name=rust_starter" --quiet) || true
    else
        info "No project containers found"
    fi
    
    # Remove project images
    if docker images --filter "reference=rust-backend-starter" --quiet | grep -q .; then
        log "Removing project images..."
        docker rmi $(docker images --filter "reference=rust-backend-starter" --quiet) || true
    else
        info "No project images found"
    fi
    
    log "Project cleanup completed"
}

# Main menu
case "${1:-}" in
    "usage"|"")
        show_usage
        ;;
    "images")
        cleanup_images
        ;;
    "volumes")
        cleanup_volumes
        ;;
    "cache")
        cleanup_build_cache
        ;;
    "containers")
        cleanup_containers
        ;;
    "project")
        cleanup_project
        ;;
    "full")
        full_cleanup
        ;;
    "all")
        show_usage
        cleanup_containers
        cleanup_images
        cleanup_build_cache
        log "Standard cleanup completed. Run '$0 full' for complete cleanup."
        ;;
    "help"|"-h"|"--help")
        echo "Docker Cleanup Script Usage:"
        echo "  $0              - Show Docker system usage"
        echo "  $0 usage        - Show Docker system usage"
        echo "  $0 images       - Clean up unused images"
        echo "  $0 volumes      - Clean up unused volumes (interactive)"
        echo "  $0 cache        - Clean up build cache"
        echo "  $0 containers   - Clean up stopped containers"
        echo "  $0 project      - Clean up project-specific resources"
        echo "  $0 full         - Full system cleanup (interactive)"
        echo "  $0 all          - Run standard cleanup (containers, images, cache)"
        echo "  $0 help         - Show this help message"
        ;;
    *)
        error "Unknown command: $1"
        echo "Run '$0 help' for usage information."
        exit 1
        ;;
esac
