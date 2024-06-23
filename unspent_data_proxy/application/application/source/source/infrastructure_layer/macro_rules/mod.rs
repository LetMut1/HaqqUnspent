macro_rules! r#enum {
    ($visability:vis enum $enum_name:ident { $($enum:ident :: $enum_variant:ident $({ $($enum_variant_field:ident : $enum_variant_field_type:ty),* $(,)? })?),* $(,)? }) => {
        const _: () = {
            $(
                let _ = |r#enum: $enum| -> () {
                    match r#enum {
                        $enum :: $enum_variant => (),
                        _ => (),
                    }
                };
            )*

            ()
        };

        #[derive(serde::Serialize)]
        $visability enum $enum_name {
            $($enum_variant $({ $($enum_variant_field: $enum_variant_field_type,)* })?,)*
        }
    };
}

macro_rules! query_pattern_find_balance_snapshot_history {
    () => {
        "\
        WITH \
            now('UTC') AS now, \
            (now - INTERVAL ? HOUR) AS date_from, \
            toStartOfInterval(now + INTERVAL 1 HOUR, INTERVAL 1 HOUR, 'UTC') AS date_before \
        SELECT \
            cast( \
                round( \
                    sum( \
                        CASE \
                            WHEN as.price_btc IS NOT NULL \
                            THEN multiplyDecimal(bs.total_amount, as.price_btc) \
                            ELSE divideDecimal(multiplyDecimal(bs.total_amount, as.price_usd), asl_2.price_usd) \
                        END \
                    ),  \
                    4 \
                ), \
                'String' \
            ) AS btc_value, \
            cast( \
                round( \
                    sum( \
                        divideDecimal(multiplyDecimal(bs.total_amount, as.price_usd), asl_1.price_usd) \
                    ), \
                    2 \
                ), \
                'String' \
            ) AS fiat_value, \
            toUnixTimestamp(bs.created_at, 'UTC') AS t \
        FROM \
            ( \
                SELECT \
                    bs.created_at AS created_at, \
                    bs.asset_id AS asset_id, \
                    bs.total_amount AS total_amount \
                FROM \
                    {} AS bs \
                WHERE \
                    bs.user_id = ? \
                    AND bs.created_at >= date_from \
                    AND bs.created_at <= date_before \
                LIMIT 1 BY \
                    bs.user_id, \
                    bs.created_at, \
                    bs.asset_id \
            ) AS bs \
            INNER JOIN ( \
                SELECT \
                    as.created_at AS created_at, \
                    as.asset_id AS asset_id, \
                    as.price_usd AS price_usd, \
                    as.price_btc AS price_btc \
                FROM \
                    {} AS as \
                WHERE \
                    as.created_at >= date_from \
                    AND as.created_at <= date_before \
                LIMIT 1 BY \
                    as.asset_id, \
                    as.created_at \
            ) AS as \
            ON \
                bs.created_at = as.created_at \
                AND bs.asset_id = as.asset_id \
            CROSS JOIN ( \
                SELECT \
                    asl.price_usd AS price_usd, \
                    asl.price_btc AS price_btc \
                FROM \
                    unspentio.asset_snapshot_last AS asl \
                WHERE \
                    asl.asset_id = ? \
                    AND asl.price_usd > 0 \
                LIMIT 1 \
            ) AS asl_1 \
            CROSS JOIN ( \
                SELECT \
                    asl.price_usd AS price_usd \
                FROM \
                    unspentio.asset_snapshot_last AS asl \
                WHERE \
                    asl.asset_id = 'bitcoin' \
                    AND asl.price_usd > 0 \
                LIMIT 1 \
            ) AS asl_2 \
        GROUP BY \
            bs.created_at \
        ORDER BY \
            bs.created_at ASC \
        SETTINGS \
            optimize_read_in_order = 1, \
            optimize_aggregation_in_order = 1, \
            optimize_move_to_prewhere = 0, \
            join_use_nulls = 1"
    };
}

