-- Migration: Add owner column to accounts table
-- Date: October 8, 2025
-- Safe: Column will only be added if it doesn't exist

-- Step 1: Check if column exists (SQLite doesn't support IF NOT EXISTS for ALTER TABLE)
-- So we'll handle this in Rust code

-- Step 2: Add owner column
ALTER TABLE accounts ADD COLUMN owner TEXT;

-- Step 3: Set default value for existing accounts
UPDATE accounts SET owner = 'Ioan' WHERE owner IS NULL;