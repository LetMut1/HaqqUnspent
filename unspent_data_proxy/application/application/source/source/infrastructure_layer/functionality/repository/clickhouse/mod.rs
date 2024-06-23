pub mod asset_snapshot;
pub mod asset_snapshot_for_30_days;
pub mod asset_snapshot_for_366_days;
pub mod asset_snapshot_for_7_days;
pub mod balance_snapshot;
pub mod balance_snapshot_for_120_days;
pub mod balance_snapshot_for_30_days;
pub mod balance_snapshot_for_366_days;
pub mod balance_snapshot_for_7_days;
pub mod balance_snapshot_for_over_366_days;
pub mod base_balance_snapshot;
pub mod common;
pub mod subportfolio;
pub mod subportfolio_asset;
pub mod subportfolio_base_balance_snapshot;
pub mod subportfolio_base_balance_snapshot_for_120_days;
pub mod subportfolio_base_balance_snapshot_for_30_days;
pub mod subportfolio_base_balance_snapshot_for_366_days;
pub mod subportfolio_base_balance_snapshot_for_7_days;
pub mod subportfolio_base_balance_snapshot_for_over_366_days;
pub mod subportfolio_link;
pub mod subportfolio_trackable_wallet;

use self::{
    by::{
        By1,
        By3,
        By5,
        By6,
    },
    queried::{
        AssetSnapshotHistory,
        BalanceSnapshot,
    },
};
use crate::infrastructure_layer::{
    data::{
        auditor::Auditor,
        backtrace::BacktracePart,
        error::{
            Error,
            Other,
            Runtime,
        },
    },
    macro_rules::{
        query_pattern_find_asset_snapshot_history,
        query_pattern_find_balance_snapshot_history,
        query_pattern_find_subportfolio_base_balance_snapshot_history,
        query_pattern_lightweight_delete_subportfolio_base_balance_snapshot,
    },
};
use clickhouse::Client;
use std::marker::PhantomData;

pub struct ClickhouseRepository<E> {
    _entity: PhantomData<E>,
}

impl<E> ClickhouseRepository<E> {
    async fn find_balance_snapshot_history_<'a>(
        clickhouse_client: &'a Client,
        by_1: &'a By1<'_>,
        balance_snapshot_table_name: &'a str,
        asset_snapshot_table_name: &'a str,
    ) -> Result<Vec<BalanceSnapshot>, Auditor<Error>> {
        let query = format!(
            query_pattern_find_balance_snapshot_history!(),
            balance_snapshot_table_name,
            asset_snapshot_table_name,
        );

        let query_ = clickhouse_client.query(query.as_str()).bind(by_1.subtracted_quantity_of_hours).bind(by_1.user_id).bind(by_1.reference_asset_id);

        let balance_snapshot_registry = match query_.fetch_all::<BalanceSnapshot>().await {
            Ok(balance_snapshot_registry_) => balance_snapshot_registry_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        return Ok(balance_snapshot_registry);
    }

    async fn find_subportfolio_base_balance_snapshot_history_<'a>(
        clickhouse_client: &'a Client,
        by_6: &'a By6<'_>,
        subportfolio_base_balance_snapshot_table_name: &'a str,
        asset_snapshot_table_name: &'a str,
    ) -> Result<Vec<BalanceSnapshot>, Auditor<Error>> {
        let query = format!(
            query_pattern_find_subportfolio_base_balance_snapshot_history!(),
            subportfolio_base_balance_snapshot_table_name,
            asset_snapshot_table_name,
        );

        let query_ = clickhouse_client
            .query(query.as_str())
            .bind(by_6.subtracted_quantity_of_hours)
            .bind(by_6.user_id)
            .bind(by_6.subportfolio_id)
            .bind(by_6.reference_asset_id);

        let subportfolio_base_balance_snapshot_registry = match query_.fetch_all::<BalanceSnapshot>().await {
            Ok(subportfolio_base_balance_snapshot_registry_) => subportfolio_base_balance_snapshot_registry_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        return Ok(subportfolio_base_balance_snapshot_registry);
    }

    async fn find_asset_snapshot_history_<'a>(
        clickhouse_client: &'a Client,
        by_5: &'a By5<'_>,
        asset_snapshot_table_name: &'a str,
    ) -> Result<Vec<AssetSnapshotHistory>, Auditor<Error>> {
        let query = format!(
            query_pattern_find_asset_snapshot_history!(),
            asset_snapshot_table_name,
        );

        let query_ = clickhouse_client.query(query.as_str()).bind(by_5.subtracted_quantity_of_hours).bind(by_5.asset_id_registry);

        let asset_snapshot_history_registry = match query_.fetch_all::<AssetSnapshotHistory>().await {
            Ok(asset_snapshot_history_registry_) => asset_snapshot_history_registry_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        return Ok(asset_snapshot_history_registry);
    }

    async fn lightweight_delete_subportfolio_base_balance_snapshot_<'a>(
        clickhouse_client: &'a Client,
        by_3: &'a By3<'_>,
        subportfolio_base_balance_snapshot_table_name: &'a str,
    ) -> Result<(), Auditor<Error>> {
        let query = format!(
            query_pattern_lightweight_delete_subportfolio_base_balance_snapshot!(),
            subportfolio_base_balance_snapshot_table_name,
        );

        let query_ = clickhouse_client.query(query.as_str()).bind(by_3.user_id).bind(by_3.subportfolio_id);

        if let Err(error) = query_.execute().await {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        return Ok(());
    }
}

pub mod queried {
    use crate::domain_layer::data::entity::_remote::Asset_Id;
    use clickhouse::{
        self,
        Row,
    };
    use serde::{
        Deserialize,
        Serialize,
    };

    #[derive(Row, Serialize, Deserialize)]
    pub struct BalanceSnapshot {
        pub btc_value: String,
        pub fiat_value: String,
        pub timestamp: u32,
    }

    #[derive(Row, Serialize, Deserialize)]
    pub struct AssetSnapshotHistory {
        pub asset_id: Asset_Id,
        pub asset_price_usd_registry: Vec<AssetData>,
    }

    #[derive(Row, Serialize, Deserialize)]
    pub struct AssetData {
        pub price_usd: String,
        pub timestamp: u32,
    }
}

pub mod by {
    pub struct By1<'a> {
        pub user_id: i32,
        pub reference_asset_id: &'a str,
        pub subtracted_quantity_of_hours: i64,
    }

    pub struct By2 {
        pub user_id: i32,
    }

    pub struct By3<'a> {
        pub user_id: i32,
        pub subportfolio_id: &'a str,
    }

    pub struct By4<'a> {
        pub user_id: i32,
        pub subportfolio_id: &'a str,
        pub asset_chain_id_registry: &'a [i32],
    }

    pub struct By5<'a> {
        pub asset_id_registry: &'a [String],
        pub subtracted_quantity_of_hours: i64,
    }

    pub struct By6<'a> {
        pub user_id: i32,
        pub subportfolio_id: &'a str,
        pub reference_asset_id: &'a str,
        pub subtracted_quantity_of_hours: i64,
    }

    pub struct By7<'a> {
        pub user_id: i32,
        pub subportfolio_name: &'a str,
    }

    pub struct By8<'a> {
        pub subportfolio_link_id: &'a str,
        pub user_id: i32,
    }

    pub struct By9<'a> {
        pub subportfolio_link_id: &'a str,
    }

    pub struct By10<'a> {
        pub asset_id_registry: &'a [String],
    }

    pub struct By11 {
        pub user_id: Option<i32>,
    }
}
