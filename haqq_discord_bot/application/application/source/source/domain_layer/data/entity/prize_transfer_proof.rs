pub use super::address_verification_data::DiscordUserId as VerifiedEvmAddress_DiscordUserId;
pub use super::raffle::Id as RaffleWinners_RaffleId;
use std::marker::PhantomData;

pub use self::EvmTransactionHash as RaffleWinner_EvmTransactionHash;
pub use self::CreatedAt as RaffleWinners_CreatedAt;

pub struct EvmTransactionHash;

pub struct CreatedAt;

pub struct PrizeTransferProof {
    pub raffle_id: i64,
    _raffle_id: PhantomData<RaffleWinners_RaffleId>,

    pub discord_user_id: String,
    _discord_user_id: PhantomData<VerifiedEvmAddress_DiscordUserId>,

    pub evm_transaction_hash: String,
    _evm_transaction_hash: PhantomData<EvmTransactionHash>,

    pub created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}

impl PrizeTransferProof {
    pub fn new(
        raffle_id: i64,
        discord_user_id: String,
        evm_transaction_hash: String,
        created_at: i64,
    ) -> Self {
        return Self {
            raffle_id,
            _raffle_id: PhantomData,
            discord_user_id,
            _discord_user_id: PhantomData,
            evm_transaction_hash,
            _evm_transaction_hash: PhantomData,
            created_at,
            _created_at: PhantomData,
        };
    }
}
