use ntex::web::ServiceConfig;

pub mod extractors;
pub mod kernel;
pub mod middlewares;
pub mod response;
pub mod server;

pub use ntex::http::Method;
pub use ntex_cors::Cors;

pub type HttpHandler = fn(cfg: &mut ServiceConfig);
