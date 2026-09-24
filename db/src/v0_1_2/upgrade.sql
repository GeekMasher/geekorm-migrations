-- This migration will update the schema

ALTER TABLE Users ADD COLUMN user_type BLOB NOT NULL DEFAULT '';

