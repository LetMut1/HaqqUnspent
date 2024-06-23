pub use super::address_verification_data::DiscordUserId as VerifiedEvmAddress_DiscordUserId;
pub use super::raffle::Id as RaffleWinners_RaffleId;
use std::marker::PhantomData;

pub use self::Bech32Address as RaffleWinner_Bech32Address;
pub use self::CreatedAt as RaffleWinners_CreatedAt;

pub struct Bech32Address;

pub struct CreatedAt;

pub struct RaffleWinner {
    pub raffle_id: i64,
    _raffle_id: PhantomData<RaffleWinners_RaffleId>,

    pub discord_user_id: String,
    _discord_user_id: PhantomData<VerifiedEvmAddress_DiscordUserId>,

    pub bech32_address: String,
    _bech32_address: PhantomData<Bech32Address>,

    pub created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}

impl RaffleWinner {
    pub fn new(
        raffle_id: i64,
        discord_user_id: String,
        bech32_address: String,
        created_at: i64,
    ) -> Self {
        return Self {
            raffle_id,
            _raffle_id: PhantomData,
            discord_user_id,
            _discord_user_id: PhantomData,
            bech32_address,
            _bech32_address: PhantomData,
            created_at,
            _created_at: PhantomData,
        };
    }
}
