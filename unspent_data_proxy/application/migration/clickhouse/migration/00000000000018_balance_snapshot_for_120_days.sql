-- migrate:up
CREATE MATERIALIZED VIEW unspentio.balance_snapshot_for_120_days_materialized_view
TO unspentio.balance_snapshot_for_120_days
AS
SELECT
    bs.user_id AS user_id,
    bs.asset_id AS asset_id,
    bs.total_amount AS total_amount,
    bs.created_at AS created_at
FROM unspentio.balance_snapshot_for_30_days AS bs
WHERE dateDiff('hour', toStartOfInterval(bs.created_at, INTERVAL 1 DAY, 'UTC'), bs.created_at, 'UTC') = 0;

-- migrate:down
DROP VIEW unspentio.balance_snapshot_for_120_days_materialized_view SYNC;