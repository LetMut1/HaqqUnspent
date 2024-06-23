-- migrate:up
CREATE MATERIALIZED VIEW unspentio.balance_snapshot_for_366_days_materialized_view
TO unspentio.balance_snapshot_for_366_days
AS
SELECT
    bs.user_id AS user_id,
    bs.asset_id AS asset_id,
    bs.total_amount AS total_amount,
    bs.created_at AS created_at
FROM unspentio.balance_snapshot_for_120_days AS bs
WHERE (toDayOfWeek(bs.created_at, 0, 'UTC') AS d) = 1 OR d = 4;

-- migrate:down
DROP VIEW unspentio.balance_snapshot_for_366_days_materialized_view SYNC;