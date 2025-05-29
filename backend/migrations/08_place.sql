-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS place_id_seq;

-- Table Definition
CREATE TABLE places (
    "id" int4 NOT NULL DEFAULT nextval('place_id_seq'::regclass),
    "user_username" varchar(48) NOT NULL,
    "latitude" text NOT NULL,
    "longitude" text NOT NULL,
    "created_at" timestamp DEFAULT now(),
    PRIMARY KEY ("id")
);