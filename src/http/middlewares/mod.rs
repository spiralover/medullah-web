use dyn_clone::DynClone;
use std::future::Future;
use std::pin::Pin;

use ntex::web::{HttpRequest, WebResponse};

use crate::results::AppResult;

pub mod base_middleware;

pub type BeforeMiddlewareReturn<'a> = Pin<Box<dyn Future<Output = AppResult<HttpRequest>> + 'a>>;
pub type AfterMiddlewareReturn<'a> = Pin<Box<dyn Future<Output = AppResult<WebResponse>> + 'a>>;

pub trait BeforeMiddleware: DynClone {
    fn call(&self, req: HttpRequest) -> BeforeMiddlewareReturn;
}

pub trait AfterMiddleware: DynClone {
    fn call(&self, resp: WebResponse) -> AfterMiddlewareReturn;
}

#[derive(Clone)]
pub enum Middleware {
    Before(Box<dyn BeforeMiddleware + Send>),
    After(Box<dyn AfterMiddleware + Send>),
}

dyn_clone::clone_trait_object!(BeforeMiddleware);
dyn_clone::clone_trait_object!(AfterMiddleware);
