use std::sync::{Arc, OnceLock};

#[cfg(feature = "feat-database")]
use diesel::r2d2::ConnectionManager;
#[cfg(feature = "feat-database")]
use diesel::PgConnection;
use redis::Client;

use crate::app_state::MedullahState;
#[cfg(feature = "feat-database")]
use crate::database::DatabaseConnectionHelper;
use crate::redis::RedisPool;
use crate::services::cache_service::CacheService;
use crate::services::redis_service::RedisService;
use crate::MEDULLAH;

pub trait OnceLockHelper<'a> {
    fn app(&self) -> &'a MedullahState {
        MEDULLAH.get().unwrap()
    }

    fn front_url(&self, url: &str) -> String {
        self.app().frontend(url)
    }

    #[cfg(feature = "feat-database")]
    fn database(&self) -> &'a crate::database::DBPool {
        MEDULLAH.get().unwrap().database()
    }

    fn redis(&self) -> Arc<Client> {
        Arc::clone(&self.app().redis)
    }

    #[cfg(feature = "feat-rabbitmq")]
    fn rabbitmq(&self) -> Arc<lapin::Connection> {
        Arc::clone(&self.app().rabbit)
    }

    fn redis_pool(&self) -> Arc<RedisPool> {
        Arc::clone(&self.app().redis_pool)
    }

    fn cache(&self) -> &CacheService {
        &MEDULLAH.get().unwrap().services.cache
    }

    fn redis_service(&self) -> &RedisService {
        &MEDULLAH.get().unwrap().services.redis
    }

    #[cfg(feature = "feat-rabbitmq")]
    fn rabbitmq_service(&self) -> Arc<crate::services::rabbit_service::RabbitService> {
        Arc::clone(&MEDULLAH.get().unwrap().services.rabbitmq)
    }

    #[cfg(feature = "feat-database")]
    fn db(
        &self,
    ) -> crate::prelude::AppResult<r2d2::PooledConnection<ConnectionManager<PgConnection>>> {
        self.database().connection()
    }
}

impl<'a> OnceLockHelper<'a> for OnceLock<MedullahState> {}
