use std::env;
use std::time::Duration;
use mobc::{async_trait, Manager, Pool};
use redis::aio::Connection;
use redis::Client;

use crate::redis::{RedisConnectionManager, RedisPool};

const CACHE_POOL_MAX_OPEN: u64 = 16;
const CACHE_POOL_MAX_IDLE: u64 = 8;
const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

impl RedisConnectionManager {
    pub fn new(c: Client) -> Self {
        Self { client: c }
    }
}

#[async_trait]
impl Manager for RedisConnectionManager {
    type Connection = Connection;
    type Error = redis::RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let c = self.client.get_tokio_connection().await?;
        Ok(c)
    }

    async fn check(&self, mut conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        redis::cmd("PING").query_async(&mut conn).await?;
        Ok(conn)
    }
}

pub fn establish_redis_connection(env_prefix: &String) -> Client {
    let redis_url: String = env::var(format!("{}_REDIS_DSN", env_prefix)).unwrap();
    Client::open(redis_url).unwrap()
}

pub fn establish_redis_connection_pool(env_prefix: &String) -> RedisPool {
    let redis_url: String = env::var(format!("{}_REDIS_DSN", env_prefix)).unwrap();
    let client = Client::open(redis_url).unwrap();
    let manager = RedisConnectionManager::new(client);
    Pool::builder()
        .get_timeout(Some(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS)))
        .max_open(CACHE_POOL_MAX_OPEN)
        .max_idle(CACHE_POOL_MAX_IDLE)
        .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
        .build(manager)
}
