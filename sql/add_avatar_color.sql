-- Migration: Add avatar_color column to users table
-- This is safe to run multiple times (column will only be added if it doesn't exist)
-- Run this manually on your D1 database:
-- npx wrangler d1 execute <YOUR_DB_NAME> --file=./sql/add_avatar_color.sql

ALTER TABLE users ADD COLUMN avatar_color TEXT;
