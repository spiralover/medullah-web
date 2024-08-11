use std::sync::{Arc, OnceLock};

#[cfg(feature = "feat-database")]
use diesel::PgConnection;
#[cfg(feature = "feat-database")]
use diesel::r2d2::ConnectionManager;
use redis::Client;

use crate::app_state::MedullahState;
#[cfg(feature = "feat-database")]
use crate::database::DatabaseConnectionHelper;
use crate::MEDULLAH;
use crate::redis::Redis;
use crate::services::cache_service::CacheService;

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

    fn redis_pool(&self) -> deadpool_redis::Pool {
        self.app().redis_pool.clone()
    }

    fn redis(&self) -> &Redis {
        &MEDULLAH.get().unwrap().redis
    }

    #[cfg(feature = "feat-rabbitmq")]
    fn rabbitmq_client(&self) -> Arc<lapin::Connection> {
        Arc::clone(&self.app().rabbitmq_client)
    }

    #[cfg(feature = "feat-rabbitmq")]
    fn rabbitmq_pool(&self) -> deadpool_lapin::Pool {
        self.app().rabbitmq_pool.clone()
    }

    #[cfg(feature = "feat-rabbitmq")]
    fn rabbitmq(&self) -> Arc<crate::rabbitmq::RabbitMQ> {
        Arc::clone(&self.app().rabbitmq)
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
