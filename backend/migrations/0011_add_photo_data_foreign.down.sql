ALTER TABLE photo_data
    DROP CONSTRAINT "photo_data_photo_id_fkey",
    DROP CONSTRAINT "photo_data_lost_people_id_fkey",
    DROP CONSTRAINT "photo_data_place_id_fkey";