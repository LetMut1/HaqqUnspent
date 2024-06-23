use super::Spawner;
pub use crate::infrastructure_layer::data::control_type::TokioNonBlockingTask;
use crate::infrastructure_layer::{
    data::{
        auditor::Auditor,
        error::Error,
    },
    functionality::service::logger::Logger,
};
use std::{
    future::Future,
    marker::Send,
};
use tokio::{
    spawn,
    task::JoinHandle,
};

impl Spawner<TokioNonBlockingTask> {
    pub fn spawn_into_background<F, T>(future: F) -> ()
    where
        F: Future<Output = Result<T, Auditor<Error>>> + Send + 'static,
    {
        let future_ = async move {
            if let Err(error_auditor) = future.await {
                Logger::<Auditor<Error>>::log(&error_auditor);
            }

            return ();
        };

        spawn(future_);
    }

    pub fn spawn_processed<F, T>(future: F) -> JoinHandle<<F as Future>::Output>
    where
        F: Future + Send + 'static,
        <F as Future>::Output: Send + 'static,
    {
        return spawn(future);
    }
}
