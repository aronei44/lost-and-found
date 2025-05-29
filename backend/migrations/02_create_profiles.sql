
-- Table Definition
CREATE TABLE IF NOT EXISTS profiles (
    "username" varchar(48) NOT NULL,
    "full_name" text NOT NULL,
    "email" text,
    "phone" text,
    "address" text,
    CONSTRAINT "profiles_username_fkey" FOREIGN KEY ("username") REFERENCES users("username") ON DELETE CASCADE,
    PRIMARY KEY ("username")
);