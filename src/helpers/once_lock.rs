use std::sync::{Arc, OnceLock};

#[cfg(feature = "feat-database")]
use diesel::r2d2::ConnectionManager;
#[cfg(feature = "feat-database")]
use diesel::PgConnection;
use redis::Client;

use crate::app_state::{AppRedisQueues, AppState, Frontend};
#[cfg(feature = "feat-database")]
use crate::database::DatabaseConnectionHelper;
use crate::redis::RedisPool;
use crate::services::cache_service::CacheService;
use crate::services::redis_service::RedisService;
use crate::APP;

pub trait OnceLockHelper<'a> {
    fn app(&self) -> &'a AppState {
        APP.get().unwrap()
    }

    fn frontend(&self) -> &'a Frontend {
        &self.app().frontend
    }

    fn front_url(&self, url: &str) -> String {
        self.app().frontend(url)
    }

    #[cfg(feature = "feat-database")]
    fn database(&self) -> &'a crate::database::DBPool {
        APP.get().unwrap().database()
    }

    fn redis(&self) -> Arc<Client> {
        Arc::clone(&self.app().redis)
    }

    fn redis_pool(&self) -> Arc<RedisPool> {
        Arc::clone(&self.app().redis_pool)
    }

    fn cache(&self) -> &CacheService {
        &APP.get().unwrap().services.cache
    }

    fn redis_service(&self) -> &RedisService {
        &APP.get().unwrap().services.redis
    }

    fn redis_queues(&self) -> &AppRedisQueues {
        &APP.get().unwrap().redis_queues
    }

    #[cfg(feature = "feat-database")]
    fn db(
        &self,
    ) -> crate::prelude::AppResult<r2d2::PooledConnection<ConnectionManager<PgConnection>>> {
        self.database().connection()
    }
}

impl<'a> OnceLockHelper<'a> for OnceLock<AppState> {}
