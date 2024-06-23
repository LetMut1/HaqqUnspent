-- migrate:up
ALTER TABLE unspentio.subportfolio ADD PROJECTION projection_1
(
    SELECT
        user_id,
        id,
        name,
        description,
        created_at,
        updated_at,
        is_deleted
    ORDER BY (user_id, name)
);

-- migrate:down
ALTER TABLE unspentio.subportfolio DROP PROJECTION projection_1;