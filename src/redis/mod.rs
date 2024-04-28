use mobc::Pool;
use redis::Client;

pub mod client;
pub mod conn;

pub struct RedisConnectionManager {
    pub client: Client,
}

pub type RedisPool = Pool<RedisConnectionManager>;
