use std::marker::PhantomData;

pub use self::Bech32Address as VerifiedAddressBlacklist_Bech32Address;
pub use self::CreatedAt as VerifiedAddressBlacklist_CreatedAt;

pub struct Bech32Address;

pub struct CreatedAt;

pub struct VerifiedAddressBlacklist {
    pub bech32_address: String,
    _bech32_address: PhantomData<Bech32Address>,

    pub created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}

impl VerifiedAddressBlacklist {
    pub fn new(
        verified_bech32_address: String,
        created_at: i64,
    ) -> Self {
        return Self {
            bech32_address: verified_bech32_address,
            _bech32_address: PhantomData,
            created_at,
            _created_at: PhantomData,
        };
    }
}
