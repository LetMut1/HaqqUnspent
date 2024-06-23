use std::marker::PhantomData;

pub use self::CreatedAt as AddressVerificationData_CreatedAt;
pub use self::DiscordUserId as AddressVerificationData_DiscordUserId;
pub use self::ExpectedTokenQuantity as AddressVerificationData_ExpectedTokenQuantity;
pub use self::ExpiredAt as AddressVerificationData_ExpiredAt;
pub use self::Recipientbech32Address as AddressVerificationData_RecipientBech32Address;

pub struct DiscordUserId;

pub struct Recipientbech32Address;

pub struct ExpectedTokenQuantity;

pub struct CreatedAt;

pub struct ExpiredAt;

pub struct AddressVerificationData {
    pub discord_user_id: String,
    _discord_user_id: PhantomData<DiscordUserId>,

    pub recipient_bech32_address: String,
    _recipient_bech32_address: PhantomData<Recipientbech32Address>,

    pub expected_token_quantity: String,
    _expected_token_quantity: PhantomData<ExpectedTokenQuantity>,

    pub created_at: i64,
    _created_at: PhantomData<CreatedAt>,

    pub expired_at: i64,
    _expired_at: PhantomData<ExpiredAt>,
}

impl AddressVerificationData {
    pub fn new(
        discord_user_id: String,
        recipient_bech32_address: String,
        expected_token_quantity: String,
        created_at: i64,
        expired_at: i64,
    ) -> Self {
        return Self {
            discord_user_id,
            _discord_user_id: PhantomData,
            recipient_bech32_address,
            _recipient_bech32_address: PhantomData,
            expected_token_quantity,
            _expected_token_quantity: PhantomData,
            created_at,
            _created_at: PhantomData,
            expired_at,
            _expired_at: PhantomData,
        };
    }
}
