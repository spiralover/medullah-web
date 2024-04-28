use std::future::Future;
use std::time::Duration;
use log::error;
use tokio::{spawn, time};
use crate::enums::app_message::AppMessage;
use crate::results::AppResult;

pub struct Tokio;

impl Tokio {
    pub async fn run_blocking<Func, Ret>(func: Func) -> AppResult<Ret>
    where
        Func: FnOnce() -> Ret + Send + 'static,
        Ret: Send + 'static,
    {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        rt.spawn_blocking(func).await.map_err(AppMessage::JoinError)
    }

    pub fn timeout<Fun, Fut>(interval: u64, func: Fun)
    where
        Fun: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = AppResult<()>> + Send + 'static,
    {
        spawn( async move {
            let mut interval = time::interval(Duration::from_secs(interval));

            interval.tick().await;
            interval.tick().await;

            match func().await {
                Ok(_) => {}
                Err(err) => {
                    error!("[execution-error] {:?}", err);
                }
            }
        });
    }
}
