use hyper::{
    Body,
    Request as HyperRequest,
    Response as HyperResponse,
};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct ServerAccessToken(pub String);

pub struct ActionRound;

pub struct GeneralizedAction;

pub struct GeneralizedSpecialAction;

pub struct HealthCheck;

pub struct HttpBodyData;

pub struct CronJob;

pub struct Common;

pub struct Json;

pub struct RouteNotFound;

pub struct RunAllTasks;

pub struct Task___ForceExecute;

pub struct Task___HealthCheck;

pub struct UpdateAssets;

pub struct TokioBlockingTask;

pub struct TokioNonBlockingTask;

pub struct UTCDateTime;

pub type Request = HyperRequest<Body>;

pub type Response = HyperResponse<Body>;
