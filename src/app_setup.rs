use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use std::{env, fs};

#[cfg(feature = "feat-database")]
use diesel::r2d2::ConnectionManager;
#[cfg(feature = "feat-database")]
use diesel::PgConnection;
use log::info;
use mobc::Pool;
use redis::Client;
use tera::Tera;

use crate::app_state::{AppRedisChannels, AppRedisQueues, AppServices, AppState, Frontend};
#[cfg(feature = "feat-database")]
use crate::database::DBPool;
use crate::helpers::fs::get_cwd;
use crate::redis::{RedisConnectionManager, RedisPool};
use crate::services::cache_service::CacheService;
use crate::services::redis_service::RedisService;
use crate::APP;

const CACHE_POOL_MAX_OPEN: u64 = 16;
const CACHE_POOL_MAX_IDLE: u64 = 8;
const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

pub async fn make_app_state() -> AppState {
    let app = create_app_state().await;
    APP.set(app.clone()).expect("failed to set up TSP");
    app
}

async fn create_app_state() -> AppState {
    #[cfg(feature = "feat-database")]
    let database_pool = establish_database_connection();

    let redis = establish_redis_connection();
    let redis_pool = establish_redis_connection_pool();
    let redis_service = Arc::new(RedisService::new(redis_pool.clone()));

    // templating
    let tpl_dir = get_cwd() + "/resources/templates/**/*.tera.html";
    let tera_templating = Tera::new(tpl_dir.as_str()).unwrap();

    AppState {
        app_id: env::var("SPACE_APP_ID").unwrap(),
        app_domain: env::var("SPACE_APP_DOMAIN").unwrap(),
        app_name: env::var("SPACE_APP_NAME").unwrap(),
        app_desc: env::var("SPACE_APP_DESC").unwrap(),
        app_key: env::var("SPACE_APP_KEY").unwrap(),
        app_help_email: env::var("SPACE_APP_HELP_EMAIL").unwrap(),
        app_frontend_url: env::var("SPACE_FRONTEND_ADDRESS").unwrap(),

        frontend: load_frontend_config(),

        redis: Arc::new(redis),
        redis_pool: Arc::new(redis_pool),
        #[cfg(feature = "feat-database")]
        database: database_pool,
        tera: tera_templating,

        auth_pat_prefix: env::var("SPACE_AUTH_PAT_PREFIX").unwrap(),

        allowed_origins: get_allowed_origins(),

        // mail
        mailer_from_name: env::var("SPACE_MAIL_FROM_NAME").unwrap(),
        mailer_from_email: env::var("SPACE_MAIL_FROM_EMAIL").unwrap(),
        mailer_server_endpoint: env::var("SPACE_MAILER_SERVER_ENDPOINT").unwrap(),
        mailer_server_auth_token: env::var("SPACE_MAILER_SERVER_AUTH_TOKEN").unwrap(),
        mailer_server_application_id: env::var("SPACE_MAILER_SERVER_APPLICATION_ID").unwrap(),

        monnify_api_key: env::var("SPACE_MONNIFY_API_KEY").unwrap(),
        monnify_secret_key: env::var("SPACE_MONNIFY_SECRET_KEY").unwrap(),
        monnify_contract_code: env::var("SPACE_MONNIFY_CONTRACT_CODE").unwrap(),
        monnify_server_endpoint: env::var("SPACE_MONNIFY_SERVER_ENDPOINT").unwrap(),

        redis_queues: get_redis_queues(),
        redis_channels: get_redis_channels(),

        services: AppServices {
            redis: redis_service.clone(),
            cache: Arc::new(CacheService::new(redis_service)),
        },
    }
}

pub fn get_server_host_config() -> (String, u16, usize) {
    let host: String = env::var("SPACE_SERVER_HOST").unwrap();
    let port: u16 = env::var("SPACE_SERVER_PORT").unwrap().parse().unwrap();
    let workers: usize = env::var("SPACE_SERVER_WORKERS").unwrap().parse().unwrap();
    (host, port, workers)
}

pub fn establish_redis_connection() -> Client {
    let redis_url: String = env::var("SPACE_REDIS_DSN").unwrap();
    Client::open(redis_url).unwrap()
}

pub fn establish_redis_connection_pool() -> RedisPool {
    let redis_url: String = env::var("SPACE_REDIS_DSN").unwrap();
    let client = Client::open(redis_url).unwrap();
    let manager = RedisConnectionManager::new(client);
    Pool::builder()
        .get_timeout(Some(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS)))
        .max_open(CACHE_POOL_MAX_OPEN)
        .max_idle(CACHE_POOL_MAX_IDLE)
        .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
        .build(manager)
}

#[cfg(feature = "feat-database")]
pub fn establish_database_connection() -> DBPool {
    let db_url: String = env::var("SPACE_DATABASE_DSN").unwrap();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool.")
}

pub fn get_redis_queues() -> AppRedisQueues {
    AppRedisQueues {
        virtual_acc_num: env::var("SPACE_REDIS_QUEUE_VIRTUAL_ACC_NUM").unwrap(),
        announcement_un_synced: env::var("GENESIS_REDIS_QUEUE_ANNOUNCEMENT_UN_SYNCED").unwrap(),
    }
}

pub fn get_redis_channels() -> AppRedisChannels {
    AppRedisChannels {
        payment_received: env::var("SPACE_REDIS_CHANNEL_TRANSFER_RECEIVED").unwrap()
    }
}

pub fn get_allowed_origins() -> Vec<String> {
    let url_str = env::var("SPACE_ALLOWED_ORIGINS").unwrap();
    let origins: Vec<&str> = url_str.split(',').collect();
    origins.iter().map(|o| o.trim().to_string()).collect()
}

pub fn load_frontend_config() -> Frontend {
    let file_contents = load_config_file("frontend.toml");
    toml::from_str::<Frontend>(&file_contents).unwrap()
}

pub fn load_config_file(file: &str) -> String {
    fs::read_to_string(format!("resources/config/{}", file)).unwrap()
}

pub fn load_environment_variables(service: &str) {
    info!(
        "log level: {:?}",
        env::var("RUST_LOG").unwrap_or(String::from("info"))
    );
    info!("root directory: {:?}", service);

    // load project level .env
    let path = format!("apps/{}/.env", service);
    dotenv::from_filename(path).ok();

    // load project level .env.main
    if Path::new(".env").exists() {
        info!("loading env file: .env");
        dotenv::from_filename(".env").ok();
    }

    // load project level .env.main
    if Path::new(".env.main").exists() {
        info!("loading env file: .env.main");
        dotenv::from_filename(".env.main").ok();
    }

    // load project level .env.main
    let filename = format!(".env.{}", service);
    if Path::new(filename.as_str()).exists() {
        info!("loading env file: {}", filename);
        dotenv::from_filename(filename).ok();
    }
}
