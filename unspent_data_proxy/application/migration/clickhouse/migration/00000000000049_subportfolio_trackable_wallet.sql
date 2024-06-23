-- migrate:up
CREATE TABLE unspentio.subportfolio_trackable_wallet
(
    wallet_id Int32 CODEC(LZ4),
    user_id Int32 CODEC(LZ4),
    subportfolio_id String CODEC(LZ4),
    created_at DateTime('UTC') CODEC(LZ4),
    updated_at DateTime('UTC') CODEC(LZ4),
    is_deleted UInt8 CODEC(LZ4)
) ENGINE = ReplacingMergeTree(updated_at, is_deleted)
ORDER BY (user_id, subportfolio_id, wallet_id)
PRIMARY KEY (user_id, subportfolio_id, wallet_id)
SETTINGS
    index_granularity = 32;

-- migrate:down
DROP TABLE unspentio.subportfolio_trackable_wallet SYNC;