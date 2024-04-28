use ntex::web;

use crate::app_setup::{get_server_host_config, load_environment_variables, make_app_state};
use crate::env_logger::init_env_logger;
use crate::http::kernel::{
    ntex_default_service, register_routes, setup_cors, setup_logger, Route,
};

pub async fn start_ntex_server(app: &str, routes: Vec<Route>) -> std::io::Result<()> {
    load_environment_variables(app);

    init_env_logger();

    let app_state = make_app_state().await;
    let (host, port, workers) = get_server_host_config();

    web::HttpServer::new(move || {
        let routes = routes.clone();
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
