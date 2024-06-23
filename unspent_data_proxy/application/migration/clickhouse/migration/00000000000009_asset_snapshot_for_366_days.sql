-- migrate:up
CREATE MATERIALIZED VIEW unspentio.asset_snapshot_for_366_days_materialized_view
TO unspentio.asset_snapshot_for_366_days
AS
SELECT
    as.asset_id AS asset_id,
    as.price_usd AS price_usd,
    as.price_btc AS price_btc,
    as.created_at AS created_at
FROM unspentio.asset_snapshot_for_120_days AS as
WHERE (toDayOfWeek(as.created_at, 0, 'UTC') AS d) = 1 OR d = 4;

-- migrate:down
DROP VIEW unspentio.asset_snapshot_for_366_days_materialized_view SYNC;