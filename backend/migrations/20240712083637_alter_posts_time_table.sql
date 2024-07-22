-- Add migration script here
-- 添加 TIMESTAMPTZ 字段
ALTER TABLE posts
    ALTER COLUMN created_at TYPE TIMESTAMPTZ
    USING created_at AT TIME ZONE 'UTC';
