use std::future::Future;
use std::pin::Pin;
use ntex::web;
use ntex::web::HttpRequest;
use crate::http::middlewares::executor::{MiddlewareExecutor};
use crate::results::AppResult;

mod executor;

pub type BeforeMiddlewareHandler =
fn(HttpRequest) -> Pin<Box<dyn Future<Output = AppResult<HttpRequest>>>>;

pub type AfterMiddlewareHandler =
fn(web::WebResponse) -> Pin<Box<dyn Future<Output = AppResult<web::WebResponse>>>>;

#[derive(Clone)]
pub enum Middleware {
    Before(BeforeMiddlewareHandler),
    After(AfterMiddlewareHandler),
}

impl Middleware {
    pub fn middleware(&self) -> MiddlewareExecutor {
        MiddlewareExecutor::new(self.clone())
    }
}
