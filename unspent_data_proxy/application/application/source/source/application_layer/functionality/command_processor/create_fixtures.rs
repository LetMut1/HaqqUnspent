use super::CommandProcessor;
pub use crate::infrastructure_layer::data::control_type::CreateFixtures;
use crate::{
    domain_layer::data::entity::{
        _remote::{
            Asset_Id,
            User_Id,
        },
        asset_snapshot::{
            AssetSnapshot,
            AssetSnapshot_CreatedAt,
            AssetSnapshot_PriceBtc,
            AssetSnapshot_PriceUsd,
        },
        balance_snapshot::{
            BalanceSnapshot,
            BalanceSnapshot_CreatedAt,
            BalanceSnapshot_TotalAmount,
        },
    },
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            environment_configuration::ENVIRONMENT_CONFIGURATION,
            error::{
                Error,
                Other,
                Runtime,
            },
        },
        functionality::{
            repository::clickhouse::ClickhouseRepository,
            service::{
                converter::{
                    Convert,
                    Converter,
                },
                creator::{
                    clickhouse_client::ClickhouseClient,
                    Creator,
                },
            },
        },
    },
};
use chrono::{
    Duration,
    Utc,
};
use rand::{
    thread_rng,
    Rng,
};
use tokio::runtime::Builder;

impl CommandProcessor<CreateFixtures> {
    const BALANCE_SNAPSHOT_QUANTITY: u64 = 24 * 366;
    const BALANCE_SNAPSHOT_CREATED_AT_DIFFERENCE_SECONDS: i64 = 60 * 60 * 24 * 366 * 3;
    const USER_QUANTITY: i32 = 100;
    const BITCOIN_ASSET_ID: &'static str = "bitcoin";

    pub fn process() -> Result<(), Auditor<Error>> {
        let runtime = match Builder::new_current_thread().enable_all().build() {
            Ok(runtime_) => runtime_,
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

        if let Err(mut error_auditor) = runtime.block_on(Self::create_fixtures()) {
            error_auditor.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                ),
            );

            return Err(error_auditor);
        }

        return Ok(());
    }

    pub async fn create_fixtures() -> Result<(), Auditor<Error>> {
        let clickhouse_client = match Creator::<ClickhouseClient>::create(
            ENVIRONMENT_CONFIGURATION.resource.clickhouse.url.0,
            ENVIRONMENT_CONFIGURATION.resource.clickhouse.user.0,
            ENVIRONMENT_CONFIGURATION.resource.clickhouse.password.0,
            ENVIRONMENT_CONFIGURATION.resource.clickhouse.database.0,
        ) {
            Ok(clickhouse_client_) => clickhouse_client_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error_auditor);
            }
        };

        let asset_snapshot_price_usd = AssetSnapshot_PriceUsd("0.001".to_string());

        let balance_snapshot_total_amount = BalanceSnapshot_TotalAmount("1.00000000005".to_string());

        let mut balance_snapshot_registry: Vec<BalanceSnapshot> = vec![];

        let mut asset_snapshot_registry: Vec<AssetSnapshot> = vec![];

        let now = Utc::now();

        let asset_id_range = 'a'..='z';

        let mut asset_id_registry = asset_id_range
            .into_iter()
            .map(
                |character: char| -> String {
                    return character.to_string();
                },
            )
            .collect::<Vec<String>>();

        asset_id_registry.push(Self::BITCOIN_ASSET_ID.to_string());

        '_a: for user_id in 1..=Self::USER_QUANTITY {
            '_b: for asset_id in asset_id_registry.iter() {
                let asset_id_ = Asset_Id(asset_id.clone());

                '_c: for _ in 1..=Self::BALANCE_SNAPSHOT_QUANTITY {
                    let created_at = match now.checked_sub_signed(Duration::seconds(thread_rng().gen_range::<i64, _>(1..=Self::BALANCE_SNAPSHOT_CREATED_AT_DIFFERENCE_SECONDS))) {
                        Some(created_at_) => created_at_,
                        None => {
                            return Err(
                                Auditor::<Error>::new(
                                    Error::Logic {
                                        message: "The value is too high.",
                                    },
                                    BacktracePart::new(
                                        line!(),
                                        file!(),
                                    ),
                                ),
                            );
                        }
                    };

                    let created_at_ = match <Converter as Convert<i64, u32>>::convert(created_at.timestamp()) {
                        Ok(created_at__) => created_at__,
                        Err(mut error_auditor) => {
                            error_auditor.add_backtrace_part(
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                ),
                            );

                            return Err(error_auditor);
                        }
                    };

                    balance_snapshot_registry.push(
                        BalanceSnapshot {
                            user_id: User_Id(user_id),
                            asset_id: asset_id_.clone(),
                            total_amount: balance_snapshot_total_amount.clone(),
                            created_at: BalanceSnapshot_CreatedAt(created_at_),
                        },
                    );

                    let asset_snapshot_price_btc = if thread_rng().gen_range::<u8, _>(1..=3) == 1 {
                        Some(AssetSnapshot_PriceBtc("0.1".to_string()))
                    } else {
                        None
                    };

                    asset_snapshot_registry.push(
                        AssetSnapshot {
                            asset_id: asset_id_.clone(),
                            price_usd: asset_snapshot_price_usd.clone(),
                            price_btc: asset_snapshot_price_btc,
                            created_at: AssetSnapshot_CreatedAt(created_at_),
                        },
                    );
                }

                if let Err(mut error_auditor) = ClickhouseRepository::<BalanceSnapshot>::create(
                    &clickhouse_client,
                    balance_snapshot_registry.as_slice(),
                )
                .await
                {
                    error_auditor.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    );

                    return Err(error_auditor);
                }

                balance_snapshot_registry.clear();

                if let Err(mut error_auditor) = ClickhouseRepository::<AssetSnapshot>::create(
                    &clickhouse_client,
                    asset_snapshot_registry.as_slice(),
                )
                .await
                {
                    error_auditor.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    );

                    return Err(error_auditor);
                }

                asset_snapshot_registry.clear();
            }
        }

        return Ok(());
    }
}
