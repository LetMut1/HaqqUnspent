use super::ClickhouseRepository;
use crate::{
    domain_layer::data::entity::base_balance_snapshot::BaseBalanceSnapshot,
    infrastructure_layer::data::{
        auditor::Auditor,
        backtrace::BacktracePart,
        error::{
            Error,
            Other,
            Runtime,
        },
    },
};
use clickhouse::{
    self,
    Client,
};

impl ClickhouseRepository<BaseBalanceSnapshot> {
    pub async fn create<'a>(clickhouse_client: &'a Client, base_balance_snapshot_registry: &'a [BaseBalanceSnapshot]) -> Result<(), Auditor<Error>> {
        if base_balance_snapshot_registry.is_empty() {
            return Ok(());
        }

        let mut query = "\
            INSERT INTO unspentio.base_balance_snapshot \
            ( \
                user_id, \
                exchange_id, \
                exchange_name, \
                wallet_id, \
                wallet_address, \
                wallet_label, \
                asset_network, \
                asset_chain_id, \
                asset_id, \
                amount, \
                created_at \
            ) \
            VALUES"
            .to_string();

        for _ in base_balance_snapshot_registry {
            query = format!(
                "{} ( \
                    ?, \
                    ?, \
                    ?, \
                    ?, \
                    ?, \
                    ?, \
                    ?, \
                    ?, \
                    ?, \
                    toDecimal128(?, 19), \
                    fromUnixTimestamp(?)
                ),",
                query,
            );
        }

        let mut query_ = clickhouse_client.query(query.as_str());

        for base_balance_snapshot in base_balance_snapshot_registry {
            query_ = query_
                .bind(base_balance_snapshot.user_id)
                .bind(&base_balance_snapshot.exchange_id)
                .bind(&base_balance_snapshot.exchange_name)
                .bind(base_balance_snapshot.wallet_id)
                .bind(&base_balance_snapshot.wallet_address)
                .bind(&base_balance_snapshot.wallet_label)
                .bind(&base_balance_snapshot.asset_network)
                .bind(base_balance_snapshot.asset_chain_id)
                .bind(&base_balance_snapshot.asset_id)
                .bind(&base_balance_snapshot.amount)
                .bind(base_balance_snapshot.created_at);
        }

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
