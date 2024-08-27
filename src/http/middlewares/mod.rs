use std::future::Future;
use std::pin::Pin;
use ntex::web::{HttpRequest, WebResponse};
use crate::http::middlewares::executor::{MiddlewareExecutor};
use crate::results::AppResult;

mod executor;

pub type BeforeMiddlewareHandler =
fn(HttpRequest) -> Pin<Box<dyn Future<Output = AppResult<HttpRequest>>>>;

pub type AfterMiddlewareHandler =
fn(WebResponse) -> Pin<Box<dyn Future<Output = AppResult<WebResponse>>>>;

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
