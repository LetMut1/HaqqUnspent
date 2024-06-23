pub use self::{
    CirculatingSupply as Asset_CirculatingSupply,
    Id as Asset_Id,
    LastUpdatedTimestamp as Asset_LastUpdateTimestamp,
    MarketCapUsd as Asset_MarketCapUsd,
    Name as Asset_Name,
    PercentChange1y as Asset_PercentChange1y,
    PercentChange24h as Asset_PercentChange24h,
    PercentChange24hBtc as Asset_PercentChange24hBtc,
    PercentChange30d as Asset_PercentChange30d,
    PercentChange7d as Asset_PercentChange7d,
    PlatformId as Asset_PlatfotmId,
    PriceBtc as Asset_PriceBtc,
    PriceUsd as Asset_PriceUsd,
    Rank as Asset_Rank,
    Symbol as Asset_Symbol,
    TotalSupply as Asset_TotalSupply,
    Type as Asset_Type,
};
use serde::{
    Deserialize,
    Serialize,
};
use std::marker::PhantomData;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(pub String);

pub struct Name;

pub struct Symbol;

pub struct PriceUsd;

pub struct PriceBtc;

pub struct MarketCapUsd;

pub struct PercentChange24h;

pub struct PercentChange7d;

pub struct PercentChange30d;

pub struct PercentChange1y;

pub struct PercentChange24hBtc;

#[derive(Serialize, Deserialize)]
pub enum Type {
    Fiat,
    Crypto,
    FiatStablecoin,
    BtcStablecoin,
    EthSTablecoin,
}

impl Type {
    pub fn to_string<'a>(&'a self) -> String {
        return match *self {
            Self::Fiat => "fiat".to_string(),
            Self::Crypto => "crypto".to_string(),
            Self::FiatStablecoin => "fiat-stablecoin".to_string(),
            Self::BtcStablecoin => "btc-stablecoin".to_string(),
            Self::EthSTablecoin => "eth-stablecoin".to_string(),
        };
    }
}

pub struct Rank;

pub struct TotalSupply;

pub struct CirculatingSupply;

pub struct PlatformId;

pub struct LastUpdatedTimestamp;

pub struct ImageUrl;

pub struct Asset {
    // TODO
    pub id: Id,
    _id: PhantomData<Id>,

    pub name: String,
    _name: PhantomData<Name>,

    pub symbol: String,
    _symbol: PhantomData<Symbol>,

    pub price_usd: Option<String>,
    _price_usd: PhantomData<PriceUsd>,

    pub price_btc: Option<String>,
    _price_btc: PhantomData<PriceBtc>,

    pub market_cap_usd: Option<String>,
    _market_cap_usd: PhantomData<MarketCapUsd>,

    pub percent_change_24h: Option<String>,
    _percent_change_24h: PhantomData<PercentChange24h>,

    pub percent_change_7d: Option<String>,
    _percent_change_7d: PhantomData<PercentChange7d>,

    pub percent_change_30d: Option<String>,
    _percent_change_30d: PhantomData<PercentChange30d>,

    pub percent_change_1y: Option<String>,
    _percent_change_1y: PhantomData<PercentChange1y>,

    pub percent_change_24h_btc: Option<String>,
    _percent_change_24h_btc: PhantomData<PercentChange24hBtc>,

    pub r#type: Type,

    pub rank: Option<String>,
    _rank: PhantomData<Rank>,

    pub total_supply: Option<String>,
    _total_supply: PhantomData<TotalSupply>,

    pub circulating_supply: Option<String>,
    _circulating_supply: PhantomData<CirculatingSupply>,

    pub platform_id: Option<String>,
    _platform_id: PhantomData<PlatformId>,

    pub last_updated_timestamp: i64,
    _last_updated_timestamp: PhantomData<LastUpdatedTimestamp>,

    pub image_url: Option<String>,
    _image_url: PhantomData<ImageUrl>,
}

impl Asset {
    pub fn new(
        id: Id,
        name: String,
        symbol: String,
        price_usd: Option<String>,
        price_btc: Option<String>,
        market_cap_usd: Option<String>,
        percent_change_24h: Option<String>,
        percent_change_7d: Option<String>,
        percent_change_30d: Option<String>,
        percent_change_1y: Option<String>,
        percent_change_24h_btc: Option<String>,
        r#type: Type,
        rank: Option<String>,
        total_supply: Option<String>,
        circulating_supply: Option<String>,
        platform_id: Option<String>,
        last_updated_timestamp: i64,
        image_url: Option<String>,
    ) -> Self {
        return Self {
            id,
            _id: PhantomData,
            name,
            _name: PhantomData,
            symbol,
            _symbol: PhantomData,
            price_usd,
            _price_usd: PhantomData,
            price_btc,
            _price_btc: PhantomData,
            market_cap_usd,
            _market_cap_usd: PhantomData,
            percent_change_24h,
            _percent_change_24h: PhantomData,
            percent_change_7d,
            _percent_change_7d: PhantomData,
            percent_change_30d,
            _percent_change_30d: PhantomData,
            percent_change_1y,
            _percent_change_1y: PhantomData,
            percent_change_24h_btc,
            _percent_change_24h_btc: PhantomData,
            r#type,
            rank,
            _rank: PhantomData,
            total_supply,
            _total_supply: PhantomData,
            circulating_supply,
            _circulating_supply: PhantomData,
            platform_id,
            _platform_id: PhantomData,
            last_updated_timestamp,
            _last_updated_timestamp: PhantomData,
            image_url,
            _image_url: PhantomData,
        };
    }
}
