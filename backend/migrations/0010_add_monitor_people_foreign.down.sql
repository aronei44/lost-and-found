ALTER TABLE monitor_people
    DROP CONSTRAINT IF EXISTS "monitor_people_user_username_fkey",
    DROP CONSTRAINT IF EXISTS "monitor_people_lost_people_id_fkey";