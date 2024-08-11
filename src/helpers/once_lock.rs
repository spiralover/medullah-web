use deadpool::managed::{Object, Pool};
use deadpool_lapin::Manager;
#[cfg(feature = "feat-database")]
use diesel::r2d2::ConnectionManager;
#[cfg(feature = "feat-database")]
use diesel::PgConnection;
use redis::Client;
use std::sync::{Arc, OnceLock};

use crate::app_state::MedullahState;
#[cfg(feature = "feat-database")]
use crate::database::DatabaseConnectionHelper;
use crate::redis::{Redis, RedisPool};
use crate::services::cache_service::CacheService;
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

    fn redis_client(&self) -> Arc<Client> {
        Arc::clone(&self.app().redis_client)
    }

    fn redis_pool(&self) -> Arc<RedisPool> {
        Arc::clone(&self.app().redis_pool)
    }

    fn redis(&self) -> &Redis {
        &MEDULLAH.get().unwrap().redis
    }

    #[cfg(feature = "feat-rabbitmq")]
    fn rabbitmq_client(&self) -> Arc<lapin::Connection> {
        Arc::clone(&self.app().rabbitmq_client)
    }

    #[cfg(feature = "feat-rabbitmq")]
    fn rabbitmq_pool(&self) -> Pool<Manager, Object<Manager>> {
        self.app().rabbitmq_pool.clone()
    }

    #[cfg(feature = "feat-rabbitmq")]
    fn rabbitmq(&self) -> Arc<lapin::Connection> {
        Arc::clone(&self.app().rabbitmq_client)
    }

    fn cache(&self) -> &CacheService {
        &MEDULLAH.get().unwrap().services.cache
    }

    #[cfg(feature = "feat-database")]
    fn db(
        &self,
    ) -> crate::prelude::AppResult<r2d2::PooledConnection<ConnectionManager<PgConnection>>> {
        self.database().connection()
    }
}

impl<'a> OnceLockHelper<'a> for OnceLock<MedullahState> {}
