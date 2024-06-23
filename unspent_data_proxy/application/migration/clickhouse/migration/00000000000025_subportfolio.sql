-- migrate:up
CREATE TABLE unspentio.subportfolio
(
    user_id Int32 CODEC(LZ4),
    id String CODEC(LZ4),
    name String CODEC(LZ4),
    description Nullable(String) CODEC(LZ4),
    created_at DateTime('UTC') CODEC(LZ4),
    updated_at DateTime('UTC') CODEC(LZ4),
    is_deleted UInt8 CODEC(LZ4)
) ENGINE = ReplacingMergeTree(updated_at, is_deleted)
ORDER BY (user_id, id)
PRIMARY KEY (user_id, id)
SETTINGS
    index_granularity = 32;

-- migrate:down
DROP TABLE unspentio.subportfolio SYNC;