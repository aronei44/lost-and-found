
-- Table Definition
CREATE TABLE IF NOT EXISTS users (
    "username" varchar(48) NOT NULL,
    "password" text NOT NULL,
    "created_at" timestamp DEFAULT now(),
    "last_active" timestamp DEFAULT now(),
    PRIMARY KEY ("username")
);
