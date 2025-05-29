-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS lost_people_id_seq;

-- Table Definition
CREATE TABLE lost_people (
    "id" int4 NOT NULL DEFAULT nextval('lost_people_id_seq'::regclass),
    "fullname" varchar(256) NOT NULL,
    "alias" varchar(48),
    "gender" varchar(1) DEFAULT 'L'::character varying,
    "born_date" date,
    "last_condition" text,
    "is_found" bool DEFAULT false,
    "lost_date" date NOT NULL,
    "found_date" date,
    PRIMARY KEY ("id")
);