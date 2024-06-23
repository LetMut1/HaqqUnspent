-- migrate:up
ALTER TABLE unspentio.subportfolio_link ADD PROJECTION projection_1
(
    SELECT
        id,
        user_id,
        subportfolio_id,
        is_active,
        description,
        created_at,
        updated_at,
        is_deleted
    ORDER BY (user_id, subportfolio_id)
);

-- migrate:down
ALTER TABLE unspentio.subportfolio_link DROP PROJECTION projection_1;