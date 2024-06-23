pub mod matching;

pub const ACTION_ROUTE: ActionRoute = ActionRoute {
    balance_snapshot: BalanceSnapshot {
        get_history: BalanceSnapshot::GET_HISTORY,
        create: BalanceSnapshot::CREATE,
    },
    base_balance_snapshot: BaseBalanceSnapshot {
        create: BaseBalanceSnapshot::CREATE,
    },
    subportfolio_base_balance_snapshot: SubportfolioBaseBalanceSnapshot {
        get_history: SubportfolioBaseBalanceSnapshot::GET_HISTORY,
        get_history_for_subportfolio_link: SubportfolioBaseBalanceSnapshot::GET_HISTORY_FOR_SUBPORTFOLIO_LINK,
    },
    asset_snapshot: AssetSnapshot {
        create: AssetSnapshot::CREATE,
        get_history: AssetSnapshot::GET_HISTORY,
        get_history_for_subportfolio_link: AssetSnapshot::GET_HISTORY_FOR_SUBPORTFOLIO_LINK,
        get_history_for_price_difference_percentage_calculating: AssetSnapshot::GET_HISTORY_FOR_PRICE_DIFFERENCE_PERCENTAGE_CALCULATING,
    },
    subportfolio: Subportfolio {
        create: Subportfolio::CREATE,
        delete: Subportfolio::DELETE,
        get_all: Subportfolio::GET_ALL,
        update: Subportfolio::UPDATE,
    },
    subportfolio_asset: SubportfolioAsset {
        update: SubportfolioAsset::UPDATE,
        get_all_for_subportfolio: SubportfolioAsset::GET_ALL_FOR_SUBPORTFOLIO,
        get_all_for_subportfolio_link: SubportfolioAsset::GET_ALL_FOR_SUBPORTFOLIO_LINK,
        create_for_trackable_wallet: SubportfolioAsset::CREATE_FOR_TRACKABLE_WALLET,
    },
    subportfolio_link: SubportfolioLink {
        create: SubportfolioLink::CREATE,
        delete: SubportfolioLink::DELETE,
        get_all: SubportfolioLink::GET_ALL,
        update: SubportfolioLink::UPDATE,
    },
    subportfolio_trackable_wallet: SubportfolioTrackableWallet {
        get_all: SubportfolioTrackableWallet::GET_ALL,
        get_all_for_subportfolio: SubportfolioTrackableWallet::GET_ALL_FOR_SUBPORTFOLIO,
        update: SubportfolioTrackableWallet::UPDATE,
    },
    health_check_1: ActionRoute::HEALTH_CHECK_1,
    health_check_2: ActionRoute::HEALTH_CHECK_2,
};

pub struct ActionRoute {
    pub balance_snapshot: BalanceSnapshot,
    pub base_balance_snapshot: BaseBalanceSnapshot,
    pub subportfolio_base_balance_snapshot: SubportfolioBaseBalanceSnapshot,
    pub asset_snapshot: AssetSnapshot,
    pub subportfolio: Subportfolio,
    pub subportfolio_asset: SubportfolioAsset,
    pub subportfolio_link: SubportfolioLink,
    pub subportfolio_trackable_wallet: SubportfolioTrackableWallet,
    pub health_check_1: &'static str,
    pub health_check_2: &'static str,
}

impl ActionRoute {
    const HEALTH_CHECK_1: &'static str = "/healthz";
    const HEALTH_CHECK_2: &'static str = "/healthz_all";
}

pub struct BalanceSnapshot {
    pub get_history: &'static str,
    pub create: &'static str,
}

impl BalanceSnapshot {
    const GET_HISTORY: &'static str = "/balance_snapshot/history";
    const CREATE: &'static str = "/balance_snapshot/create";
}

pub struct BaseBalanceSnapshot {
    pub create: &'static str,
}

impl BaseBalanceSnapshot {
    const CREATE: &'static str = "/base_balance_snapshot/create";
}

pub struct SubportfolioBaseBalanceSnapshot {
    pub get_history: &'static str,
    pub get_history_for_subportfolio_link: &'static str,
}

impl SubportfolioBaseBalanceSnapshot {
    const GET_HISTORY: &'static str = "/subportfolio_base_balance_snapshot/history";
    const GET_HISTORY_FOR_SUBPORTFOLIO_LINK: &'static str = "/subportfolio_base_balance_snapshot/history_for_subportfolio_link";
}

pub struct AssetSnapshot {
    pub create: &'static str,
    pub get_history: &'static str,
    pub get_history_for_subportfolio_link: &'static str,
    pub get_history_for_price_difference_percentage_calculating: &'static str,
}

impl AssetSnapshot {
    const CREATE: &'static str = "/asset_snapshot/create";
    const GET_HISTORY: &'static str = "/asset_snapshot/history";
    const GET_HISTORY_FOR_SUBPORTFOLIO_LINK: &'static str = "/asset_snapshot/history_for_subportfolio_link";
    const GET_HISTORY_FOR_PRICE_DIFFERENCE_PERCENTAGE_CALCULATING: &'static str = "/asset_snapshot/history_for_price_difference_percentage_calculating";
}

pub struct Subportfolio {
    pub create: &'static str,
    pub delete: &'static str,
    pub get_all: &'static str,
    pub update: &'static str,
}

impl Subportfolio {
    const CREATE: &'static str = "/subportfolio/create";
    const DELETE: &'static str = "/subportfolio/delete";
    const GET_ALL: &'static str = "/subportfolio/all";
    const UPDATE: &'static str = "/subportfolio/update";
}

pub struct SubportfolioAsset {
    pub update: &'static str,
    pub get_all_for_subportfolio: &'static str,
    pub get_all_for_subportfolio_link: &'static str,
    pub create_for_trackable_wallet: &'static str,
}

impl SubportfolioAsset {
    const UPDATE: &'static str = "/subportfolio_asset/update";
    const GET_ALL_FOR_SUBPORTFOLIO: &'static str = "/subportfolio_asset/all_for_subportfolio";
    const GET_ALL_FOR_SUBPORTFOLIO_LINK: &'static str = "/subportfolio_asset/all_for_subportfolio_link";
    const CREATE_FOR_TRACKABLE_WALLET: &'static str = "/subportfolio_asset/create_for_trackable_wallet";
}

pub struct SubportfolioLink {
    pub create: &'static str,
    pub delete: &'static str,
    pub get_all: &'static str,
    pub update: &'static str,
}

impl SubportfolioLink {
    const CREATE: &'static str = "/subportfolio_link/create";
    const DELETE: &'static str = "/subportfolio_link/delete";
    const GET_ALL: &'static str = "/subportfolio_link/all";
    const UPDATE: &'static str = "/subportfolio_link/update";
}

pub struct SubportfolioTrackableWallet {
    pub get_all: &'static str,
    pub get_all_for_subportfolio: &'static str,
    pub update: &'static str,
}

impl SubportfolioTrackableWallet {
    const GET_ALL: &'static str = "/subportfolio_trackable_wallet/all";
    const GET_ALL_FOR_SUBPORTFOLIO: &'static str = "/subportfolio_trackable_wallet/all_for_subportfolio";
    const UPDATE: &'static str = "/subportfolio_trackable_wallet/update";
}
