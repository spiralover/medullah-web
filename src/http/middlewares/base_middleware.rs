use log::{debug, error};
use ntex::service::{Middleware as NtexMiddleware, Service, ServiceCtx};
use ntex::web::{Error, ErrorRenderer, HttpRequest, WebRequest, WebResponse};

use crate::enums::app_message::{get_middleware_level_message, get_status_code};
use crate::helpers::responder::Responder;
use crate::http::middlewares::Middleware;
use crate::prelude::AppMessage;

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
            Middleware::Before(ref mid) => match mid.call(&req).await {
                Ok(_) => {
                    let request = WebRequest::from_parts(req, payload).unwrap();
                    debug!("calling http controller -> method...");
                    ctx.call(&self.service, request).await
                }
                Err(err) => error_from_app_message(err, req),
            },

            // execute after executing handler
            Middleware::After(ref mid) => {
                let request = WebRequest::from_parts(req, payload).unwrap();
                match ctx.call(&self.service, request).await {
                    Ok(resp) => match mid.call(&resp).await {
                        Ok(_) => Ok(resp),
                        // log error and return response generated from controller
                        Err(err) => {
                            error!("[middleware-level-error][post-exec] {:?}", err);
                            Ok(resp)
                        }
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

fn error_from_app_message(message: AppMessage, req: HttpRequest) -> Result<WebResponse, Error> {
    let msg = get_middleware_level_message(&message);
    let resp = Responder::message(&msg, get_status_code(&message));
    error!("[middleware-level-error][pre-exec] {:?}", message);
    Ok(WebResponse::new(resp, req))
}
