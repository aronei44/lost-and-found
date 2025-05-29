
ALTER TABLE found_people
    ADD CONSTRAINT "found_people_place_id_fkey" FOREIGN KEY ("place_id") REFERENCES place("id") ON DELETE CASCADE,
    ADD CONSTRAINT "found_people_lost_people_id_fkey" FOREIGN KEY ("lost_people_id") REFERENCES lost_people("id") ON DELETE CASCADE;
