use hyper::{
    Body,
    Request as HyperRequest,
    Response as HyperResponse,
};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct ServerAccessToken(pub String);

#[derive(Deserialize)]
#[serde(transparent)]
pub struct AccessToken(pub String);

pub struct ActionRound;

pub struct AssetSnapshot___Create;

pub struct AssetSnapshot___GetHistory;

pub struct AssetSnapshot___GetHistoryForSubportfolioLink;

pub struct AssetSnapshot___GetHistoryForPriceDifferencePercentageCalculating;

pub struct BalanceSnapshot___Create;

pub struct BalanceSnapshot___GetHistory;

pub struct BaseBalanceSnapshot___Create;

pub struct CreateFixtures;

pub struct CorsPreflightRequest;

pub struct GeneralizedAction;

pub struct HealthCheck1;

pub struct HealthCheck2;

pub struct HttpBodyData;

pub struct Json;

pub struct Subportfolio___Create;

pub struct Subportfolio___Delete;

pub struct Subportfolio___GetAll;

pub struct Subportfolio___Update;

pub struct SubportfolioLink___Create;

pub struct SubportfolioLink___Delete;

pub struct SubportfolioLink___Update;

pub struct SubportfolioLink___GetAll;

pub struct SubportfolioAsset___GetAllForSubportfolio;

pub struct SubportfolioAsset___GetAllForSubportfolioLink;

pub struct SubportfolioAsset___Update;

pub struct SubportfolioBaseBalanceSnapshot___GetHistory;

pub struct SubportfolioBaseBalanceSnapshot___GetHistoryForSubportfolioLink;

pub struct SubportfolioTrackableWallet___Update;

pub struct SubportfolioTrackableWallet___GetAll;

pub struct SubportfolioTrackableWallet___GetAllForSubportfolio;

pub struct SubportfolioAsset___CreateForTrackableWallet;

pub struct RouteNotFound;

pub struct RunServer;

pub struct SnapshotRange;

pub struct TokioBlockingTask;

pub struct TokioNonBlockingTask;

pub struct UTCDateTime;

pub struct DefaultValue;

pub type Request = HyperRequest<Body>;

pub type Response = HyperResponse<Body>;