macro_rules! query_pattern_find_subportfolio_base_balance_snapshot_history {
    () => {
        "\
        WITH \
            now('UTC') AS now, \
            (now - INTERVAL ? HOUR) AS date_from, \
            toStartOfInterval(now + INTERVAL 1 HOUR, INTERVAL 1 HOUR, 'UTC') AS date_before \
        SELECT \
            cast( \
                round( \
                    sum( \
                        CASE \
                            WHEN as.price_btc IS NOT NULL \
                            THEN multiplyDecimal(sbbs.total_amount, as.price_btc) \
                            ELSE divideDecimal(multiplyDecimal(sbbs.total_amount, as.price_usd), asl_2.price_usd) \
                        END \
                    ), \
                    4 \
                ), \
                'String' \
            ) AS btc_value, \
            cast( \
                round( \
                    sum( \
                        divideDecimal(multiplyDecimal(sbbs.total_amount, as.price_usd), asl_1.price_usd) \
                    ), \
                    2 \
                ), \
                'String' \
            ) AS fiat_value, \
            toUnixTimestamp(sbbs.created_at, 'UTC') AS t \
        FROM \
            ( \
                SELECT \
                    sbbs.created_at AS created_at, \
                    sbbs.asset_id AS asset_id, \
                    sum(sbbs.amount) AS total_amount \
                FROM \
                    {} AS sbbs \
                WHERE \
                    sbbs.user_id = ? \
                    AND sbbs.subportfolio_id = ? \
                    AND sbbs.created_at >= date_from \
                    AND sbbs.created_at <= date_before \
                GROUP BY \
                    sbbs.user_id, \
                    sbbs.subportfolio_id, \
                    sbbs.created_at, \
                    sbbs.asset_id \
                LIMIT 1 BY \
                    sbbs.user_id, \
                    sbbs.subportfolio_id, \
                    sbbs.created_at, \
                    sbbs.asset_id \
            ) AS sbbs \
            INNER JOIN ( \
                SELECT \
                    as.created_at AS created_at, \
                    as.asset_id AS asset_id, \
                    as.price_usd AS price_usd, \
                    as.price_btc AS price_btc \
                FROM \
                    {} AS as \
                WHERE \
                    as.created_at >= date_from \
                    AND as.created_at <= date_before \
                LIMIT 1 BY \
                    as.asset_id, \
                    as.created_at \
            ) AS as \
            ON \
                sbbs.created_at = as.created_at \
                AND sbbs.asset_id = as.asset_id \
            CROSS JOIN ( \
                SELECT \
                    asl.price_usd AS price_usd, \
                    asl.price_btc AS price_btc \
                FROM \
                    unspentio.asset_snapshot_last AS asl \
                WHERE \
                    asl.asset_id = ? \
                    AND asl.price_usd > 0 \
                LIMIT 1 \
            ) AS asl_1 \
            CROSS JOIN ( \
                SELECT \
                    asl.price_usd AS price_usd \
                FROM \
                    unspentio.asset_snapshot_last AS asl \
                WHERE \
                    asl.asset_id = 'bitcoin' \
                    AND asl.price_usd > 0 \
                LIMIT 1 \
            ) AS asl_2 \
        GROUP BY \
            sbbs.created_at \
        ORDER BY \
            sbbs.created_at ASC \
        SETTINGS \
            optimize_read_in_order = 1, \
            optimize_aggregation_in_order = 1, \
            optimize_move_to_prewhere = 0"
    };
}

macro_rules! query_pattern_find_asset_snapshot_history {
    () => {
        "\
        WITH \
            now('UTC') AS now, \
            (now - INTERVAL ? HOUR) AS date_from, \
            toStartOfInterval(now + INTERVAL 1 HOUR, INTERVAL 1 HOUR, 'UTC') AS date_before \
        SELECT \
            r.ai AS ai, \
            groupArray((r.pu, r.t)) AS a \
        FROM \
            ( \
                SELECT \
                    as_.asset_id AS ai, \
                    cast(as_.price_usd, 'String') AS pu, \
                    toUnixTimestamp(as_.created_at, 'UTC') AS t \
                FROM \
                    {} AS as_ \
                WHERE \
                    as_.asset_id IN ? \
                    AND as_.created_at >= date_from \
                    AND as_.created_at <= date_before \
                ORDER BY \
                    as_.asset_id ASC, \
                    as_.created_at ASC \
                LIMIT 1 BY \
                    as_.asset_id, \
                    as_.created_at \
            ) AS r \
        GROUP BY \
            r.ai \
        SETTINGS \
            optimize_read_in_order = 1, \
            optimize_move_to_prewhere = 0"
    };
}

macro_rules! query_pattern_lightweight_delete_subportfolio_base_balance_snapshot {
    () => {
        "\
        DELETE FROM {} \
        WHERE \
            user_id = ? \
            AND subportfolio_id = ? \
        SETTINGS \
            optimize_move_to_prewhere = 0"
    };
}

pub(crate) use query_pattern_find_asset_snapshot_history;
pub(crate) use query_pattern_find_balance_snapshot_history;
pub(crate) use query_pattern_find_subportfolio_base_balance_snapshot_history;
pub(crate) use query_pattern_lightweight_delete_subportfolio_base_balance_snapshot;
pub(crate) use r#enum;
