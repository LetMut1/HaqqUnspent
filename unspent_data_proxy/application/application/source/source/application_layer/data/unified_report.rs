use serde::Serialize;

#[derive(Serialize)]
pub enum UnifiedReport<T, P> {
    Target {
        data: Data<T>,
    },
    Precedent {
        precedent: P,
    },
}

impl<T, P> UnifiedReport<T, P>
where
    T: Serialize,
    P: Serialize,
{
    pub fn empty() -> Self {
        return Self::Target {
            data: Data::Empty,
        };
    }

    pub fn filled(data: T) -> Self {
        return Self::Target {
            data: Data::Filled {
                data,
            },
        };
    }

    pub fn precedent(precedent: P) -> Self {
        return Self::Precedent {
            precedent,
        };
    }
}

#[derive(Serialize)]
pub enum Data<D> {
    Empty,
    Filled {
        data: D,
    },
}

pub enum CommonPrecedent {
    Subportfolio_AlreadyExists,
    Subportfolio_DoesNotExist,
    Subportfolio_MaximumQuantityPerUser,
    SubportfolioAsset_AlreadyExist,
    SubportfolioAsset_DoesNotExist,
    SubportfolioAsset_MaximumQuantityPerSubportfolio,
    SubportfolioLink_DoesNotExist,
    SubportfolioLink_IsNotActive,
    SubportfolioLink_MaximumQuantityPerUserAndSubportfolio,
    SubportfolioTrackableWallet_MaximumQuantityPerUserAndSubportfolio,
}
