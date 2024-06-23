-- migrate:up
CREATE MATERIALIZED VIEW unspentio.subportfolio_base_balance_snapshot_for_7_days_materialized_view
TO unspentio.subportfolio_base_balance_snapshot_for_7_days
AS
SELECT
    sbbs.user_id AS user_id,
    sbbs.subportfolio_id AS subportfolio_id,
    sbbs.exchange_id AS exchange_id,
    sbbs.exchange_name AS exchange_name,
    sbbs.wallet_id AS wallet_id,
    sbbs.wallet_address AS wallet_address,
    sbbs.wallet_label AS wallet_label,
    sbbs.asset_network AS asset_network,
    sbbs.asset_chain_id AS asset_chain_id,
    sbbs.asset_id AS asset_id,
    sbbs.amount AS amount,
    CASE
        WHEN dateDiff('minute', toStartOfInterval(sbbs.created_at, INTERVAL 1 HOUR, 'UTC') AS ca, sbbs.created_at, 'UTC') < 30
        THEN ca
        ELSE ca + INTERVAL 1 HOUR
    END AS created_at
FROM unspentio.subportfolio_base_balance_snapshot AS sbbs;

-- migrate:down
DROP VIEW unspentio.subportfolio_base_balance_snapshot_for_7_days_materialized_view SYNC;