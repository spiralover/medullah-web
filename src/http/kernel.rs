use ntex::http::{header, StatusCode};
use ntex::web::middleware::Logger;
use ntex::web::ServiceConfig;
use ntex::{web, web::Route as NtexRoute};
use ntex_cors::Cors;
use std::rc::Rc;

use crate::helpers::responder::json_error_message_status;
use crate::http::middlewares::base_middleware::BaseMiddleware;

use crate::http::middlewares::Middleware;

#[derive(Clone)]
pub struct Controller {
    pub path: String,
    pub handler: fn(cfg: &mut ServiceConfig),
}

#[derive(Clone)]
pub struct Route {
    pub prefix: String,
    pub middlewares: Vec<Middleware>,
    pub controllers: Vec<Controller>,
}

pub fn register_routes(config: &mut ServiceConfig, routes: Vec<Route>) {
    log::debug!("discovering routes...");

    for route in routes.clone() {
        let middlewares: Vec<Rc<Middleware>> = route
            .middlewares
            .clone()
            .iter()
            .map(|m| Rc::new(m.to_owned()))
            .collect();

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
                        .wrap(BaseMiddleware::new(middlewares.first().unwrap().clone()))
                        .configure(controller.handler);
                    config.service(scope);
                } else if total == 2 {
                    let scope = web::scope(path.as_str())
                        .wrap(BaseMiddleware::new(middlewares.first().unwrap().clone()))
                        .wrap(BaseMiddleware::new(middlewares.last().unwrap().clone()))
                        .configure(controller.handler);
                    config.service(scope);
                } else {
                    let scope = web::scope(path.as_str())
                        .wrap(BaseMiddleware::new(middlewares.first().unwrap().clone()))
                        .wrap(BaseMiddleware::new(middlewares.get(1).unwrap().clone()))
                        .wrap(BaseMiddleware::new(middlewares.last().unwrap().clone()))
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
    Logger::new("%{r}a \"%r7\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T")
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
        json_error_message_status("Requested Resource(s) Not Found", StatusCode::NOT_FOUND)
    })
}

pub fn register_middlewares(_config: &mut ServiceConfig) {
    // for middleware in middlewares() {
    // }
}
