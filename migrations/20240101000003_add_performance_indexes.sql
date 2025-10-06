-- Add performance indexes for items table
-- Note: CONCURRENTLY removed as SQLx runs migrations in transactions

-- Index on user_id for faster lookups
CREATE INDEX IF NOT EXISTS idx_items_user_id 
    ON items(user_id);

-- Index on status for filtering
CREATE INDEX IF NOT EXISTS idx_items_status 
    ON items(status);

-- Index on created_at for sorting
CREATE INDEX IF NOT EXISTS idx_items_created_at 
    ON items(created_at DESC);

-- Composite index for common query pattern: WHERE user_id = X ORDER BY created_at
CREATE INDEX IF NOT EXISTS idx_items_user_created 
    ON items(user_id, created_at DESC);

-- Composite index for status filtering per user
CREATE INDEX IF NOT EXISTS idx_items_user_status 
    ON items(user_id, status, created_at DESC);
