ALTER TABLE place
    ADD CONSTRAINT "place_user_username_fkey"
    FOREIGN KEY ("user_username") REFERENCES users("username") ON DELETE CASCADE;
