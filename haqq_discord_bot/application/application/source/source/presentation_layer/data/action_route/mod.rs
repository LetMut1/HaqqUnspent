pub mod matching;

pub const ACTION_ROUTE: ActionRoute = ActionRoute {
    health_check: ActionRoute::HEALTH_CHECK,
};

pub struct ActionRoute {
    pub health_check: &'static str,
}

impl ActionRoute {
    const HEALTH_CHECK: &'static str = "/healthz";
}
