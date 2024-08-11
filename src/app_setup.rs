use std::path::Path;
use std::sync::Arc;
use std::{env, fs};

#[cfg(feature = "feat-database")]
use diesel::r2d2::ConnectionManager;
#[cfg(feature = "feat-database")]
use diesel::PgConnection;
use log::info;
#[cfg(feature = "feat-templating")]
use tera::Tera;

use crate::app_state::{AppServices, MedullahState};
#[cfg(feature = "feat-database")]
use crate::database::DBPool;
use crate::helpers::fs::get_cwd;
#[cfg(feature = "feat-rabbitmq")]
use crate::rabbitmq::conn::establish_rabbit_connection;
use crate::redis::conn::{establish_redis_connection, establish_redis_connection_pool};
use crate::services::cache_service::CacheService;
#[cfg(feature = "feat-rabbitmq")]
use crate::services::rabbit_service::RabbitService;
use crate::services::redis_service::RedisService;
use crate::MEDULLAH;

pub struct MedullahSetup {
    pub env_prefix: String,
    pub private_key: String,
    pub public_key: String,
    pub auth_iss_public_key: String,
}

pub async fn make_app_state(setup: MedullahSetup) -> MedullahState {
    let app = create_app_state(setup).await;
    MEDULLAH.set(app.clone()).expect("failed to set up TSP");
    app
}

async fn create_app_state(setup: MedullahSetup) -> MedullahState {
    let env_prefix = setup.env_prefix;

    #[cfg(feature = "feat-database")]
    let database_pool = establish_database_connection(&env_prefix);

    let redis = establish_redis_connection(&env_prefix);
    let redis_pool = establish_redis_connection_pool(&env_prefix);
    let redis_service = Arc::new(RedisService::new(redis_pool.clone()));

    // RabbitMQ
    #[cfg(feature = "feat-rabbitmq")]
    let rabbit = Arc::new(establish_rabbit_connection(&env_prefix).await);

    // templating
    #[cfg(feature = "feat-templating")]
    let tera_templating = {
        let tpl_dir = get_cwd() + "/resources/templates/**/*.tera.html";
        Tera::new(tpl_dir.as_str()).unwrap()
    };

    MedullahState {
        app_id: env::var(format!("{}_APP_ID", env_prefix)).unwrap(),
        app_domain: env::var(format!("{}_APP_DOMAIN", env_prefix)).unwrap(),
        app_name: env::var(format!("{}_APP_NAME", env_prefix)).unwrap(),
        app_desc: env::var(format!("{}_APP_DESC", env_prefix)).unwrap(),
        app_help_email: env::var(format!("{}_APP_HELP_EMAIL", env_prefix)).unwrap(),
        app_frontend_url: env::var(format!("{}_FRONTEND_ADDRESS", env_prefix)).unwrap(),

        app_private_key: setup.private_key,
        app_public_key: setup.public_key,
        app_key: env::var(format!("{}_APP_KEY", env_prefix)).unwrap(),

        redis: Arc::new(redis),
        redis_pool: Arc::new(redis_pool),
        #[cfg(feature = "feat-rabbitmq")]
        rabbit: rabbit.clone(),
        #[cfg(feature = "feat-database")]
        database: database_pool,
        #[cfg(feature = "feat-templating")]
        tera: tera_templating,

        auth_iss_public_key: setup.auth_iss_public_key,
        auth_pat_prefix: env::var(format!("{}_AUTH_PAT_PREFIX", env_prefix)).unwrap(),
        auth_token_lifetime: env::var(format!("{}_AUTH_TOKEN_LIFETIME", env_prefix))
            .unwrap()
            .parse()
            .unwrap(),

        allowed_origins: get_allowed_origins(&env_prefix),

        // mail
        mailer_from_name: env::var(format!("{}_MAIL_FROM_NAME", env_prefix)).unwrap(),
        mailer_from_email: env::var(format!("{}_MAIL_FROM_EMAIL", env_prefix)).unwrap(),
        mailer_server_endpoint: env::var(format!("{}_MAILER_SERVER_ENDPOINT", env_prefix)).unwrap(),
        mailer_server_auth_token: env::var(format!("{}_MAILER_SERVER_AUTH_TOKEN", env_prefix))
            .unwrap(),
        mailer_server_application_id: env::var(format!(
            "{}_MAILER_SERVER_APPLICATION_ID",
            env_prefix
        ))
        .unwrap(),

        services: AppServices {
            redis: redis_service.clone(),
            cache: Arc::new(CacheService::new(redis_service)),
            #[cfg(feature = "feat-rabbitmq")]
            rabbitmq: Arc::from(RabbitService::new(rabbit.clone()).await),
        },
    }
}

pub fn get_server_host_config(env_prefix: &String) -> (String, u16, usize) {
    let host: String = env::var(format!("{}_SERVER_HOST", env_prefix)).unwrap();
    let port: u16 = env::var(format!("{}_SERVER_PORT", env_prefix))
        .unwrap()
        .parse()
        .unwrap();
    let workers: usize = env::var(format!("{}_SERVER_WORKERS", env_prefix))
        .unwrap()
        .parse()
        .unwrap();
    (host, port, workers)
}

pub fn get_allowed_origins(env_prefix: &String) -> Vec<String> {
    let url_str = env::var(format!("{}_ALLOWED_ORIGINS", env_prefix)).unwrap();
    let origins: Vec<&str> = url_str.split(',').collect();
    origins.iter().map(|o| o.trim().to_string()).collect()
}

#[cfg(feature = "feat-database")]
pub fn establish_database_connection(env_prefix: &String) -> DBPool {
    let db_url: String = env::var(format!("{}_DATABASE_DSN", env_prefix)).unwrap();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool.")
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
