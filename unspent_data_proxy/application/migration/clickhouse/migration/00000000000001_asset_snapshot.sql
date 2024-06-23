-- migrate:up
CREATE TABLE unspentio.asset_snapshot
(
    asset_id String CODEC(LZ4),
    price_usd Decimal128(19) CODEC(LZ4),
    price_btc Nullable(Decimal128(19)) CODEC(LZ4),
    created_at DateTime('UTC') CODEC(LZ4)
) ENGINE = ReplacingMergeTree()
ORDER BY (created_at, asset_id)
PRIMARY KEY (created_at, asset_id)
SETTINGS
    index_granularity = 8192
COMMENT 'Table to store assets for all time.';

-- migrate:down
DROP TABLE unspentio.asset_snapshot SYNC;