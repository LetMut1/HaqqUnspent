-- migrate:up
CREATE MATERIALIZED VIEW unspentio.balance_snapshot_for_7_days_materialized_view
TO unspentio.balance_snapshot_for_7_days
AS
SELECT
    bs.user_id AS user_id,
    bs.asset_id AS asset_id,
    bs.total_amount AS total_amount,
    CASE
        WHEN dateDiff('minute', toStartOfInterval(bs.created_at, INTERVAL 1 HOUR, 'UTC') AS ca, bs.created_at, 'UTC') < 30
        THEN ca
        ELSE ca + INTERVAL 1 HOUR
    END AS created_at
FROM unspentio.balance_snapshot AS bs;

-- migrate:down
DROP VIEW unspentio.balance_snapshot_for_7_days_materialized_view SYNC;