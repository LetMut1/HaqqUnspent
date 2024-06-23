-- migrate:up
CREATE TABLE unspentio.subportfolio_asset
(
    user_id Int32 CODEC(LZ4),
    subportfolio_id String CODEC(LZ4),
    exchange_id String DEFAULT '' CODEC(LZ4),
    exchange_name String DEFAULT '' CODEC(LZ4),
    wallet_id Int32 DEFAULT -2147483648 CODEC(LZ4),
    wallet_address String DEFAULT '' CODEC(LZ4),
    wallet_label String DEFAULT '' CODEC(LZ4),
    asset_network String DEFAULT '' CODEC(LZ4),
    asset_chain_id Int32 DEFAULT -2147483648 CODEC(LZ4),
    asset_id String CODEC(LZ4),
    created_at DateTime('UTC') CODEC(LZ4),
    updated_at DateTime('UTC') CODEC(LZ4),
    is_deleted UInt8 CODEC(LZ4)
) ENGINE = ReplacingMergeTree(updated_at, is_deleted)
ORDER BY (user_id, subportfolio_id, exchange_id, wallet_id, asset_network, asset_chain_id, asset_id)
PRIMARY KEY (user_id, subportfolio_id, exchange_id, wallet_id, asset_network, asset_chain_id, asset_id)
SETTINGS
    index_granularity = 1024;

-- migrate:down
DROP TABLE unspentio.subportfolio_asset SYNC;