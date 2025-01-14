-- This migration will update the schema

ALTER TABLE Users ADD COLUMN password TEXT NOT NULL DEFAULT '';

ALTER TABLE Posts ADD COLUMN created_at TEXT NOT NULL DEFAULT '';

ALTER TABLE Posts ADD COLUMN updated_at TEXT NOT NULL DEFAULT '';

