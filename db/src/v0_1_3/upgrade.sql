-- This migration will update the schema

ALTER TABLE Users ADD COLUMN last_login TEXT NOT NULL DEFAULT '';

