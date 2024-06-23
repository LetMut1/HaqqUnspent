-- migrate:up
CREATE MATERIALIZED VIEW unspentio.asset_snapshot_for_30_days_materialized_view
TO unspentio.asset_snapshot_for_30_days
AS
SELECT
    as.asset_id AS asset_id,
    as.price_usd AS price_usd,
    as.price_btc AS price_btc,
    CASE
        WHEN dateDiff('hour', toStartOfInterval(as.created_at, INTERVAL 12 HOUR, 'UTC') AS ca, as.created_at, 'UTC') AS dd = 0
        THEN ca
        WHEN dd = 11
        THEN ca + INTERVAL 12 HOUR
        ELSE ca
    END AS created_at
FROM unspentio.asset_snapshot_for_7_days AS as
WHERE dd = 0 OR dd = 11;

-- migrate:down
DROP VIEW unspentio.asset_snapshot_for_30_days_materialized_view SYNC;