use ntex::http::{header, StatusCode};
use ntex::web::middleware::Logger;
use ntex::web::ServiceConfig;
use ntex::{web, web::Route as NtexRoute};
use ntex_cors::Cors;

use crate::helpers::responder::Responder;
use crate::http::middlewares::Middleware;

pub struct Controller {
    pub path: String,
    pub handler: fn(cfg: &mut ServiceConfig),
}

pub struct Route {
    pub prefix: String,
    pub middlewares: Vec<Middleware>,
    pub controllers: Vec<Controller>,
}

pub fn register_routes(config: &mut ServiceConfig, routes: Vec<Route>) {
    log::debug!("discovering routes...");

    for route in routes {
        for controller in &route.controllers {
            let path = route.prefix.as_str().to_owned() + controller.path.as_str();
            log::debug!(
                "route group: {}",
                if path.is_empty() { "/" } else { path.as_str() }
            );

            if path.is_empty() {
                config.service(web::scope("").configure(controller.handler));
            } else if !route.middlewares.is_empty() {
                let total = route.middlewares.len();

                if total == 1 {
                    let scope = web::scope(path.as_str())
                        .wrap(route.middlewares.first().unwrap().middleware())
                        .configure(controller.handler);
                    config.service(scope);
                } else if total == 2 {
                    let scope = web::scope(path.as_str())
                        .wrap(route.middlewares.first().unwrap().middleware())
                        .wrap(route.middlewares.last().unwrap().middleware())
                        .configure(controller.handler);
                    config.service(scope);
                } else {
                    let scope = web::scope(path.as_str())
                        .wrap(route.middlewares.first().unwrap().middleware())
                        .wrap(route.middlewares.get(1).unwrap().middleware())
                        .wrap(route.middlewares.last().unwrap().middleware())
                        .configure(controller.handler);
                    config.service(scope);
                }
            } else {
                config.service(web::scope(path.as_str()).configure(controller.handler));
            }
        }
    }

    log::debug!("route discovery finished :)");
}

pub fn setup_logger() -> Logger {
    Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T")
        .exclude("/favicon.ico")
        .exclude("/system/docker-health-check")
}

pub fn setup_cors(origins: Vec<String>) -> Cors {
    let mut cors = Cors::new();

    for origin in origins {
        cors = cors.allowed_origin(origin.as_str());
    }

    cors.allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600)
}

pub fn ntex_default_service() -> NtexRoute {
    web::to(|| async {
        Responder::message("Requested Resource(s) Not Found", StatusCode::NOT_FOUND)
    })
}

pub fn register_middlewares(_config: &mut ServiceConfig) {
    // for middleware in middlewares() {
    // }
}
