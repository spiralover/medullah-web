use std::future::Future;
use std::num::NonZeroUsize;
use std::time::Duration;

use futures_util::StreamExt;
use log::{error, info};
use redis::Msg;
use tokio::runtime::Handle;
use tokio::time;

use crate::helpers::once_lock::OnceLockHelper;
use crate::results::redis_result::RedisResultToAppResult;
use crate::results::AppResult;
use crate::APP;

pub struct Redis;

impl Redis {
    pub fn secs(ms: u64) -> Option<u64> {
        Some(ms * 1000)
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `queue`: redis queue you are polling
    /// * `interval`: interval within which the queue will be polled in microsecond, default: 500ms
    /// * `len`: total number of items to be pulled per each poll, default: 1
    /// * `func`: async function to be executed for each poll
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub async fn poll_queue<F, Fut>(
        queue: String,
        interval: Option<u64>,
        len: Option<NonZeroUsize>,
        func: F,
    ) where
        F: FnOnce(String) -> Fut + Copy + Send + 'static,
        Fut: Future<Output = AppResult<()>> + Send + 'static,
    {
        info!("[queue] polling: {}", queue);

        let mut interval = time::interval(Duration::from_micros(interval.unwrap_or(500)));

        loop {
            let queue = queue.clone();
            let popped = APP.redis_service().rpop(&queue.clone(), len).await;
            match popped {
                Ok(Some(item)) => {
                    Handle::current().spawn(async move {
                        match func(item).await {
                            Ok(_) => {}
                            Err(err) => {
                                error!("[queue][{}] executor returned error: {:?}", queue, err);
                            }
                        }
                    });
                }
                Ok(None) => {
                    interval.tick().await;
                }
                Err(err) => {
                    error!("[queue][{}] failed to pop queue: {:?}", queue, err);
                    interval.tick().await;
                }
            };
        }
    }

    pub async fn subscribe<F, Fut>(channel: String, func: F) -> AppResult<()>
    where
        F: FnOnce(AppResult<String>) -> Fut + Copy + Send + 'static,
        Fut: Future<Output = AppResult<()>> + Send + 'static,
    {
        let conn = APP.redis().get_tokio_connection().await?;

        info!("[subscriber] subscribing to: {}", channel.clone());

        let mut pubsub = conn.into_pubsub();
        pubsub.subscribe(&[channel.clone()]).await?;

        let mut stream = pubsub.into_on_message();
        while let Some(msg) = stream.next().await {
            let channel = channel.clone();
            Handle::current().spawn(async move {
                let msg: Msg = msg; // to make RustRover happy
                let received = msg.get_payload::<String>().into_app_result();

                match func(received).await {
                    Ok(_) => {}
                    Err(err) => {
                        error!(
                            "[subscriber][{}] executor returned error: {:?}",
                            channel, err
                        );
                    }
                };
            });
        }

        Ok(())
    }
}
