pub enum ActionRoute {
    BalanceSnapshot {
        balance_snapshot: BalanceSnapshot,
    },
    BaseBalanceSnapshot {
        base_balance_snapshot: BaseBalanceSnapshot,
    },
    SubportfolioBaseBalanceSnapshot {
        subportfolio_base_balance_snapshot: SubportfolioBaseBalanceSnapshot,
    },
    AssetSnapshot {
        asset_snapshot: AssetSnapshot,
    },
    Subportfolio {
        subportfolio: Subportfolio,
    },
    SubportfolioAsset {
        subportfolio_asset: SubportfolioAsset,
    },
    SubportfolioLink {
        subportfolio_link: SubportfolioLink,
    },
    SubportfolioTrackableWallet {
        subportfolio_trackable_wallet: SubportfolioTrackableWallet,
    },
    HealthCheck1,
    HealthCheck2,
}

pub enum BalanceSnapshot {
    GetHistory,
    Create,
}

pub enum BaseBalanceSnapshot {
    Create,
}

pub enum SubportfolioBaseBalanceSnapshot {
    GetHistory,
    GetHistoryForSubportfolioLink,
}

pub enum AssetSnapshot {
    Create,
    GetHistory,
    GetHistoryForSubportfolioLink,
    GetHistoryForPriceDifferencePercentageCalculating,
}

pub enum Subportfolio {
    Create,
    Delete,
    GetAll,
    Update,
}

pub enum SubportfolioAsset {
    Update,
    GetAllForSubportfolio,
    GetAllForSubportfolioLink,
    CreateForTrackableWallet,
}

pub enum SubportfolioLink {
    Create,
    Delete,
    GetAll,
    Update,
}

pub enum SubportfolioTrackableWallet {
    GetAll,
    GetAllForSubportfolio,
    Update,
}
