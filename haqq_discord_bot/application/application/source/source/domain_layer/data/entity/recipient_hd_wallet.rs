use std::marker::PhantomData;

pub use self::CreatedAt as RecipientHdWallet_CreatedAt;
pub use self::Id as RecipientHdWallet_Id;
pub use self::MnemonicDerevationPathIndex as RecipientHdWallet_MnemonicDerevationPathIndex;
pub use self::MnemonicPhrase as RecipientHdWallet_MnemonicPhrase;

pub struct Id;

pub struct MnemonicPhrase;

pub struct MnemonicDerevationPathIndex;

pub struct CreatedAt;

pub struct RecipientHdWallet {
    pub id: i64,
    _id: PhantomData<Id>,

    pub mnemonic_phrase: String,
    _mnemonic_phrase: PhantomData<MnemonicPhrase>,

    pub mnemonic_derevation_path_index: i32,
    _mnemonic_derevation_path_index: PhantomData<MnemonicDerevationPathIndex>,

    pub created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}

impl RecipientHdWallet {
    pub fn new(
        id: i64,
        mnemonic_phrase: String,
        mnemonic_derevation_path_index: i32,
        created_at: i64,
    ) -> Self {
        return Self {
            id,
            _id: PhantomData,
            mnemonic_phrase,
            _mnemonic_phrase: PhantomData,
            mnemonic_derevation_path_index,
            _mnemonic_derevation_path_index: PhantomData,
            created_at,
            _created_at: PhantomData,
        };
    }
}

pub struct RecipientHdWallet_1 {
    pub id: i64,
}

pub struct RecipientHdWallet_2 {
    pub mnemonic_phrase: String,
    pub mnemonic_derevation_path_index: i32,
}
