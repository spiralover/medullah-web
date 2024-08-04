use dyn_clone::DynClone;
use std::future::Future;
use std::pin::Pin;

use ntex::web::{HttpRequest, WebResponse};

use crate::results::AppResult;

pub mod base_middleware;

pub type BeforeMiddlewareReturn<'a> = Pin<Box<dyn Future<Output = AppResult<()>> + 'a>>;
pub type AfterMiddlewareReturn<'a> = Pin<Box<dyn Future<Output = AppResult<()>> + 'a>>;

pub trait BeforeMiddleware: DynClone {
    fn call<'a>(&'a self, request: &'a HttpRequest) -> BeforeMiddlewareReturn<'a>;
}

pub trait AfterMiddleware: DynClone {
    fn call<'a>(&'a self, resp: &'a WebResponse) -> AfterMiddlewareReturn<'a>;
}

#[derive(Clone)]
pub enum Middleware {
    Before(Box<dyn BeforeMiddleware + Send>),
    After(Box<dyn AfterMiddleware + Send>),
}

dyn_clone::clone_trait_object!(BeforeMiddleware);
dyn_clone::clone_trait_object!(AfterMiddleware);
