macro_rules! task_started_to_execute_pattern {
    () => {
        "The '{}' task started to execute."
    };
}

macro_rules! coingecko_response_status_pattern {
    () => {
        "Coingecko response status code is {}."
    };
}

macro_rules! task_executing_by_force_execute_pattern {
    () => {
        "The {} task executing by ForceExecute action is complited successfully."
    };
}

pub(crate) use coingecko_response_status_pattern;
pub(crate) use task_executing_by_force_execute_pattern;
pub(crate) use task_started_to_execute_pattern;
