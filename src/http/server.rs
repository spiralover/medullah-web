use std::future::Future;

use log::error;
use ntex::web;

use crate::app_setup::{
    get_server_host_config, load_environment_variables, make_app_state, MedullahSetup,
};
use crate::env_logger::init_env_logger;
use crate::http::kernel::{ntex_default_service, register_routes, Route, setup_cors, setup_logger};
use crate::prelude::{AppResult, MedullahState};

pub struct ServerConfig {
    pub app: String,
    pub routes: Vec<Route>,
    pub env_prefix: String,
    pub private_key: String,
    pub public_key: String,
    pub auth_iss_public_key: String,
    #[cfg(feature = "feat-static")]
    pub static_config: StaticFileConfig,
}

#[cfg(feature = "feat-static")]
pub struct StaticFileConfig {
    pub path: String,
    pub dir: String,
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

    let app_state = make_app_state(MedullahSetup {
        public_key: config.public_key,
        private_key: config.private_key,
        env_prefix: config.env_prefix.clone(),
        auth_iss_public_key: config.auth_iss_public_key,
    })
    .await;

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
        let app = web::App::new()
            .state(app_state.clone())
            .configure(|cfg| register_routes(cfg, routes))
            .wrap(setup_logger())
            .wrap(setup_cors(app_state.allowed_origins.clone()).finish())
            .default_service(ntex_default_service());

        if cfg!(feature = "feat-static") {
            #[cfg(feature = "feat-static")]
            {
                return app.service(ntex_files::Files::new(
                    &config.static_config.path,
                    &config.static_config.dir,
                ));
            }
        }

        app
    })
    .bind((host, port))?
    .workers(workers)
    .run()
    .await
}
