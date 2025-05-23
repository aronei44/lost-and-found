-- Add pgcrypto extension for UUID generation
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS users_id_seq;

-- Table Definition
CREATE TABLE IF NOT EXISTS users (
    "id" int4 NOT NULL DEFAULT nextval('users_id_seq'::regclass),
    "username" varchar(48) NOT NULL,
    "password" text NOT NULL,
    "created_at" timestamp DEFAULT now(),
    "last_active" timestamp DEFAULT now()
);