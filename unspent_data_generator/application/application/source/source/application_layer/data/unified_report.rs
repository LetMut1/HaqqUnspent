use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
pub enum UnifiedReport<T, P> {
    Target {
        data: Data<T>,
    },
    Precedent {
        precedent: P,
    },
}

impl<T, P> UnifiedReport<T, P> {
    pub fn is_data_empty<'a>(&'a self) -> bool {
        let data = match *self {
            Self::Target {
                data: ref data_,
            } => data_,
            _ => {
                return false;
            }
        };

        let is_empty = match *data {
            Data::Empty => true,
            _ => false,
        };

        return is_empty;
    }

    pub fn get_data(self) -> Option<T> {
        let data = match self {
            Self::Target {
                data: data_,
            } => data_,
            _ => {
                return None;
            }
        };

        let data_ = match data {
            Data::Filled {
                data: data__,
            } => Some(data__),
            _ => None,
        };

        return data_;
    }
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

#[derive(Serialize, Deserialize)]
pub enum Data<D> {
    Empty,
    Filled {
        data: D,
    },
}
