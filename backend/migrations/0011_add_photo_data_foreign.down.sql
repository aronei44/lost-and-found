ALTER TABLE photo_data
    DROP CONSTRAINT "photo_data_photos_id_fkey",
    DROP CONSTRAINT "photo_data_lost_people_id_fkey",
    DROP CONSTRAINT "photo_data_places_id_fkey";