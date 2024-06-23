-- migrate:up
CREATE TABLE unspentio.asset_snapshot_last
(
    asset_id String CODEC(LZ4),
    price_usd Decimal128(19) CODEC(LZ4),
    price_btc Nullable(Decimal128(19)) CODEC(LZ4),
    created_at DateTime('UTC') CODEC(LZ4)
) ENGINE = ReplacingMergeTree(created_at)
ORDER BY (asset_id)
PRIMARY KEY (asset_id)
SETTINGS
    index_granularity = 8192
COMMENT 'Table to store only last assets.';

-- migrate:down
DROP TABLE unspentio.asset_snapshot_last SYNC;