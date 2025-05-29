ALTER TABLE photo_data
    ADD CONSTRAINT "photo_data_photo_id_fkey" FOREIGN KEY ("photo_id") REFERENCES photo("id") ON DELETE CASCADE,
    ADD CONSTRAINT "photo_data_lost_people_id_fkey" FOREIGN KEY ("lost_people_id") REFERENCES lost_people("id") ON DELETE CASCADE,
    ADD CONSTRAINT "photo_data_place_id_fkey" FOREIGN KEY ("place_id") REFERENCES place("id") ON DELETE CASCADE;
