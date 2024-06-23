-- migrate:up
CREATE TABLE unspentio.subportfolio_link
(
    id String CODEC(LZ4),
    user_id Int32 CODEC(LZ4),
    subportfolio_id String CODEC(LZ4),
    is_active Bool CODEC(LZ4),
    description Nullable(String) CODEC(LZ4),
    created_at DateTime('UTC') CODEC(LZ4),
    updated_at DateTime('UTC') CODEC(LZ4),
    is_deleted UInt8 CODEC(LZ4)
) ENGINE = ReplacingMergeTree(updated_at, is_deleted)
ORDER BY (id)
PRIMARY KEY (id)
SETTINGS
    index_granularity = 32;

-- migrate:down
DROP TABLE unspentio.subportfolio_link SYNC;