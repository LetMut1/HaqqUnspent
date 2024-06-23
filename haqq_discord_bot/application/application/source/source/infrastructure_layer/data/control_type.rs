use hyper::Body;
use hyper::Request as HyperRequest;
use hyper::Response as HyperResponse;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct ServerAccessToken(pub String);

pub struct ActionRound;

pub struct GeneralizedAction;

pub struct HealthCheck;

pub struct HttpBodyData;

pub struct Common;

pub struct Json;

pub struct RouteNotFound;

pub struct RunBot;

pub struct RunServer;

pub struct UpdateDiscordRoles;

pub struct DiscordCompositeCustomId;

pub struct VerifyWallet;

pub struct ServeRaffle;

pub struct CancelRaffle;

pub struct CompleteRaffle;

pub struct UpdateRaffle;

pub struct Cosmos;

pub struct EVM;

pub struct TokioBlockingTask;

pub struct TokioNonBlockingTask;

pub struct UTCDateTime;

pub type Request = HyperRequest<Body>;

pub type Response = HyperResponse<Body>;
