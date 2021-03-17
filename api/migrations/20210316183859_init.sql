-- Add migration script here
CREATE TABLE guild (
    id BIGINT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    icon_url TEXT NOT NULL,
    locale TEXT NOT NULL DEFAULT 'ja-JP'
)
