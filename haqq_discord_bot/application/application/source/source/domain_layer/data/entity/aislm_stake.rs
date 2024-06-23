use std::marker::PhantomData;

pub use self::Amount as AislmStake_Amount;
pub use self::CreatedAt as AislmStake_CreatedAt;
pub use self::EvmAddress as AislmStake_EvmAddress;
pub use self::Id as AislmStake_Id;
pub use super::address_verification_data::DiscordUserId as AislmStake_DiscordUserId;
pub use super::raffle::Id as RaffleParticipant_RaffleId;

pub struct Id;

pub struct Amount;

pub struct EvmAddress;

pub struct CreatedAt;

pub struct AislmStake {
    pub id: i64,
    _id: PhantomData<Id>,

    pub amount: String,
    _amount: PhantomData<Amount>,

    pub raffle_id: i64,
    _raffle_id: PhantomData<RaffleParticipant_RaffleId>,

    pub bech32_address: String,
    _bech32_address: PhantomData<EvmAddress>,

    pub discord_user_id: String,
    _discord_user_id: PhantomData<AislmStake_DiscordUserId>,

    pub created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}

impl AislmStake {
    pub fn new(
        id: i64,
        amount: String,
        raffle_id: i64,
        bech32_address: String,
        discord_user_id: String,
        created_at: i64,
    ) -> Self {
        return Self {
            id,
            _id: PhantomData,
            amount,
            _amount: PhantomData,
            raffle_id,
            _raffle_id: PhantomData,
            bech32_address,
            _bech32_address: PhantomData,
            discord_user_id,
            _discord_user_id: PhantomData,
            created_at,
            _created_at: PhantomData,
        };
    }
}

pub struct AislmStake_1 {
    pub id: i64,
    pub amount: String,
    pub bech32_address: String,
    pub discord_user_id: String,
    pub created_at: i64,
}

pub struct AislmStake_2 {
    pub id: i64,
    pub amount: String,
    pub discord_user_id: String,
}
