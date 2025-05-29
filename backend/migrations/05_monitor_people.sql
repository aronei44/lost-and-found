-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Table Definition
CREATE TABLE monitor_people (
    "user_username" varchar(48) NOT NULL,
    "lost_people_id" int4 NOT NULL,
    PRIMARY KEY ("user_username","lost_people_id")
);