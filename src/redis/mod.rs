use crate::prelude::{AppMessage, AppResult, OnceLockHelper};
use crate::results::redis_result::RedisResultToAppResult;
use crate::MEDULLAH;
use futures_util::StreamExt;
use log::{error, info};
use redis::{AsyncCommands, Client, FromRedisValue, Msg};
use serde::Serialize;
use std::future::Future;
use std::num::NonZeroUsize;
use std::time::Duration;
use tokio::runtime::Handle;
use tokio::time;

pub mod conn;

pub struct RedisConnectionManager {
    pub client: Client,
}

#[derive(Clone)]
pub struct Redis {
    pool: deadpool_redis::Pool,
}

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
            let popped = MEDULLAH.redis().rpop(&queue.clone(), len).await;
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
        let mut pubsub = MEDULLAH.redis_client().get_async_pubsub().await?;

        info!("[subscriber] subscribing to: {}", channel.clone());

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

    pub fn new(pool: deadpool_redis::Pool) -> Self {
        Redis { pool }
    }

    pub async fn redis(&self) -> AppResult<deadpool_redis::Connection> {
        self.pool.get().await.map_err(AppMessage::RedisPoolError)
    }

    pub async fn queue<T: Serialize>(&self, queue: String, data: T) -> AppResult<i32> {
        match self.pool.get().await {
            Ok(mut conn) => {
                let content = serde_json::to_string(&data).unwrap();
                conn.lpush::<&str, &str, i32>(&*queue, &content)
                    .await
                    .into_app_result()
            }
            Err(err) => Err(AppMessage::RedisPoolError(err)),
        }
    }

    pub async fn set<T: Serialize>(&self, key: String, value: T) -> AppResult<String> {
        match self.pool.get().await {
            Ok(mut conn) => {
                let content = serde_json::to_string(&value).unwrap();
                conn.set::<String, String, String>(key, content)
                    .await
                    .into_app_result()
            }
            Err(err) => Err(AppMessage::RedisPoolError(err)),
        }
    }

    pub async fn get<T: FromRedisValue>(&self, key: String) -> AppResult<T> {
        match self.pool.get().await {
            Ok(mut conn) => conn.get::<String, T>(key).await.into_app_result(),
            Err(err) => Err(AppMessage::RedisPoolError(err)),
        }
    }

    pub async fn delete(&self, key: String) -> AppResult<i32> {
        match self.pool.get().await {
            Ok(mut conn) => conn.del::<String, i32>(key).await.into_app_result(),
            Err(err) => Err(AppMessage::RedisPoolError(err)),
        }
    }

    pub async fn publish<T: Serialize>(&self, channel: String, data: T) -> AppResult<i32> {
        match self.pool.get().await {
            Ok(mut conn) => {
                let content = serde_json::to_string(&data).unwrap();
                conn.publish::<String, String, i32>(channel, content)
                    .await
                    .into_app_result()
            }
            Err(err) => Err(AppMessage::RedisPoolError(err)),
        }
    }

    pub async fn rpop(&self, key: &str, count: Option<NonZeroUsize>) -> AppResult<Option<String>> {
        match self.pool.get().await {
            Ok(mut conn) => conn
                .rpop::<&str, Option<String>>(key, count)
                .await
                .into_app_result(),
            Err(err) => Err(AppMessage::RedisPoolError(err)),
        }
    }

    pub async fn flush_all(&self) -> AppResult<()> {
        match self.pool.get().await {
            Ok(mut conn) => redis::cmd("FLUSHALL")
                .query_async(&mut *conn)
                .await
                .into_app_result(),
            Err(err) => Err(AppMessage::RedisPoolError(err)),
        }
    }

    pub async fn flush_db(&self) -> AppResult<()> {
        match self.pool.get().await {
            Ok(mut conn) => redis::cmd("FLUSHDB")
                .query_async(&mut *conn)
                .await
                .into_app_result(),
            Err(err) => Err(AppMessage::RedisPoolError(err)),
        }
    }
}
