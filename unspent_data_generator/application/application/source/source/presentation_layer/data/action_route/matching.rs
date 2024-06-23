pub enum ActionRoute {
    Task {
        task: Task,
    },
    HealthCheck,
}

pub enum Task {
    ForceExecute,
    HealthCheck,
}
