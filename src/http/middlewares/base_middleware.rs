use log::debug;
use ntex::service::{Middleware as NtexMiddleware, Service, ServiceCtx};
use ntex::web::{Error, ErrorRenderer, WebRequest, WebResponse};
use std::rc::Rc;

use crate::http::middlewares::Middleware;

pub struct BaseMiddleware {
    pub middleware: Rc<Middleware>,
}

impl BaseMiddleware {
    pub fn new(middleware: Rc<Middleware>) -> BaseMiddleware {
        Self { middleware }
    }
}

impl<S> NtexMiddleware<S> for BaseMiddleware {
    type Service = BaseMiddlewareInternal<S>;

    fn create(&self, service: S) -> Self::Service {
        BaseMiddlewareInternal {
            service,
            middleware: self.middleware.clone(),
        }
    }
}

pub struct BaseMiddlewareInternal<S> {
    service: S,
    middleware: Rc<Middleware>,
}

impl<S, Err> Service<WebRequest<Err>> for BaseMiddlewareInternal<S>
where
    S: Service<WebRequest<Err>, Response = WebResponse, Error = Error>,
    Err: ErrorRenderer,
{
    type Response = WebResponse;
    type Error = Error;

    ntex::forward_ready!(service);
    ntex::forward_shutdown!(service);

    async fn call(
        &self,
        request: WebRequest<Err>,
        ctx: ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> {
        let (req, payload) = request.into_parts();

        match *self.middleware {
            // execute before calling handler
            Middleware::Before(ref mid) => match mid.call(req).await {
                Ok(req) => {
                    let request = WebRequest::from_parts(req, payload).unwrap();
                    debug!("calling http controller -> method...");
                    ctx.call(&self.service, request).await
                }
                Err(err) => Err(Error::from(err)),
            },

            // execute after executing handler
            Middleware::After(ref mid) => {
                let request = WebRequest::from_parts(req, payload).unwrap();
                match ctx.call(&self.service, request).await {
                    Ok(resp) => match mid.call(resp).await {
                        Ok(resp) => Ok(resp),
                        Err(err) => Err(Error::from(err)),
                    },
                    Err(err) => Err(err),
                }
            }
        }
    }
}
