-- migrate:up
CREATE MATERIALIZED VIEW unspentio.asset_snapshot_for_7_days_materialized_view
TO unspentio.asset_snapshot_for_7_days
AS
SELECT
    as.asset_id AS asset_id,
    as.price_usd AS price_usd,
    as.price_btc AS price_btc,
    CASE
        WHEN dateDiff('minute', toStartOfInterval(as.created_at, INTERVAL 1 HOUR, 'UTC') AS ca, as.created_at, 'UTC') < 30
        THEN ca
        ELSE ca + INTERVAL 1 HOUR
    END AS created_at
FROM unspentio.asset_snapshot AS as;

-- migrate:down
DROP VIEW unspentio.asset_snapshot_for_7_days_materialized_view SYNC;