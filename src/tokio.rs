use crate::enums::app_message::AppMessage;
use crate::results::AppResult;
use log::error;
use std::future::Future;
use std::time::Duration;
use tokio::{spawn, time};

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

    ///
    ///
    /// # Arguments
    ///
    /// * `interval`: an interval within which the given function will be executed (in milliseconds)
    /// * `func`: The function that will be executed
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn timeout<Fun, Fut>(interval: u64, func: Fun, name: &str)
    where
        Fun: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = AppResult<()>> + Send + 'static,
    {
        let name = name.to_owned();
        spawn(async move {
            let mut interval = time::interval(Duration::from_millis(interval));

            interval.tick().await;
            interval.tick().await;

            match func().await {
                Ok(_) => {}
                Err(err) => {
                    error!("[execution-error][{}] {:?}", name, err);
                }
            }
        });
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `interval`: an interval within which the given function will be executed (in milliseconds)
    /// * `func`: the function that will be executed repeatedly
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn tick<Fun, Fut>(interval: u64, func: Fun, name: &str)
    where
        Fun: Fn() -> Fut + Send + 'static,
        Fut: Future<Output = AppResult<()>> + Send + 'static,
    {
        let name = name.to_owned();
        spawn(async move {
            let mut interval = time::interval(Duration::from_millis(interval));

            loop {
                interval.tick().await;

                match func().await {
                    Ok(_) => {}
                    Err(err) => {
                        error!("[execution-error][{}] {:?}", name, err);
                    }
                }
            }
        });
    }
}
