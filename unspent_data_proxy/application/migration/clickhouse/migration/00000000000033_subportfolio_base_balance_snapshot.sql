-- migrate:up
CREATE MATERIALIZED VIEW unspentio.subportfolio_base_balance_snapshot_materialized_view
TO unspentio.subportfolio_base_balance_snapshot
AS
SELECT
    bbs.user_id	AS user_id,
    sa.subportfolio_id AS subportfolio_id,
    bbs.exchange_id AS exchange_id,
    bbs.exchange_name AS exchange_name,
    bbs.wallet_id AS wallet_id,
    bbs.wallet_address AS wallet_address,
    bbs.wallet_label AS wallet_label,
    bbs.asset_network AS asset_network,
    bbs.asset_chain_id AS asset_chain_id,
    bbs.asset_id AS asset_id,
    bbs.amount AS amount,
    bbs.created_at AS created_at
FROM unspentio.base_balance_snapshot AS bbs
INNER JOIN unspentio.subportfolio_asset AS sa
ON
    bbs.user_id = sa.user_id
    AND bbs.exchange_id = sa.exchange_id
    AND bbs.wallet_id = sa.wallet_id
    AND bbs.asset_network = sa.asset_network
    AND bbs.asset_chain_id = sa.asset_chain_id
    AND bbs.asset_id = sa.asset_id
SETTINGS
    final = 1;

-- migrate:down
DROP VIEW unspentio.subportfolio_base_balance_snapshot_materialized_view SYNC;