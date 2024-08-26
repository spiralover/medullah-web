use ntex::web::ServiceConfig;

pub mod extractors;
pub mod kernel;
pub mod middlewares;
pub mod response;
pub mod server;

pub type HttpHandler = fn(cfg: &mut ServiceConfig);
