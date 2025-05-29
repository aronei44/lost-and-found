-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS photo_id_seq;

-- Table Definition
CREATE TABLE photos (
    "id" int4 NOT NULL DEFAULT nextval('photo_id_seq'::regclass),
    "bucket" varchar(48) NOT NULL,
    "path" text NOT NULL,
    PRIMARY KEY ("id")
);