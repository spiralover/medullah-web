use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use redis::Client as RedisClient;
#[cfg(feature = "feat-templating")]
use tera::{Context, Tera};
use crate::helpers::hmac::Hmac;
use crate::helpers::jwt::Jwt;
#[cfg(feature = "feat-rabbitmq")]
use crate::rabbitmq::RabbitMQ;
use crate::redis::Redis;
use crate::services::cache_service::CacheService;

#[derive(Clone)]
pub struct MedullahState {
    pub app_id: String,
    pub app_domain: String,
    pub app_name: String,
    pub app_desc: String,
    pub app_help_email: String,
    pub app_frontend_url: String,
    pub app_key: String,
    pub app_private_key: String,
    pub app_public_key: String,

    #[cfg(feature = "feat-templating")]
    pub(crate) tera: Tera,

    pub(crate) redis_client: Arc<RedisClient>,
    pub(crate) redis_pool: deadpool_redis::Pool,
    pub(crate) redis: Arc<Redis>,
    #[cfg(feature = "feat-rabbitmq")]
    pub rabbitmq_client: Arc<lapin::Connection>,
    #[cfg(feature = "feat-rabbitmq")]
    pub rabbitmq_pool: deadpool_lapin::Pool,
    #[cfg(feature = "feat-rabbitmq")]
    pub rabbitmq: Arc<RabbitMQ>,
    #[cfg(feature = "feat-database")]
    pub(crate) database: crate::database::DBPool,

    /// personal access token prefix
    pub auth_pat_prefix: String,

    /// authentication token lifetime (in minutes)
    pub auth_token_lifetime: i64,

    /// authentication issuer public key
    pub auth_iss_public_key: String,

    /// list of comma-separated allowed origins
    pub allowed_origins: Vec<String>,

    #[cfg(feature = "feat-mailer")]
    pub mailer_config: AppMailerConfig,

    pub services: AppServices,

    pub helpers: AppHelpers,
}

#[derive(Clone)]
pub struct AppMailerConfig {
    pub from_name: String,
    pub from_email: String,
    pub server_endpoint: String,
    pub server_auth_token: String,
    pub server_application_id: String,
}

#[derive(Clone)]
pub struct AppHelpers {
    pub jwt: Jwt,
}

#[derive(Clone)]
pub struct AppServices {
    pub cache: Arc<CacheService>,
}

impl MedullahState {
    #[cfg(feature = "feat-database")]
    pub fn database(&self) -> &crate::database::DBPool {
        &self.database
    }

    pub fn redis(&self) -> Arc<Redis> {
        self.redis.clone()
    }

    #[cfg(feature = "feat-rabbitmq")]
    pub fn rabbitmq(&self) -> Arc<RabbitMQ> {
        self.rabbitmq.clone()
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

impl Debug for MedullahState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("application state")
    }
}
