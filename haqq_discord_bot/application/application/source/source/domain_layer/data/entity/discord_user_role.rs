use std::marker::PhantomData;

pub use self::StakersClubMember as DiscordUserRole_StakersClubMember;
pub use self::UpdatedAt as DiscordUserRole_UpdatedAt;
pub use self::WalletVerified as DiscordUserRole_WalletVerified;
pub use super::address_verification_data::DiscordUserId as VerifiedEvmAddress_DiscordUserId;

pub struct WalletVerified;

pub struct StakersClubMember;

pub struct UpdatedAt;

pub struct DiscordUserRole {
    pub discord_user_id: String,
    _discord_user_id: PhantomData<VerifiedEvmAddress_DiscordUserId>,

    pub wallet_verified: bool,
    _wallet_verified: PhantomData<WalletVerified>,

    pub stakers_club_member: bool,
    _stakers_club_member: PhantomData<StakersClubMember>,

    pub updated_at: i64,
    _updated_at: PhantomData<UpdatedAt>,
}

impl DiscordUserRole {
    pub fn new(
        discord_user_id: String,
        wallet_verified: bool,
        stakers_club_member: bool,
        updated_at: i64,
    ) -> Self {
        return Self {
            discord_user_id,
            _discord_user_id: PhantomData,
            wallet_verified,
            _wallet_verified: PhantomData,
            stakers_club_member,
            _stakers_club_member: PhantomData,
            updated_at,
            _updated_at: PhantomData,
        };
    }
}

pub struct DiscordUserRole_1 {
    pub wallet_verified: bool,
    pub stakers_club_member: bool,
}

pub struct DiscordUserRole_2 {
    pub wallet_verified: bool,
    pub stakers_club_member: bool,
    pub updated_at: i64,
}

pub struct DiscordUserRole_3 {
    pub wallet_verified: bool,
    pub updated_at: i64,
}

pub struct DiscordUserRole_4 {
    pub stakers_club_member: bool,
    pub updated_at: i64,
}
