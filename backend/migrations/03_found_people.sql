-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS found_people_id_seq;

-- Table Definition
CREATE TABLE found_people (
    "id" int4 NOT NULL DEFAULT nextval('found_people_id_seq'::regclass),
    "place_id" int4 NOT NULL,
    "lost_people_id" int4 NOT NULL,
    PRIMARY KEY ("id")
);