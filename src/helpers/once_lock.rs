#[allow(unused_imports)]
use std::sync::{Arc, OnceLock};

#[cfg(feature = "feat-database")]
use diesel::r2d2::ConnectionManager;
#[cfg(feature = "feat-database")]
use diesel::PgConnection;

use crate::app_state::MedullahState;
#[cfg(feature = "feat-database")]
use crate::database::DatabaseConnectionHelper;
#[cfg(feature = "feat-redis")]
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

    #[cfg(feature = "feat-redis")]
    fn redis_client(&self) -> Arc<redis::Client> {
        Arc::clone(&self.app().redis_client)
    }

    #[cfg(feature = "feat-redis")]
    fn redis_pool(&self) -> deadpool_redis::Pool {
        self.app().redis_pool.clone()
    }

    #[cfg(feature = "feat-redis")]
    fn redis(&self) -> &crate::redis::Redis {
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

    #[cfg(feature = "feat-redis")]
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
