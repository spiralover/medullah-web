use crate::http::middlewares::Middleware;
use log::{debug, error, info};
use ntex::service::{Middleware as ServiceMiddleware, Service, ServiceCtx};
use ntex::web;
use ntex::web::{Error, WebRequest};

#[derive(Clone)]
pub struct MiddlewareExecutor {
    handler: Middleware,
}

impl MiddlewareExecutor {
    pub fn new(handler: Middleware) -> Self {
        MiddlewareExecutor { handler }
    }
}

impl<S> ServiceMiddleware<S> for MiddlewareExecutor {
    type Service = SayHiMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        SayHiMiddleware {
            service,
            middleware: self.handler.clone(),
        }
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
    middleware: Middleware,
}

impl<S, Err> Service<web::WebRequest<Err>> for SayHiMiddleware<S>
where
    S: Service<web::WebRequest<Err>, Response = web::WebResponse, Error = web::Error>,
    Err: web::ErrorRenderer,
{
    type Response = web::WebResponse;
    type Error = web::Error;

    ntex::forward_ready!(service);

    async fn call(
        &self,
        request: web::WebRequest<Err>,
        ctx: ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> {
        let (req, payload) = request.into_parts();
        info!("{} {}", req.method(), req.path());

        match self.middleware {
            // execute before calling handler
            Middleware::Before(ref mid) => match mid(req).await {
                Ok(req) => {
                    let request = WebRequest::from_parts(req, payload).unwrap();
                    debug!("calling http controller -> method...");
                    ctx.call(&self.service, request).await
                }
                Err(err) => Err(Error::new(err)),
            },

            // execute after executing handler
            Middleware::After(ref mid) => {
                let request = WebRequest::from_parts(req, payload).unwrap();
                match ctx.call(&self.service, request).await {
                    Ok(resp) => match mid(resp).await {
                        Ok(resp) => Ok(resp),
                        // log error and return response generated from controller
                        Err(err) => {
                            error!("[middleware-level-error][post-exec] {:?}", err);
                            Err(Error::new(err))
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
