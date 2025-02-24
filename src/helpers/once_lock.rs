#[allow(unused_imports)]
use std::sync::{Arc, OnceLock};

use crate::app_state::{AppHelpers, MedullahState};
#[cfg(feature = "database")]
use crate::database::DatabaseConnectionHelper;
#[cfg(feature = "redis")]
use crate::services::cache_service::CacheService;
use crate::MEDULLAH;
#[cfg(feature = "database")]
use diesel::r2d2::ConnectionManager;
#[cfg(feature = "database")]
use diesel::PgConnection;

pub trait OnceLockHelper {
    fn app(&self) -> &MedullahState {
        MEDULLAH.get().unwrap()
    }

    fn helpers(&self) -> &AppHelpers {
        &MEDULLAH.get().unwrap().helpers
    }

    fn front_url(&self, url: &str) -> String {
        self.app().frontend(url)
    }

    #[cfg(feature = "database")]
    fn database(&self) -> &crate::database::DBPool {
        MEDULLAH.get().unwrap().database()
    }

    #[cfg(feature = "redis")]
    fn redis_pool(&self) -> deadpool_redis::Pool {
        self.app().redis_pool.clone()
    }

    #[cfg(feature = "redis")]
    fn redis(&self) -> &crate::redis::Redis {
        &MEDULLAH.get().unwrap().redis
    }

    #[cfg(feature = "rabbitmq")]
    fn rabbitmq_pool(&self) -> deadpool_lapin::Pool {
        self.app().rabbitmq_pool.clone()
    }

    #[cfg(feature = "rabbitmq")]
    fn rabbitmq(&self) -> Arc<tokio::sync::Mutex<crate::prelude::RabbitMQ>> {
        Arc::clone(&self.app().rabbitmq)
    }

    #[cfg(feature = "redis")]
    fn cache(&self) -> &CacheService {
        &MEDULLAH.get().unwrap().services.cache
    }

    #[cfg(feature = "database")]
    fn db(
        &self,
    ) -> crate::prelude::AppResult<r2d2::PooledConnection<ConnectionManager<PgConnection>>> {
        self.database().connection()
    }
}

impl OnceLockHelper for OnceLock<MedullahState> {}
