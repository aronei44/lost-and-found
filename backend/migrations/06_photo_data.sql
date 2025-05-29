-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS photo_data_id_seq;

-- Table Definition
CREATE TABLE photo_data (
    "id" int4 NOT NULL DEFAULT nextval('photo_data_id_seq'::regclass),
    "photo_id" int4 NOT NULL,
    "lost_people_id" int4 NOT NULL,
    "place_id" int4,
    PRIMARY KEY ("id")
);