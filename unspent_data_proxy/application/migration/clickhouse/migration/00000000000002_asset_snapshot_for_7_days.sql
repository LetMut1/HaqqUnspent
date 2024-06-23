-- migrate:up
CREATE TABLE unspentio.asset_snapshot_for_7_days
(
    asset_id String CODEC(LZ4),
    price_usd Decimal128(19) CODEC(LZ4),
    price_btc Nullable(Decimal128(19)) CODEC(LZ4),
    created_at DateTime('UTC') CODEC(LZ4)
) ENGINE = ReplacingMergeTree()
ORDER BY (created_at, asset_id)
PRIMARY KEY (created_at, asset_id)
TTL created_at + INTERVAL 8 DAY
    DELETE
SETTINGS
    index_granularity = 8192
COMMENT 'Table to store assets for 7 days at the beginning of every hour.';

-- migrate:down
DROP TABLE unspentio.asset_snapshot_for_7_days SYNC;