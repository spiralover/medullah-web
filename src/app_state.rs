use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use redis::Client as RedisClient;
use serde::Deserialize;
use tera::{Context, Tera};

use crate::redis::RedisPool;
use crate::services::cache_service::CacheService;
use crate::services::redis_service::RedisService;

#[derive(Clone)]
pub struct AppState {
    pub app_id: String,
    pub app_domain: String,
    pub app_name: String,
    pub app_desc: String,
    pub app_help_email: String,
    pub app_frontend_url: String,
    pub app_key: String,

    pub frontend: Frontend,

    #[cfg(feature = "feat-templating")]
    pub(crate) tera: Tera,

    pub(crate) redis: Arc<RedisClient>,
    pub(crate) redis_pool: Arc<RedisPool>,
    #[cfg(feature = "feat-database")]
    pub(crate) database: crate::database::DBPool,

    pub auth_pat_prefix: String,

    pub allowed_origins: Vec<String>,

    pub mailer_from_name: String,
    pub mailer_from_email: String,
    pub mailer_server_endpoint: String,
    pub mailer_server_auth_token: String,
    pub mailer_server_application_id: String,

    pub redis_queues: AppRedisQueues,
    pub redis_channels: AppRedisChannels,

    pub monnify_contract_code: String,
    pub monnify_api_key: String,
    pub monnify_secret_key: String,
    pub monnify_server_endpoint: String,

    pub services: AppServices,
}

#[derive(Clone)]
pub struct AppServices {
    pub redis: Arc<RedisService>,
    pub cache: Arc<CacheService>,
}

#[derive(Clone)]
pub struct AppRedisQueues {
    pub virtual_acc_num: String,
    pub announcement_un_synced: String,
}

#[derive(Clone)]
pub struct AppRedisChannels {
    pub payment_received: String
}

#[derive(Deserialize, Clone)]
pub struct Frontend {
    pub url: FrontendUrl,
}

#[derive(Deserialize, Clone)]
pub struct FrontendUrl {
    pub base: String,
    pub profile: String,
    pub login: String,
    pub register: String,
    pub forget_password: String,
    pub change_password: String,
}

impl AppState {
    #[cfg(feature = "feat-database")]
    pub fn database(&self) -> &crate::database::DBPool {
        &self.database
    }

    pub fn redis(&self) -> &RedisPool {
        &self.redis_pool
    }

    pub fn title(&self, text: &str) -> String {
        format!("{} - {}", text, self.app_name)
    }

    pub fn frontend(&self, url: &str) -> String {
        format!("{}/{}", self.app_frontend_url, url)
    }

    #[cfg(feature = "feat-templating")]
    pub fn render(&self, mut file: String, context: Context) -> String {
        if !file.ends_with(".tera.html") {
            file.push_str(".tera.html");
        }

        match self.tera.render(&file, &context) {
            Ok(string) => string,
            Err(error) => panic!("{}", error),
        }
    }
}

impl Debug for AppState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("application state")
    }
}
