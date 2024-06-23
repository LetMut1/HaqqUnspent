-- migrate:up
CREATE MATERIALIZED VIEW unspentio.asset_snapshot_last_materialized_view
TO unspentio.asset_snapshot_last
AS
SELECT
    as.asset_id AS asset_id,
    as.price_usd AS price_usd,
    as.price_btc AS price_btc,
    as.created_at AS created_at
FROM unspentio.asset_snapshot AS as;

-- migrate:down
DROP VIEW unspentio.asset_snapshot_last_materialized_view SYNC;