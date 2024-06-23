-- migrate:up
ALTER TABLE unspentio.asset_snapshot_for_7_days ADD PROJECTION projection_1
(
    SELECT
        asset_id,
        price_usd,
        price_btc,
        created_at
    ORDER BY (asset_id, created_at)
);

-- migrate:down
ALTER TABLE unspentio.asset_snapshot_for_7_days DROP PROJECTION projection_1;