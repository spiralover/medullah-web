use std::future::Future;
use std::sync::Arc;

use log::debug;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::enums::app_message::AppMessage;
use crate::prelude::IntoAppResult;
use crate::results::AppResult;
use crate::services::redis_service::RedisService;

#[derive(Clone)]
pub struct CacheService {
    redis: Arc<RedisService>,
}

impl CacheService {
    pub fn new(r: Arc<RedisService>) -> CacheService {
        CacheService { redis: r }
    }

    pub fn redis(&self) -> &RedisService {
        &self.redis
    }

    pub async fn put<T>(&self, key: &str, value: T) -> AppResult<String>
    where
        T: Serialize,
    {
        self.redis.set(key.to_string(), value).await
    }

    pub async fn get<T: DeserializeOwned>(&mut self, key: &str) -> AppResult<Option<T>> {
        let data = self.redis.get::<Option<String>>(key.to_string()).await?;

        match data {
            None => Ok(None),
            Some(data) => Ok(Some(
                serde_json::from_str::<T>(&data).map_err(AppMessage::SerdeError)?,
            )),
        }
    }

    pub async fn delete(&self, key: &str) -> AppResult<String> {
        self.redis.delete(key.to_string()).await
    }

    pub async fn get_or_put<Val, Fun, Fut>(&self, key: &str, setter: Fun) -> AppResult<Val>
    where
        Val: Serialize + DeserializeOwned + Clone,
        Fun: FnOnce(&Self) -> Fut + Send + 'static,
        Fut: Future<Output = AppResult<Val>> + Send + 'static,
    {
        let result = self.redis.get::<Option<String>>(key.to_string()).await;

        match result {
            Ok(option) => match option {
                None => {
                    debug!("'{}' is missing in cache, executing setter()...", key);
                    match setter(self).await {
                        Ok(value) => match self.put(key, value.clone()).await {
                            Ok(_) => Ok(value),
                            Err(err) => Err(err),
                        },
                        Err(err) => Err(err),
                    }
                }
                Some(data) => {
                    debug!("'{}' collected from cache :)", key);
                    serde_json::from_str::<Val>(&Box::pin(data)).into_app_result()
                }
            },
            Err(err) => Err(err),
        }
    }
}
