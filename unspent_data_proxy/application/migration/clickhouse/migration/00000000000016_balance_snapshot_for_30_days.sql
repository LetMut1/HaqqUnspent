-- migrate:up
CREATE MATERIALIZED VIEW unspentio.balance_snapshot_for_30_days_materialized_view
TO unspentio.balance_snapshot_for_30_days
AS
SELECT
    bs.user_id AS user_id,
    bs.asset_id AS asset_id,
    bs.total_amount AS total_amount,
    CASE
        WHEN dateDiff('hour', toStartOfInterval(bs.created_at, INTERVAL 12 HOUR, 'UTC') AS ca, bs.created_at, 'UTC') AS dd = 0
        THEN ca
        WHEN dd = 11
        THEN ca + INTERVAL 12 HOUR
        ELSE ca
    END AS created_at
FROM unspentio.balance_snapshot_for_7_days AS bs
WHERE dd = 0 OR dd = 11;

-- migrate:down
DROP VIEW unspentio.balance_snapshot_for_30_days_materialized_view SYNC;