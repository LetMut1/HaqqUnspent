use std::marker::PhantomData;

pub use self::CreatedAt as VerifiedBech32Address_CreatdAt;
pub use self::Value as VerifiedBech32Address_Value;
pub use super::address_verification_data::DiscordUserId as VerifiedBech32Address_DiscordUserId;

pub struct Value;

pub struct CreatedAt;

pub struct VerifiedBech32Address {
    pub value: String,
    _value: PhantomData<Value>,

    pub discord_user_id: String,
    _discord_user_id: PhantomData<VerifiedBech32Address_DiscordUserId>,

    pub created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}

impl VerifiedBech32Address {
    pub fn new(
        value: String,
        discord_user_id: String,
        created_at: i64,
    ) -> Self {
        return Self {
            value,
            _value: PhantomData,
            discord_user_id,
            _discord_user_id: PhantomData,
            created_at,
            _created_at: PhantomData,
        };
    }
}

pub struct VerifiedBech32Address_1 {
    pub value: String,
}

pub struct VerifiedBech32Address_2 {
    pub discord_user_id: String,
}

pub struct VerifiedBech32Address_3 {
    pub value: String,
    pub discord_user_id: String,
}
