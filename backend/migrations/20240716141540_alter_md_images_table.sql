-- Add migration script here
ALTER TABLE markdowns
ADD COLUMN image_urls TEXT[];
