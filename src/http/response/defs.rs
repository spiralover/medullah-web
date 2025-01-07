use crate::contracts::ResponseCodeContract;
use crate::prelude::{AppMessage, HttpResult};
use ntex::web::HttpResponse;

pub trait ResultResponse {
    fn send_result<C: ResponseCodeContract>(self, code: C) -> HttpResult;

    fn send_result_msg<C: ResponseCodeContract>(self, code: C, msg: &str) -> HttpResult;
}

pub trait NtexBlockingResultResponder {
    fn respond_code<C: ResponseCodeContract>(self, msg: &str, code: C) -> HttpResult;

    fn respond_msg(self, suc: &str) -> HttpResult;

    fn respond(self) -> HttpResult;
}

pub trait StructResponse: Sized {
    fn into_response(self) -> HttpResponse;

    fn respond_code<C: ResponseCodeContract>(
        self,
        code: C,
        msg: &str,
    ) -> Result<HttpResponse, AppMessage>;

    fn respond_msg(self, msg: &str) -> Result<HttpResponse, AppMessage>;

    fn respond(self) -> Result<HttpResponse, AppMessage>;
}

pub trait OptionResultResponse<T> {
    fn is_empty(&self) -> bool;

    fn is_error(&self) -> bool;

    fn is_error_or_empty(&self) -> bool;

    fn send_response<C: ResponseCodeContract>(self, code: C, msg: &str) -> HttpResult;
}
