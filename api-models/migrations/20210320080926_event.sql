-- Add migration script here
CREATE TABLE events (
    id SERIAL PRIMARY KEY NOT NULL,
    guild_id BIGINT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    notifications TEXT[] NOT NULL,
    color TEXT NOT NULL DEFAULT '#0000ff',
    is_all_day BOOLEAN NOT NULL DEFAULT FALSE,
    start_at TIMESTAMP NOT NULL,
    end_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE event_settings (
    guild_id BIGINT PRIMARY KEY NOT NULL,
    channel_id BIGINT NOT NULL
);
