ALTER TABLE photo_data
    ADD CONSTRAINT "photo_data_photos_id_fkey" FOREIGN KEY ("photo_id") REFERENCES photos("id") ON DELETE CASCADE,
    ADD CONSTRAINT "photo_data_lost_people_id_fkey" FOREIGN KEY ("lost_people_id") REFERENCES lost_people("id") ON DELETE CASCADE,
    ADD CONSTRAINT "photo_data_places_id_fkey" FOREIGN KEY ("place_id") REFERENCES places("id") ON DELETE CASCADE;
