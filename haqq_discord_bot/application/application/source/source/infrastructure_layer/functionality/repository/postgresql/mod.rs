pub mod address_verification_data;
pub mod aislm_stake;
pub mod discord_user_role;
pub mod prize_transfer_proof;
pub mod raffle_participant;
pub mod raffle_winner;
pub mod raffle;
pub mod recipient_hd_wallet;
pub mod sender_hd_wallet;
pub mod verified_address_blacklist;
pub mod verified_bech32_address;

use std::marker::PhantomData;

pub struct PostgresqlRepository<E> {
    _entity: PhantomData<E>,
}

pub mod insert {
    use crate::domain_layer::data::entity::raffle::Raffle_Status;

    pub struct Insert_1 {
        pub recipient_hd_wallet_mnemonic_phrase: String,
        pub recipient_hd_wallet_mnemonic_derevation_path_index: i32,
        pub recipient_hd_wallet_created_at: i64,
    }

    pub struct Insert_2 {
        pub raffle_islm_prize_amount: i64,
        pub raffle_winners_number: i64,
        pub raffle_seed: String,
        pub raffle_aes_key: String,
        pub raffle_status: Raffle_Status,
        pub raffle_created_at: i64,
        pub raffle_expired_at: i64,
    }

    pub struct Insert_3 {
        pub aislm_stake_amount: String,
        pub aislm_stake_raffle_id: i64,
        pub aislm_stake_bech32_address: String,
        pub aislm_stake_discord_user_id: String,
        pub aislm_stake_created_at: i64,
    }

    pub struct Insert_4 {
        pub sender_hd_wallet_mnemonic_phrase: String,
        pub sender_hd_wallet_mnemonic_derevation_path_index: i32,
        pub sender_hd_wallet_created_at: i64,
    }
}

pub mod update {
    use crate::domain_layer::data::entity::raffle::Raffle_Status;

    pub struct Update_1 {
        pub discord_user_role_wallet_verified: bool,
        pub discord_user_role_stakers_club_member: bool,
        pub discord_user_role_updated_at: i64,
    }

    pub struct Update_2 {
        pub discord_user_role_wallet_verified: bool,
        pub discord_user_role_updated_at: i64,
    }

    pub struct Update_3 {
        pub discord_user_role_stakers_club_member: bool,
        pub discord_user_role_updated_at: i64,
    }

    pub struct Update_4 {
        pub raffle_status: Raffle_Status,
    }

    #[derive(Clone, Copy)]
    pub struct Update_5 {
        pub raffle_islm_prize_amount: i64,
        pub raffle_winners_number: i64,
        pub raffle_expired_at: i64,
    }
}

pub mod by {
    pub struct By_1 {
        pub recipient_hd_wallet_id: i64,
    }

    pub struct By_2<'a> {
        pub address_verification_data_discord_user_id: &'a str,
        pub address_verification_data_recipient_bech32_address: &'a str,
    }

    pub struct By_3<'a> {
        pub verified_bech32_address_discord_user_id: &'a str,
    }

    pub struct By_4<'a> {
        pub verified_address_blacklist_bech32_address: &'a str,
    }

    pub struct By_5<'a> {
        pub verified_bech32_address_value: &'a str,
    }

    pub struct By_6<'a> {
        pub raffle_participant_discord_user_id: &'a str,
        pub raffle_participant_raffle_id: i64,
    }

    pub struct By_7<'a> {
        pub discord_user_role_discord_user_id: &'a str,
    }

    #[derive(Clone, Copy)]
    pub struct By_8 {
        pub raffle_id: i64,
    }

    pub struct By_9 {
        pub aislm_stake_raffle_id: i64,
    }

    pub struct By_10 {
        pub sender_hd_wallet_id: i64,
    }
}
