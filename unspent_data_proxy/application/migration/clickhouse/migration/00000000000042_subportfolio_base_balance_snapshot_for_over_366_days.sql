-- migrate:up
CREATE TABLE unspentio.subportfolio_base_balance_snapshot_for_over_366_days
(
    user_id	Int32 CODEC(LZ4),
    subportfolio_id String CODEC(LZ4),
    exchange_id String DEFAULT '' CODEC(LZ4),
    exchange_name String DEFAULT '' CODEC(LZ4),
    wallet_id Int32 DEFAULT -2147483648 CODEC(LZ4),
    wallet_address String DEFAULT '' CODEC(LZ4),
    wallet_label String DEFAULT '' CODEC(LZ4),
    asset_network String DEFAULT '' CODEC(LZ4),
    asset_chain_id Int32 DEFAULT -2147483648 CODEC(LZ4),
    asset_id String CODEC(LZ4),
    amount Decimal128(19) CODEC(LZ4),
    created_at DateTime('UTC') CODEC(LZ4)
) ENGINE = ReplacingMergeTree()
ORDER BY (user_id, subportfolio_id, created_at, asset_id, exchange_id, wallet_id, asset_network, asset_chain_id)
PRIMARY KEY (user_id, subportfolio_id, created_at, asset_id, exchange_id, wallet_id, asset_network, asset_chain_id)
SETTINGS
    index_granularity = 8192
COMMENT 'Table to store subportfolio base balances for over 366 years at the beginning of every week.';

-- migrate:down
DROP TABLE unspentio.subportfolio_base_balance_snapshot_for_over_366_days SYNC;