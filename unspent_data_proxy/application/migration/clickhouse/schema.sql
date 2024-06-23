
--
-- Database schema
--

CREATE DATABASE IF NOT EXISTS unspentio;

CREATE TABLE unspentio.asset_snapshot
(
    `asset_id` String CODEC(LZ4),
    `price_usd` Decimal(38, 19) CODEC(LZ4),
    `price_btc` Nullable(Decimal(38, 19)) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (created_at, asset_id)
ORDER BY (created_at, asset_id)
SETTINGS index_granularity = 8192
COMMENT 'Table to store assets for all time.';

CREATE TABLE unspentio.asset_snapshot_for_120_days
(
    `asset_id` String CODEC(LZ4),
    `price_usd` Decimal(38, 19) CODEC(LZ4),
    `price_btc` Nullable(Decimal(38, 19)) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (created_at, asset_id)
ORDER BY (created_at, asset_id)
TTL created_at + toIntervalDay(122)
SETTINGS index_granularity = 8192
COMMENT 'Table to store assets for 120 days at the beginning of every day.';

CREATE MATERIALIZED VIEW unspentio.asset_snapshot_for_120_days_materialized_view TO unspentio.asset_snapshot_for_120_days
(
    `asset_id` String,
    `price_usd` Decimal(38, 19),
    `price_btc` Nullable(Decimal(38, 19)),
    `created_at` DateTime('UTC')
) AS
SELECT
    as.asset_id AS asset_id,
    as.price_usd AS price_usd,
    as.price_btc AS price_btc,
    as.created_at AS created_at
FROM unspentio.asset_snapshot_for_30_days AS as
WHERE dateDiff('hour', toStartOfInterval(as.created_at, toIntervalDay(1), 'UTC'), as.created_at, 'UTC') = 0;

CREATE TABLE unspentio.asset_snapshot_for_30_days
(
    `asset_id` String CODEC(LZ4),
    `price_usd` Decimal(38, 19) CODEC(LZ4),
    `price_btc` Nullable(Decimal(38, 19)) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4),
    PROJECTION projection_1
    (
        SELECT
            asset_id,
            price_usd,
            price_btc,
            created_at
        ORDER BY
            asset_id,
            created_at
    )
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (created_at, asset_id)
ORDER BY (created_at, asset_id)
TTL created_at + toIntervalDay(32)
SETTINGS index_granularity = 8192
COMMENT 'Table to store assets for 30 days at the beginning of every 12 hours starting from 00:00.';

CREATE MATERIALIZED VIEW unspentio.asset_snapshot_for_30_days_materialized_view TO unspentio.asset_snapshot_for_30_days
(
    `asset_id` String,
    `price_usd` Decimal(38, 19),
    `price_btc` Nullable(Decimal(38, 19)),
    `created_at` DateTime('UTC')
) AS
SELECT
    as.asset_id AS asset_id,
    as.price_usd AS price_usd,
    as.price_btc AS price_btc,
    multiIf((dateDiff('hour', toStartOfInterval(as.created_at, toIntervalHour(12), 'UTC') AS ca, as.created_at, 'UTC') AS dd) = 0, ca, dd = 11, ca + toIntervalHour(12), ca) AS created_at
FROM unspentio.asset_snapshot_for_7_days AS as
WHERE (dd = 0) OR (dd = 11);

CREATE TABLE unspentio.asset_snapshot_for_366_days
(
    `asset_id` String CODEC(LZ4),
    `price_usd` Decimal(38, 19) CODEC(LZ4),
    `price_btc` Nullable(Decimal(38, 19)) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (created_at, asset_id)
ORDER BY (created_at, asset_id)
TTL created_at + toIntervalDay(368)
SETTINGS index_granularity = 8192
COMMENT 'Table to store assets for 366 days at the beginning of every half of week - monday and thursday.';

CREATE MATERIALIZED VIEW unspentio.asset_snapshot_for_366_days_materialized_view TO unspentio.asset_snapshot_for_366_days
(
    `asset_id` String,
    `price_usd` Decimal(38, 19),
    `price_btc` Nullable(Decimal(38, 19)),
    `created_at` DateTime('UTC')
) AS
SELECT
    as.asset_id AS asset_id,
    as.price_usd AS price_usd,
    as.price_btc AS price_btc,
    as.created_at AS created_at
FROM unspentio.asset_snapshot_for_120_days AS as
WHERE ((toDayOfWeek(as.created_at, 0, 'UTC') AS d) = 1) OR (d = 4);

CREATE TABLE unspentio.asset_snapshot_for_7_days
(
    `asset_id` String CODEC(LZ4),
    `price_usd` Decimal(38, 19) CODEC(LZ4),
    `price_btc` Nullable(Decimal(38, 19)) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4),
    PROJECTION projection_1
    (
        SELECT
            asset_id,
            price_usd,
            price_btc,
            created_at
        ORDER BY
            asset_id,
            created_at
    )
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (created_at, asset_id)
ORDER BY (created_at, asset_id)
TTL created_at + toIntervalDay(8)
SETTINGS index_granularity = 8192
COMMENT 'Table to store assets for 7 days at the beginning of every hour.';

CREATE MATERIALIZED VIEW unspentio.asset_snapshot_for_7_days_materialized_view TO unspentio.asset_snapshot_for_7_days
(
    `asset_id` String,
    `price_usd` Decimal(38, 19),
    `price_btc` Nullable(Decimal(38, 19)),
    `created_at` DateTime('UTC')
) AS
SELECT
    as.asset_id AS asset_id,
    as.price_usd AS price_usd,
    as.price_btc AS price_btc,
    multiIf(dateDiff('minute', toStartOfInterval(as.created_at, toIntervalHour(1), 'UTC') AS ca, as.created_at, 'UTC') < 30, ca, ca + toIntervalHour(1)) AS created_at
FROM unspentio.asset_snapshot AS as;

CREATE TABLE unspentio.asset_snapshot_for_over_366_days
(
    `asset_id` String CODEC(LZ4),
    `price_usd` Decimal(38, 19) CODEC(LZ4),
    `price_btc` Nullable(Decimal(38, 19)) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (created_at, asset_id)
ORDER BY (created_at, asset_id)
SETTINGS index_granularity = 8192
COMMENT 'Table to store assets for over 366 years at the beginning of every week.';

CREATE MATERIALIZED VIEW unspentio.asset_snapshot_for_over_366_days_materialized_view TO unspentio.asset_snapshot_for_over_366_days
(
    `asset_id` String,
    `price_usd` Decimal(38, 19),
    `price_btc` Nullable(Decimal(38, 19)),
    `created_at` DateTime('UTC')
) AS
SELECT
    as.asset_id AS asset_id,
    as.price_usd AS price_usd,
    as.price_btc AS price_btc,
    as.created_at AS created_at
FROM unspentio.asset_snapshot_for_366_days AS as
WHERE toDayOfWeek(as.created_at, 0, 'UTC') = 1;

CREATE TABLE unspentio.asset_snapshot_last
(
    `asset_id` String CODEC(LZ4),
    `price_usd` Decimal(38, 19) CODEC(LZ4),
    `price_btc` Nullable(Decimal(38, 19)) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4)
)
ENGINE = ReplacingMergeTree(created_at)
PRIMARY KEY asset_id
ORDER BY asset_id
SETTINGS index_granularity = 8192
COMMENT 'Table to store only last assets.';

CREATE MATERIALIZED VIEW unspentio.asset_snapshot_last_materialized_view TO unspentio.asset_snapshot_last
(
    `asset_id` String,
    `price_usd` Decimal(38, 19),
    `price_btc` Nullable(Decimal(38, 19)),
    `created_at` DateTime('UTC')
) AS
SELECT
    as.asset_id AS asset_id,
    as.price_usd AS price_usd,
    as.price_btc AS price_btc,
    as.created_at AS created_at
FROM unspentio.asset_snapshot AS as;

CREATE TABLE unspentio.balance_snapshot
(
    `user_id` Int32 CODEC(LZ4),
    `asset_id` String CODEC(LZ4),
    `total_amount` Decimal(38, 19) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (user_id, created_at, asset_id)
ORDER BY (user_id, created_at, asset_id)
SETTINGS index_granularity = 8192
COMMENT 'Table to store balances for all time.';

CREATE TABLE unspentio.balance_snapshot_for_120_days
(
    `user_id` Int32 CODEC(LZ4),
    `asset_id` String CODEC(LZ4),
    `total_amount` Decimal(38, 19) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (user_id, created_at, asset_id)
ORDER BY (user_id, created_at, asset_id)
TTL created_at + toIntervalDay(122)
SETTINGS index_granularity = 8192
COMMENT 'Table to store balances for 120 days at the beginning of every day.';

CREATE MATERIALIZED VIEW unspentio.balance_snapshot_for_120_days_materialized_view TO unspentio.balance_snapshot_for_120_days
(
    `user_id` Int32,
    `asset_id` String,
    `total_amount` Decimal(38, 19),
    `created_at` DateTime('UTC')
) AS
SELECT
    bs.user_id AS user_id,
    bs.asset_id AS asset_id,
    bs.total_amount AS total_amount,
    bs.created_at AS created_at
FROM unspentio.balance_snapshot_for_30_days AS bs
WHERE dateDiff('hour', toStartOfInterval(bs.created_at, toIntervalDay(1), 'UTC'), bs.created_at, 'UTC') = 0;

CREATE TABLE unspentio.balance_snapshot_for_30_days
(
    `user_id` Int32 CODEC(LZ4),
    `asset_id` String CODEC(LZ4),
    `total_amount` Decimal(38, 19) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (user_id, created_at, asset_id)
ORDER BY (user_id, created_at, asset_id)
TTL created_at + toIntervalDay(32)
SETTINGS index_granularity = 8192
COMMENT 'Table to store balances for 30 days at the beginning of every 12 hours starting from 00:00.';

CREATE MATERIALIZED VIEW unspentio.balance_snapshot_for_30_days_materialized_view TO unspentio.balance_snapshot_for_30_days
(
    `user_id` Int32,
    `asset_id` String,
    `total_amount` Decimal(38, 19),
    `created_at` DateTime('UTC')
) AS
SELECT
    bs.user_id AS user_id,
    bs.asset_id AS asset_id,
    bs.total_amount AS total_amount,
    multiIf((dateDiff('hour', toStartOfInterval(bs.created_at, toIntervalHour(12), 'UTC') AS ca, bs.created_at, 'UTC') AS dd) = 0, ca, dd = 11, ca + toIntervalHour(12), ca) AS created_at
FROM unspentio.balance_snapshot_for_7_days AS bs
WHERE (dd = 0) OR (dd = 11);

CREATE TABLE unspentio.balance_snapshot_for_366_days
(
    `user_id` Int32 CODEC(LZ4),
    `asset_id` String CODEC(LZ4),
    `total_amount` Decimal(38, 19) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (user_id, created_at, asset_id)
ORDER BY (user_id, created_at, asset_id)
TTL created_at + toIntervalDay(368)
SETTINGS index_granularity = 8192
COMMENT 'Table to store balances for 366 days at the beginning of every half of week - monday and thursday.';

CREATE MATERIALIZED VIEW unspentio.balance_snapshot_for_366_days_materialized_view TO unspentio.balance_snapshot_for_366_days
(
    `user_id` Int32,
    `asset_id` String,
    `total_amount` Decimal(38, 19),
    `created_at` DateTime('UTC')
) AS
SELECT
    bs.user_id AS user_id,
    bs.asset_id AS asset_id,
    bs.total_amount AS total_amount,
    bs.created_at AS created_at
FROM unspentio.balance_snapshot_for_120_days AS bs
WHERE ((toDayOfWeek(bs.created_at, 0, 'UTC') AS d) = 1) OR (d = 4);

CREATE TABLE unspentio.balance_snapshot_for_7_days
(
    `user_id` Int32 CODEC(LZ4),
    `asset_id` String CODEC(LZ4),
    `total_amount` Decimal(38, 19) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (user_id, created_at, asset_id)
ORDER BY (user_id, created_at, asset_id)
TTL created_at + toIntervalDay(8)
SETTINGS index_granularity = 8192
COMMENT 'Table to store balances for 7 days at the beginning of every hour.';

CREATE MATERIALIZED VIEW unspentio.balance_snapshot_for_7_days_materialized_view TO unspentio.balance_snapshot_for_7_days
(
    `user_id` Int32,
    `asset_id` String,
    `total_amount` Decimal(38, 19),
    `created_at` DateTime('UTC')
) AS
SELECT
    bs.user_id AS user_id,
    bs.asset_id AS asset_id,
    bs.total_amount AS total_amount,
    multiIf(dateDiff('minute', toStartOfInterval(bs.created_at, toIntervalHour(1), 'UTC') AS ca, bs.created_at, 'UTC') < 30, ca, ca + toIntervalHour(1)) AS created_at
FROM unspentio.balance_snapshot AS bs;

CREATE TABLE unspentio.balance_snapshot_for_over_366_days
(
    `user_id` Int32 CODEC(LZ4),
    `asset_id` String CODEC(LZ4),
    `total_amount` Decimal(38, 19) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (user_id, created_at, asset_id)
ORDER BY (user_id, created_at, asset_id)
SETTINGS index_granularity = 8192
COMMENT 'Table to store balances for over 366 years at the beginning of every week.';

CREATE MATERIALIZED VIEW unspentio.balance_snapshot_for_over_366_days_materialized_view TO unspentio.balance_snapshot_for_over_366_days
(
    `user_id` Int32,
    `asset_id` String,
    `total_amount` Decimal(38, 19),
    `created_at` DateTime('UTC')
) AS
SELECT
    bs.user_id AS user_id,
    bs.asset_id AS asset_id,
    bs.total_amount AS total_amount,
    bs.created_at AS created_at
FROM unspentio.balance_snapshot_for_366_days AS bs
WHERE toDayOfWeek(bs.created_at, 0, 'UTC') = 1;

CREATE TABLE unspentio.base_balance_snapshot
(
    `user_id` Int32 CODEC(LZ4),
    `exchange_id` String DEFAULT '' CODEC(LZ4),
    `exchange_name` String DEFAULT '' CODEC(LZ4),
    `wallet_id` Int32 DEFAULT -2147483648 CODEC(LZ4),
    `wallet_address` String DEFAULT '' CODEC(LZ4),
    `wallet_label` String DEFAULT '' CODEC(LZ4),
    `asset_network` String DEFAULT '' CODEC(LZ4),
    `asset_chain_id` Int32 DEFAULT -2147483648 CODEC(LZ4),
    `asset_id` String CODEC(LZ4),
    `amount` Decimal(38, 19) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (user_id, created_at, asset_id, exchange_id, wallet_id, asset_network, asset_chain_id)
ORDER BY (user_id, created_at, asset_id, exchange_id, wallet_id, asset_network, asset_chain_id)
SETTINGS index_granularity = 8192
COMMENT 'Table to store base balances for all time.';

CREATE TABLE unspentio.migration_schema
(
    `version` String,
    `ts` DateTime DEFAULT now(),
    `applied` UInt8 DEFAULT 1
)
ENGINE = ReplacingMergeTree(ts)
PRIMARY KEY version
ORDER BY version
SETTINGS index_granularity = 8192;

CREATE TABLE unspentio.subportfolio
(
    `user_id` Int32 CODEC(LZ4),
    `id` String CODEC(LZ4),
    `name` String CODEC(LZ4),
    `description` Nullable(String) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4),
    `updated_at` DateTime('UTC') CODEC(LZ4),
    `is_deleted` UInt8 CODEC(LZ4),
    PROJECTION projection_1
    (
        SELECT
            user_id,
            id,
            name,
            description,
            created_at,
            updated_at,
            is_deleted
        ORDER BY
            user_id,
            name
    )
)
ENGINE = ReplacingMergeTree(updated_at, is_deleted)
PRIMARY KEY (user_id, id)
ORDER BY (user_id, id)
SETTINGS index_granularity = 32;

CREATE TABLE unspentio.subportfolio_asset
(
    `user_id` Int32 CODEC(LZ4),
    `subportfolio_id` String CODEC(LZ4),
    `exchange_id` String DEFAULT '' CODEC(LZ4),
    `exchange_name` String DEFAULT '' CODEC(LZ4),
    `wallet_id` Int32 DEFAULT -2147483648 CODEC(LZ4),
    `wallet_address` String DEFAULT '' CODEC(LZ4),
    `wallet_label` String DEFAULT '' CODEC(LZ4),
    `asset_network` String DEFAULT '' CODEC(LZ4),
    `asset_chain_id` Int32 DEFAULT -2147483648 CODEC(LZ4),
    `asset_id` String CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4),
    `updated_at` DateTime('UTC') CODEC(LZ4),
    `is_deleted` UInt8 CODEC(LZ4)
)
ENGINE = ReplacingMergeTree(updated_at, is_deleted)
PRIMARY KEY (user_id, subportfolio_id, exchange_id, wallet_id, asset_network, asset_chain_id, asset_id)
ORDER BY (user_id, subportfolio_id, exchange_id, wallet_id, asset_network, asset_chain_id, asset_id)
SETTINGS index_granularity = 1024;

CREATE TABLE unspentio.subportfolio_base_balance_snapshot
(
    `user_id` Int32 CODEC(LZ4),
    `subportfolio_id` String CODEC(LZ4),
    `exchange_id` String DEFAULT '' CODEC(LZ4),
    `exchange_name` String DEFAULT '' CODEC(LZ4),
    `wallet_id` Int32 DEFAULT -2147483648 CODEC(LZ4),
    `wallet_address` String DEFAULT '' CODEC(LZ4),
    `wallet_label` String DEFAULT '' CODEC(LZ4),
    `asset_network` String DEFAULT '' CODEC(LZ4),
    `asset_chain_id` Int32 DEFAULT -2147483648 CODEC(LZ4),
    `asset_id` String CODEC(LZ4),
    `amount` Decimal(38, 19) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (user_id, subportfolio_id, created_at, asset_id, exchange_id, wallet_id, asset_network, asset_chain_id)
ORDER BY (user_id, subportfolio_id, created_at, asset_id, exchange_id, wallet_id, asset_network, asset_chain_id)
SETTINGS index_granularity = 8192
COMMENT 'Table to store subportfolio base balances for all time.';

CREATE TABLE unspentio.subportfolio_base_balance_snapshot_for_120_days
(
    `user_id` Int32 CODEC(LZ4),
    `subportfolio_id` String CODEC(LZ4),
    `exchange_id` String DEFAULT '' CODEC(LZ4),
    `exchange_name` String DEFAULT '' CODEC(LZ4),
    `wallet_id` Int32 DEFAULT -2147483648 CODEC(LZ4),
    `wallet_address` String DEFAULT '' CODEC(LZ4),
    `wallet_label` String DEFAULT '' CODEC(LZ4),
    `asset_network` String DEFAULT '' CODEC(LZ4),
    `asset_chain_id` Int32 DEFAULT -2147483648 CODEC(LZ4),
    `asset_id` String CODEC(LZ4),
    `amount` Decimal(38, 19) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (user_id, subportfolio_id, created_at, asset_id, exchange_id, wallet_id, asset_network, asset_chain_id)
ORDER BY (user_id, subportfolio_id, created_at, asset_id, exchange_id, wallet_id, asset_network, asset_chain_id)
TTL created_at + toIntervalDay(122)
SETTINGS index_granularity = 8192
COMMENT 'Table to store subportfolio base balances for 120 days at the beginning of every day.';

CREATE MATERIALIZED VIEW unspentio.subportfolio_base_balance_snapshot_for_120_days_materialized_view TO unspentio.subportfolio_base_balance_snapshot_for_120_days
(
    `user_id` Int32,
    `subportfolio_id` String,
    `exchange_id` String,
    `exchange_name` String,
    `wallet_id` Int32,
    `wallet_address` String,
    `wallet_label` String,
    `asset_network` String,
    `asset_chain_id` Int32,
    `asset_id` String,
    `amount` Decimal(38, 19),
    `created_at` DateTime('UTC')
) AS
SELECT
    sbbs.user_id AS user_id,
    sbbs.subportfolio_id AS subportfolio_id,
    sbbs.exchange_id AS exchange_id,
    sbbs.exchange_name AS exchange_name,
    sbbs.wallet_id AS wallet_id,
    sbbs.wallet_address AS wallet_address,
    sbbs.wallet_label AS wallet_label,
    sbbs.asset_network AS asset_network,
    sbbs.asset_chain_id AS asset_chain_id,
    sbbs.asset_id AS asset_id,
    sbbs.amount AS amount,
    sbbs.created_at AS created_at
FROM unspentio.subportfolio_base_balance_snapshot_for_30_days AS sbbs
WHERE dateDiff('hour', toStartOfInterval(sbbs.created_at, toIntervalDay(1), 'UTC'), sbbs.created_at, 'UTC') = 0;

CREATE TABLE unspentio.subportfolio_base_balance_snapshot_for_30_days
(
    `user_id` Int32 CODEC(LZ4),
    `subportfolio_id` String CODEC(LZ4),
    `exchange_id` String DEFAULT '' CODEC(LZ4),
    `exchange_name` String DEFAULT '' CODEC(LZ4),
    `wallet_id` Int32 DEFAULT -2147483648 CODEC(LZ4),
    `wallet_address` String DEFAULT '' CODEC(LZ4),
    `wallet_label` String DEFAULT '' CODEC(LZ4),
    `asset_network` String DEFAULT '' CODEC(LZ4),
    `asset_chain_id` Int32 DEFAULT -2147483648 CODEC(LZ4),
    `asset_id` String CODEC(LZ4),
    `amount` Decimal(38, 19) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (user_id, subportfolio_id, created_at, asset_id, exchange_id, wallet_id, asset_network, asset_chain_id)
ORDER BY (user_id, subportfolio_id, created_at, asset_id, exchange_id, wallet_id, asset_network, asset_chain_id)
TTL created_at + toIntervalDay(32)
SETTINGS index_granularity = 8192
COMMENT 'Table to store subportfolio base balances for 30 days at the beginning of every 12 hours starting from 00:00.';

CREATE MATERIALIZED VIEW unspentio.subportfolio_base_balance_snapshot_for_30_days_materialized_view TO unspentio.subportfolio_base_balance_snapshot_for_30_days
(
    `user_id` Int32,
    `subportfolio_id` String,
    `exchange_id` String,
    `exchange_name` String,
    `wallet_id` Int32,
    `wallet_address` String,
    `wallet_label` String,
    `asset_network` String,
    `asset_chain_id` Int32,
    `asset_id` String,
    `amount` Decimal(38, 19),
    `created_at` DateTime('UTC')
) AS
SELECT
    sbbs.user_id AS user_id,
    sbbs.subportfolio_id AS subportfolio_id,
    sbbs.exchange_id AS exchange_id,
    sbbs.exchange_name AS exchange_name,
    sbbs.wallet_id AS wallet_id,
    sbbs.wallet_address AS wallet_address,
    sbbs.wallet_label AS wallet_label,
    sbbs.asset_network AS asset_network,
    sbbs.asset_chain_id AS asset_chain_id,
    sbbs.asset_id AS asset_id,
    sbbs.amount AS amount,
    multiIf((dateDiff('hour', toStartOfInterval(sbbs.created_at, toIntervalHour(12), 'UTC') AS ca, sbbs.created_at, 'UTC') AS dd) = 0, ca, dd = 11, ca + toIntervalHour(12), ca) AS created_at
FROM unspentio.subportfolio_base_balance_snapshot_for_7_days AS sbbs;

CREATE TABLE unspentio.subportfolio_base_balance_snapshot_for_366_days
(
    `user_id` Int32 CODEC(LZ4),
    `subportfolio_id` String CODEC(LZ4),
    `exchange_id` String DEFAULT '' CODEC(LZ4),
    `exchange_name` String DEFAULT '' CODEC(LZ4),
    `wallet_id` Int32 DEFAULT -2147483648 CODEC(LZ4),
    `wallet_address` String DEFAULT '' CODEC(LZ4),
    `wallet_label` String DEFAULT '' CODEC(LZ4),
    `asset_network` String DEFAULT '' CODEC(LZ4),
    `asset_chain_id` Int32 DEFAULT -2147483648 CODEC(LZ4),
    `asset_id` String CODEC(LZ4),
    `amount` Decimal(38, 19) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (user_id, subportfolio_id, created_at, asset_id, exchange_id, wallet_id, asset_network, asset_chain_id)
ORDER BY (user_id, subportfolio_id, created_at, asset_id, exchange_id, wallet_id, asset_network, asset_chain_id)
TTL created_at + toIntervalDay(368)
SETTINGS index_granularity = 8192
COMMENT 'Table to store subportfolio base balances for 366 days at the beginning of every half of week - monday and thursday.';

CREATE MATERIALIZED VIEW unspentio.subportfolio_base_balance_snapshot_for_366_days_materialized_view TO unspentio.subportfolio_base_balance_snapshot_for_366_days
(
    `user_id` Int32,
    `subportfolio_id` String,
    `exchange_id` String,
    `exchange_name` String,
    `wallet_id` Int32,
    `wallet_address` String,
    `wallet_label` String,
    `asset_network` String,
    `asset_chain_id` Int32,
    `asset_id` String,
    `amount` Decimal(38, 19),
    `created_at` DateTime('UTC')
) AS
SELECT
    sbbs.user_id AS user_id,
    sbbs.subportfolio_id AS subportfolio_id,
    sbbs.exchange_id AS exchange_id,
    sbbs.exchange_name AS exchange_name,
    sbbs.wallet_id AS wallet_id,
    sbbs.wallet_address AS wallet_address,
    sbbs.wallet_label AS wallet_label,
    sbbs.asset_network AS asset_network,
    sbbs.asset_chain_id AS asset_chain_id,
    sbbs.asset_id AS asset_id,
    sbbs.amount AS amount,
    sbbs.created_at AS created_at
FROM unspentio.subportfolio_base_balance_snapshot_for_120_days AS sbbs
WHERE ((toDayOfWeek(sbbs.created_at, 0, 'UTC') AS d) = 1) OR (d = 4);

CREATE TABLE unspentio.subportfolio_base_balance_snapshot_for_7_days
(
    `user_id` Int32 CODEC(LZ4),
    `subportfolio_id` String CODEC(LZ4),
    `exchange_id` String DEFAULT '' CODEC(LZ4),
    `exchange_name` String DEFAULT '' CODEC(LZ4),
    `wallet_id` Int32 DEFAULT -2147483648 CODEC(LZ4),
    `wallet_address` String DEFAULT '' CODEC(LZ4),
    `wallet_label` String DEFAULT '' CODEC(LZ4),
    `asset_network` String DEFAULT '' CODEC(LZ4),
    `asset_chain_id` Int32 DEFAULT -2147483648 CODEC(LZ4),
    `asset_id` String CODEC(LZ4),
    `amount` Decimal(38, 19) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (user_id, subportfolio_id, created_at, asset_id, exchange_id, wallet_id, asset_network, asset_chain_id)
ORDER BY (user_id, subportfolio_id, created_at, asset_id, exchange_id, wallet_id, asset_network, asset_chain_id)
TTL created_at + toIntervalDay(8)
SETTINGS index_granularity = 8192
COMMENT 'Table to store subportfolio base balances for 7 days at the beginning of every hour.';

CREATE MATERIALIZED VIEW unspentio.subportfolio_base_balance_snapshot_for_7_days_materialized_view TO unspentio.subportfolio_base_balance_snapshot_for_7_days
(
    `user_id` Int32,
    `subportfolio_id` String,
    `exchange_id` String,
    `exchange_name` String,
    `wallet_id` Int32,
    `wallet_address` String,
    `wallet_label` String,
    `asset_network` String,
    `asset_chain_id` Int32,
    `asset_id` String,
    `amount` Decimal(38, 19),
    `created_at` DateTime('UTC')
) AS
SELECT
    sbbs.user_id AS user_id,
    sbbs.subportfolio_id AS subportfolio_id,
    sbbs.exchange_id AS exchange_id,
    sbbs.exchange_name AS exchange_name,
    sbbs.wallet_id AS wallet_id,
    sbbs.wallet_address AS wallet_address,
    sbbs.wallet_label AS wallet_label,
    sbbs.asset_network AS asset_network,
    sbbs.asset_chain_id AS asset_chain_id,
    sbbs.asset_id AS asset_id,
    sbbs.amount AS amount,
    multiIf(dateDiff('minute', toStartOfInterval(sbbs.created_at, toIntervalHour(1), 'UTC') AS ca, sbbs.created_at, 'UTC') < 30, ca, ca + toIntervalHour(1)) AS created_at
FROM unspentio.subportfolio_base_balance_snapshot AS sbbs;

CREATE TABLE unspentio.subportfolio_base_balance_snapshot_for_over_366_days
(
    `user_id` Int32 CODEC(LZ4),
    `subportfolio_id` String CODEC(LZ4),
    `exchange_id` String DEFAULT '' CODEC(LZ4),
    `exchange_name` String DEFAULT '' CODEC(LZ4),
    `wallet_id` Int32 DEFAULT -2147483648 CODEC(LZ4),
    `wallet_address` String DEFAULT '' CODEC(LZ4),
    `wallet_label` String DEFAULT '' CODEC(LZ4),
    `asset_network` String DEFAULT '' CODEC(LZ4),
    `asset_chain_id` Int32 DEFAULT -2147483648 CODEC(LZ4),
    `asset_id` String CODEC(LZ4),
    `amount` Decimal(38, 19) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (user_id, subportfolio_id, created_at, asset_id, exchange_id, wallet_id, asset_network, asset_chain_id)
ORDER BY (user_id, subportfolio_id, created_at, asset_id, exchange_id, wallet_id, asset_network, asset_chain_id)
SETTINGS index_granularity = 8192
COMMENT 'Table to store subportfolio base balances for over 366 years at the beginning of every week.';

CREATE MATERIALIZED VIEW unspentio.subportfolio_base_balance_snapshot_for_over_366_days_materialized_view TO unspentio.subportfolio_base_balance_snapshot_for_over_366_days
(
    `user_id` Int32,
    `subportfolio_id` String,
    `exchange_id` String,
    `exchange_name` String,
    `wallet_id` Int32,
    `wallet_address` String,
    `wallet_label` String,
    `asset_network` String,
    `asset_chain_id` Int32,
    `asset_id` String,
    `amount` Decimal(38, 19),
    `created_at` DateTime('UTC')
) AS
SELECT
    sbbs.user_id AS user_id,
    sbbs.subportfolio_id AS subportfolio_id,
    sbbs.exchange_id AS exchange_id,
    sbbs.exchange_name AS exchange_name,
    sbbs.wallet_id AS wallet_id,
    sbbs.wallet_address AS wallet_address,
    sbbs.wallet_label AS wallet_label,
    sbbs.asset_network AS asset_network,
    sbbs.asset_chain_id AS asset_chain_id,
    sbbs.asset_id AS asset_id,
    sbbs.amount AS amount,
    sbbs.created_at AS created_at
FROM unspentio.subportfolio_base_balance_snapshot_for_366_days AS sbbs
WHERE toDayOfWeek(sbbs.created_at, 0, 'UTC') = 1;

CREATE MATERIALIZED VIEW unspentio.subportfolio_base_balance_snapshot_materialized_view TO unspentio.subportfolio_base_balance_snapshot
(
    `user_id` Int32,
    `subportfolio_id` String,
    `exchange_id` String,
    `exchange_name` String,
    `wallet_id` Int32,
    `wallet_address` String,
    `wallet_label` String,
    `asset_network` String,
    `asset_chain_id` Int32,
    `asset_id` String,
    `amount` Decimal(38, 19),
    `created_at` DateTime('UTC')
) AS
SELECT
    bbs.user_id AS user_id,
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
INNER JOIN unspentio.subportfolio_asset AS sa ON (bbs.user_id = sa.user_id) AND (bbs.exchange_id = sa.exchange_id) AND (bbs.wallet_id = sa.wallet_id) AND (bbs.asset_network = sa.asset_network) AND (bbs.asset_chain_id = sa.asset_chain_id) AND (bbs.asset_id = sa.asset_id)
SETTINGS final = 1;

CREATE TABLE unspentio.subportfolio_link
(
    `id` String CODEC(LZ4),
    `user_id` Int32 CODEC(LZ4),
    `subportfolio_id` String CODEC(LZ4),
    `is_active` Bool CODEC(LZ4),
    `description` Nullable(String) CODEC(LZ4),
    `created_at` DateTime('UTC') CODEC(LZ4),
    `updated_at` DateTime('UTC') CODEC(LZ4),
    `is_deleted` UInt8 CODEC(LZ4),
    PROJECTION projection_1
    (
        SELECT
            id,
            user_id,
            subportfolio_id,
            is_active,
            description,
            created_at,
            updated_at,
            is_deleted
        ORDER BY
            user_id,
            subportfolio_id
    )
)
ENGINE = ReplacingMergeTree(updated_at, is_deleted)
PRIMARY KEY id
ORDER BY id
SETTINGS index_granularity = 32;


--
-- Dbmate schema migrations
--

INSERT INTO migration_schema (version) VALUES
    ('00000000000001'),
    ('00000000000002'),
    ('00000000000003'),
    ('00000000000004'),
    ('00000000000005'),
    ('00000000000006'),
    ('00000000000007'),
    ('00000000000008'),
    ('00000000000009'),
    ('00000000000010'),
    ('00000000000011'),
    ('00000000000012'),
    ('00000000000013'),
    ('00000000000014'),
    ('00000000000015'),
    ('00000000000016'),
    ('00000000000017'),
    ('00000000000018'),
    ('00000000000019'),
    ('00000000000020'),
    ('00000000000021'),
    ('00000000000022'),
    ('00000000000023'),
    ('00000000000024'),
    ('00000000000025'),
    ('00000000000026'),
    ('00000000000027'),
    ('00000000000028'),
    ('00000000000029'),
    ('00000000000030'),
    ('00000000000031'),
    ('00000000000032'),
    ('00000000000033'),
    ('00000000000034'),
    ('00000000000035'),
    ('00000000000036'),
    ('00000000000037'),
    ('00000000000038'),
    ('00000000000039'),
    ('00000000000040'),
    ('00000000000041'),
    ('00000000000042'),
    ('00000000000043'),
    ('00000000000044'),
    ('00000000000045'),
    ('00000000000046'),
    ('00000000000047'),
    ('00000000000048');
