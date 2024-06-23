-- migrate:up
CREATE TABLE unspentio.balance_snapshot_for_120_days
(
    user_id	Int32 CODEC(LZ4),
    asset_id String CODEC(LZ4),
    total_amount Decimal128(19) CODEC(LZ4),
    created_at DateTime('UTC') CODEC(LZ4)
) ENGINE = ReplacingMergeTree()
ORDER BY (user_id, created_at, asset_id)
PRIMARY KEY (user_id, created_at, asset_id)
TTL created_at + INTERVAL 122 DAY
    DELETE
SETTINGS
    index_granularity = 8192
COMMENT 'Table to store balances for 120 days at the beginning of every day.';

-- migrate:down
DROP TABLE unspentio.balance_snapshot_for_120_days SYNC;