use std::marker::PhantomData;

pub use self::AesKey as Raffle_AesKey;
pub use self::CreatedAt as Raffle_CreatedAt;
pub use self::ExpiredAt as Raffle_ExpiredAt;
pub use self::Id as Raffle_Id;
pub use self::Status as Raffle_Status;
pub use self::IslmPrizeAmount as Raffle_IslmPrizeAmount;
pub use self::Seed as Raffle_Seed;
pub use self::WinnersNumber as Raffle_WinnersNumber;

pub struct Id;

pub struct IslmPrizeAmount;

pub struct WinnersNumber;

pub struct Seed;

pub struct AesKey;

#[derive(PartialEq, Clone, Copy)]
pub enum Status {
    ParticipantsRecruitment,
    PrizeTransfer,
    Completed,
    Canceled,
}

impl Status {
    const PARTICIPANT_RECRUITMENT: i16 = 0;
    const PRIZE_TRANSFER: i16 = 1;
    const STATUS_COMPLETED: i16 = 2;
    const STATUS_CANCELED: i16 = 3;

    pub fn from_representation(representation: i16) -> Self {
        let self_ = match representation {
            Self::PARTICIPANT_RECRUITMENT => Self::ParticipantsRecruitment,
            Self::PRIZE_TRANSFER => Self::PrizeTransfer,
            Self::STATUS_COMPLETED => Self::Completed,
            Self::STATUS_CANCELED => Self::Canceled,
            _ => Self::Completed,
        };

        return self_;
    }

    pub fn to_representation<'a>(&'a self) -> i16 {
        let representation = match *self {
            Self::ParticipantsRecruitment => Self::PARTICIPANT_RECRUITMENT,
            Self::PrizeTransfer => Self::PRIZE_TRANSFER,
            Self::Completed => Self::STATUS_COMPLETED,
            Self::Canceled => Self::STATUS_CANCELED,
        };

        return representation;
    }
}

pub struct CreatedAt;

pub struct ExpiredAt;

pub struct Raffle {
    pub id: i64,
    _id: PhantomData<Id>,

    pub islm_prize_amount: i64,
    _islm_prize_amount: PhantomData<IslmPrizeAmount>,

    pub winners_number: i64,
    _winners_number: PhantomData<WinnersNumber>,

    pub seed: String,
    _seed: PhantomData<Seed>,

    pub aes_key: String,
    _aes_key: PhantomData<AesKey>,

    pub status: Status,

    pub created_at: i64,
    _created_at: PhantomData<CreatedAt>,

    pub expired_at: i64,
    _expired_at: PhantomData<ExpiredAt>,
}

impl Raffle {
    pub fn new(
        id: i64,
        islm_prize_amount: i64,
        winners_number: i64,
        seed: String,
        aes_key: String,
        status: Status,
        created_at: i64,
        expired_at: i64,
    ) -> Self {
        return Self {
            id,
            _id: PhantomData,
            islm_prize_amount,
            _islm_prize_amount: PhantomData,
            winners_number,
            _winners_number: PhantomData,
            seed,
            _seed: PhantomData,
            aes_key,
            _aes_key: PhantomData,
            status,
            created_at,
            _created_at: PhantomData,
            expired_at,
            _expired_at: PhantomData,
        };
    }
}

pub struct Raffle_1 {
    pub id: i64,
    pub expired_at: i64,
}

pub struct Raffle_2 {
    pub status: Status,
}

pub struct Raffle_3 {
    pub status: Status,
    pub expired_at: i64,
}

#[derive(Clone, Copy)]
pub struct Raffle_4 {
    pub id: i64,
}

pub struct Raffle_5 {
    pub islm_prize_amount: i64,
    pub winners_number: i64,
    pub expired_at: i64,
}