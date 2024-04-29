use std::future::Future;

use log::error;
use ntex::web;

use crate::app_setup::{get_server_host_config, load_environment_variables, make_app_state};
use crate::env_logger::init_env_logger;
use crate::http::kernel::{ntex_default_service, register_routes, Route, setup_cors, setup_logger};
use crate::prelude::{AppResult, MedullahState};

pub struct ServerConfig {
    pub app: String,
    pub env_prefix: String,
    pub routes: Vec<Route>,
}

pub async fn start_ntex_server<Callback, Fut>(
    config: ServerConfig,
    callback: Callback,
) -> std::io::Result<()>
where
    Callback: FnOnce(MedullahState) -> Fut + Copy + Send + 'static,
    Fut: Future<Output = AppResult<()>> + Send + 'static,
{
    load_environment_variables(&config.app);

    init_env_logger();

    let app_state = make_app_state(config.env_prefix.clone()).await;
    let (host, port, workers) = get_server_host_config(&config.env_prefix);

    match callback(app_state.clone()).await {
        Ok(_) => {}
        Err(err) => {
            error!("app bootstrap callback returned error: {:?}", err);
            panic!("boostrap failed");
        }
    }

    web::HttpServer::new(move || {
        let routes = config.routes.clone();
        web::App::new()
            .state(app_state.clone())
            .configure(|cfg| register_routes(cfg, routes))
            .wrap(setup_logger())
            .wrap(setup_cors(app_state.allowed_origins.clone()).finish())
            .default_service(ntex_default_service())
    })
    .bind((host, port))?
    .workers(workers)
    .run()
    .await
}
