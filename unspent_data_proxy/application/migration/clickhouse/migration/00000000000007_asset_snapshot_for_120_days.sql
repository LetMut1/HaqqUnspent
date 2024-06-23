-- migrate:up
CREATE MATERIALIZED VIEW unspentio.asset_snapshot_for_120_days_materialized_view
TO unspentio.asset_snapshot_for_120_days
AS
SELECT
    as.asset_id AS asset_id,
    as.price_usd AS price_usd,
    as.price_btc AS price_btc,
    as.created_at AS created_at
FROM unspentio.asset_snapshot_for_30_days AS as
WHERE dateDiff('hour', toStartOfInterval(as.created_at, INTERVAL 1 DAY, 'UTC'), as.created_at, 'UTC') = 0;

-- migrate:down
DROP VIEW unspentio.asset_snapshot_for_120_days_materialized_view SYNC;