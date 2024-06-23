pub use crate::infrastructure_layer::data::control_type::AssetSnapshot___Create;
use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::{
        data::entity::asset_snapshot::{
            AssetSnapshot,
            AssetSnapshot_PriceBtc,
            AssetSnapshot_PriceUsd,
        },
        functionality::service::validator::Validator,
    },
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            error::Error,
            invalid_argument::InvalidArgument,
            invalid_argument_result::InvalidArgumentResult,
            void::Void,
        },
        functionality::{
            repository::clickhouse::ClickhouseRepository,
            service::validator::{
                server_access_token::ServerAccessToken,
                Validator as Validator_,
            },
        },
    },
};
use clickhouse::Client;
use serde::Deserialize;

impl ActionProcessor<AssetSnapshot___Create> {
    pub async fn process(incoming: Option<Incoming>, clickhouse_client: Client) -> Result<InvalidArgumentResult<UnifiedReport<Void, Void>>, Auditor<Error>> {
        let incoming_ = match incoming {
            Some(incoming__) => incoming__,
            None => {
                return Err(
                    Auditor::<Error>::new(
                        Error::create_incoming_invalid_state(),
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        if !Validator_::<ServerAccessToken>::is_valid(&incoming_.server_access_token) {
            return Ok(
                InvalidArgumentResult::InvalidArgumentAuditor {
                    invalid_argument_auditor: Auditor::<InvalidArgument>::new(
                        InvalidArgument::new(),
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                },
            );
        }

        let mut asset_snapshot_registry: Vec<AssetSnapshot> = vec![];

        'a: for asset_snapshot in incoming_.asset_snapshot_registry.into_iter() {
            let is_valid_asset_snapshot_price_usd = match Validator::<AssetSnapshot_PriceUsd>::is_valid(&asset_snapshot.price_usd) {
                Ok(is_valid_asset_snapshot_price_usd_) => is_valid_asset_snapshot_price_usd_,
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

            if !is_valid_asset_snapshot_price_usd {
                continue 'a;
            }

            if let Some(ref price_btc) = asset_snapshot.price_btc {
                let is_valid_asset_snapshot_price_btc = match Validator::<AssetSnapshot_PriceBtc>::is_valid(price_btc) {
                    Ok(is_valid_asset_snapshot_price_btc_) => is_valid_asset_snapshot_price_btc_,
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

                if !is_valid_asset_snapshot_price_btc {
                    continue 'a;
                }
            }

            asset_snapshot_registry.push(asset_snapshot);
        }

        if !asset_snapshot_registry.is_empty() {
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
            };
        }

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::empty(),
            },
        );
    }
}

#[derive(Deserialize)]
pub struct Incoming {
    server_access_token: ServerAccessToken,
    asset_snapshot_registry: Vec<AssetSnapshot>,
}
