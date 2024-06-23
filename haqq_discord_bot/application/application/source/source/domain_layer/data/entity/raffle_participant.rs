pub use super::address_verification_data::DiscordUserId as VerifiedEvmAddress_DiscordUserId;
pub use super::raffle::Id as RaffleParticipant_RaffleId;
use std::marker::PhantomData;

pub use self::CreatedAt as RaffleParticipant_CreatedAt;

pub struct CreatedAt;

pub struct RaffleParticipant {
    pub raffle_id: i64,
    _raffle_id: PhantomData<RaffleParticipant_RaffleId>,

    pub discord_user_id: String,
    _discord_user_id: PhantomData<VerifiedEvmAddress_DiscordUserId>,

    pub created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}

impl RaffleParticipant {
    pub fn new(
        raffle_id: i64,
        discord_user_id: String,
        created_at: i64,
    ) -> Self {
        return Self {
            raffle_id,
            _raffle_id: PhantomData,
            discord_user_id,
            _discord_user_id: PhantomData,
            created_at,
            _created_at: PhantomData,
        };
    }
}
