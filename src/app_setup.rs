use std::path::Path;
#[allow(unused_imports)]
use std::sync::Arc;
use std::{env, fs};

#[cfg(feature = "mailer")]
use crate::app_state::AppMailerConfig;
use crate::app_state::{AppHelpers, AppServices, MedullahState};
#[cfg(feature = "database")]
use crate::database::DBPool;
#[cfg(feature = "jwt")]
use crate::helpers::jwt::Jwt;
#[cfg(feature = "crypto")]
use crate::helpers::password::Password;
use crate::http::Method;
#[cfg(feature = "rabbitmq")]
use crate::prelude::RabbitMQ;
#[cfg(feature = "redis")]
use crate::prelude::Redis;
#[cfg(feature = "rabbitmq")]
use crate::rabbitmq::conn::establish_rabbit_connection_pool;
#[cfg(feature = "redis")]
use crate::redis::conn::establish_redis_connection_pool;
#[cfg(feature = "redis")]
use crate::services::cache_service::CacheService;
use crate::MEDULLAH;
#[cfg(feature = "database")]
use diesel::r2d2::ConnectionManager;
#[cfg(feature = "database")]
use diesel::PgConnection;
use log::info;
#[cfg(feature = "templating")]
use tera::Tera;

pub struct MedullahSetup {
    pub env_prefix: String,
    pub private_key: String,
    pub public_key: String,
    pub auth_iss_public_key: String,
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<Method>,
}

pub async fn make_app_state(setup: MedullahSetup) -> MedullahState {
    let app = create_app_state(setup).await;
    MEDULLAH
        .set(app.clone())
        .expect("failed to set up medullah-web");
    app
}

async fn create_app_state(setup: MedullahSetup) -> MedullahState {
    let helpers = make_helpers(&setup.env_prefix, &setup);
    let env_prefix = setup.env_prefix;

    #[cfg(feature = "database")]
    let database_pool = establish_database_connection(&env_prefix);

    #[cfg(feature = "redis")]
    let redis_pool = establish_redis_connection_pool(&env_prefix);
    #[cfg(feature = "redis")]
    let redis = Arc::new(Redis::new(redis_pool.clone()));

    // RabbitMQ
    #[cfg(feature = "rabbitmq")]
    let rabbitmq_pool = establish_rabbit_connection_pool(&env_prefix).await;

    #[cfg(feature = "rabbitmq")]
    let rabbitmq = Arc::new(tokio::sync::Mutex::new(
        RabbitMQ::new(rabbitmq_pool.clone()).await.unwrap(),
    ));

    // templating
    #[cfg(feature = "templating")]
    let tera_templating = {
        let tpl_dir = crate::helpers::fs::get_cwd() + "/resources/templates/**/*.tera.html";
        Tera::new(tpl_dir.as_str()).unwrap()
    };

    MedullahState {
        helpers,

        app_id: env::var(format!("{}_APP_ID", env_prefix)).unwrap(),
        app_domain: env::var(format!("{}_APP_DOMAIN", env_prefix)).unwrap(),
        app_name: env::var(format!("{}_APP_NAME", env_prefix)).unwrap(),
        app_desc: env::var(format!("{}_APP_DESC", env_prefix)).unwrap(),
        app_help_email: env::var(format!("{}_APP_HELP_EMAIL", env_prefix)).unwrap(),
        app_frontend_url: env::var(format!("{}_FRONTEND_ADDRESS", env_prefix)).unwrap(),

        app_private_key: setup.private_key,
        app_public_key: setup.public_key,
        app_key: env::var(format!("{}_APP_KEY", env_prefix)).unwrap(),

        #[cfg(feature = "redis")]
        redis_pool,
        #[cfg(feature = "redis")]
        redis: redis.clone(),
        #[cfg(feature = "rabbitmq")]
        rabbitmq_pool,
        #[cfg(feature = "rabbitmq")]
        rabbitmq,
        #[cfg(feature = "database")]
        database: database_pool,
        #[cfg(feature = "templating")]
        tera: tera_templating,

        #[cfg(feature = "jwt")]
        auth_iss_public_key: setup.auth_iss_public_key,
        #[cfg(feature = "jwt")]
        auth_pat_prefix: env::var(format!("{}_AUTH_PAT_PREFIX", env_prefix)).unwrap(),
        #[cfg(feature = "jwt")]
        auth_token_lifetime: env::var(format!("{}_AUTH_TOKEN_LIFETIME", env_prefix))
            .unwrap()
            .parse()
            .unwrap(),

        allowed_origins: setup.allowed_origins,
        allowed_methods: setup.allowed_methods,

        #[cfg(feature = "mailer")]
        mailer_config: make_mailer_config(&env_prefix),

        services: AppServices {
            #[cfg(feature = "redis")]
            cache: Arc::new(CacheService::new(redis)),
        },

        app_env_prefix: env_prefix,
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

#[allow(unused_variables)]
fn make_helpers(env_prefix: &str, setup: &MedullahSetup) -> AppHelpers {
    #[cfg(feature = "crypto")]
    let app_key = env::var(format!("{}_APP_KEY", env_prefix)).unwrap();

    #[cfg(feature = "jwt")]
    let token_lifetime: i64 = env::var(format!("{}_AUTH_TOKEN_LIFETIME", env_prefix))
        .unwrap()
        .parse()
        .unwrap();

    AppHelpers {
        #[cfg(feature = "jwt")]
        jwt: Arc::new(Jwt::new(
            setup.auth_iss_public_key.clone(),
            setup.private_key.clone(),
            token_lifetime,
        )),
        #[cfg(feature = "crypto")]
        password: Arc::new(Password::new(app_key)),
    }
}

#[cfg(feature = "mailer")]
fn make_mailer_config(env_prefix: &str) -> AppMailerConfig {
    AppMailerConfig {
        from_name: env::var(format!("{}_MAIL_FROM_NAME", env_prefix)).unwrap(),
        from_email: env::var(format!("{}_MAIL_FROM_EMAIL", env_prefix)).unwrap(),
        server_endpoint: env::var(format!("{}_MAILER_SERVER_ENDPOINT", env_prefix)).unwrap(),
        server_auth_token: env::var(format!("{}_MAILER_SERVER_AUTH_TOKEN", env_prefix)).unwrap(),
        server_application_id: env::var(format!("{}_MAILER_SERVER_APPLICATION_ID", env_prefix))
            .unwrap(),
    }
}

#[cfg(feature = "database")]
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
