ALTER TABLE monitor_people
    ADD CONSTRAINT "monitor_people_user_username_fkey"
        FOREIGN KEY ("user_username") REFERENCES users("username") ON DELETE CASCADE,
    ADD CONSTRAINT "monitor_people_lost_people_id_fkey"
        FOREIGN KEY ("lost_people_id") REFERENCES lost_people("id") ON DELETE CASCADE;
