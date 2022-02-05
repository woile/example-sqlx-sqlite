-- Add up migration script here
CREATE TABLE IF NOT EXISTS `users__users` (
    `uuid` BLOB PRIMARY KEY NOT NULL,
    `username` TEXT NOT NULL UNIQUE,
    `password_hash` TEXT NOT NULL,
    `email` TEXT NOT NULL UNIQUE,
    `full_name` TEXT NOT NULL,
    `is_active` BOOLEAN NOT NULL DEFAULT 0,
    `date_joined` TEXT NOT NULL,
    `last_login` TEXT NULL
);
