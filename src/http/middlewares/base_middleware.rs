use log::{debug, error};
use ntex::service::{Middleware as NtexMiddleware, Service, ServiceCtx};
use ntex::web::{Error, ErrorRenderer, WebRequest, WebResponse};

use crate::http::middlewares::Middleware;

pub struct BaseMiddleware {
    pub middleware: Middleware,
}

impl BaseMiddleware {
    pub fn new(middleware: Middleware) -> BaseMiddleware {
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
    middleware: Middleware,
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

        match self.middleware {
            // execute before calling handler
            Middleware::Before(ref mid) => match mid.call(req).await {
                Ok(req) => {
                    let request = WebRequest::from_parts(req, payload).unwrap();
                    debug!("calling http controller -> method...");
                    ctx.call(&self.service, request).await
                }
                Err(err) => {
                    error!("[middleware-level-error][pre-exec] {:?}", err);
                    Err(Error::from(err))
                },
            },

            // execute after executing handler
            Middleware::After(ref mid) => {
                let request = WebRequest::from_parts(req, payload).unwrap();
                match ctx.call(&self.service, request).await {
                    Ok(resp) => match mid.call(resp).await {
                        Ok(resp) => Ok(resp),
                        Err(err) => Err(Error::from(err)),
                    },
                    Err(err) => {
                        error!("[middleware-level-error][post-exec] {:?}", err);
                        Err(err)
                    }
                }
            }
        }
    }
}
