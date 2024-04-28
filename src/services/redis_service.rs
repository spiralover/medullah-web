use std::num::NonZeroUsize;

use log::{debug, error};
use mobc::Connection;
use redis::{AsyncCommands, FromRedisValue};
use serde::Serialize;

use crate::enums::app_message::AppMessage;
use crate::redis::{RedisConnectionManager, RedisPool};
use crate::results::redis_result::RedisResultToAppResult;
use crate::results::AppResult;

#[derive(Clone)]
pub struct RedisService {
    pool: RedisPool,
}

pub struct SubscribableQueue(pub String, pub String);

impl RedisService {
    pub fn new(pool: RedisPool) -> RedisService {
        RedisService { pool }
    }

    pub async fn redis(&self) -> AppResult<Connection<RedisConnectionManager>> {
        self.pool.get().await.map_err(AppMessage::RedisPoolError)
    }

    /// Publish and queue
    pub async fn paq<T: Serialize + Clone>(
        &self,
        queue: SubscribableQueue,
        data: T,
    ) -> AppResult<i32> {
        let result = self.queue(queue.0, data.clone()).await;

        // Push to respective channel
        debug!("[publisher]: publishing to {}", queue.1.clone());
        match self.publish(queue.1.clone(), data).await {
            Ok(_) => {}
            Err(err) => error!("[publisher][{}]: {:?}", queue.1, err),
        };

        match result {
            Ok(result) => Ok(result),
            Err(error) => {
                error!("[queue][{}]: {:?}", queue.1, error);
                Err(error)
            }
        }
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

    pub async fn delete(&self, key: String) -> AppResult<String> {
        match self.pool.get().await {
            Ok(mut conn) => conn.del::<String, String>(key).await.into_app_result(),
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
}
