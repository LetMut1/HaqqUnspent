pub mod matching;

pub const ACTION_ROUTE: ActionRoute = ActionRoute {
    health_check: ActionRoute::HEALTH_CHECK,
    task: Task {
        force_execute: Task::FORCE_EXECUTE,
        health_check: Task::HEALTH_CHECK,
    },
};

pub struct ActionRoute {
    pub health_check: &'static str,
    pub task: Task,
}

impl ActionRoute {
    const HEALTH_CHECK: &'static str = "/healthz";
}

pub struct Task {
    pub force_execute: &'static str,
    pub health_check: &'static str,
}

impl Task {
    const FORCE_EXECUTE: &'static str = "/task/force_execute";
    const HEALTH_CHECK: &'static str = "/task/health_check";
}
