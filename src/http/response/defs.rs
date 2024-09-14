use crate::prelude::{AppMessage, AppResult, HttpResult};
use ntex::http::StatusCode;
use ntex::web::HttpResponse;

pub enum Return<T> {
    Ok(T, &'static str),
    BadRequest(T, &'static str),
    Response(T, &'static str, StatusCode),
    Message(AppMessage),
}

impl<T> Return<T> {
    pub fn msg(msg: AppMessage) -> Return<T> {
        Return::Message(msg)
    }
}

pub trait ResultResponse {
    fn send_result(self) -> HttpResult;

    fn send_result_msg(self, msg: &str) -> HttpResult;
}

pub trait NtexBlockingResultResponder {
    fn respond(self) -> HttpResult;

    fn respond_msg(self, suc: &str) -> HttpResult;
}

pub trait MappableResponse<T>: Sized {
    fn respond_map<Func>(self, func: Func) -> HttpResult
    where
        Func: FnOnce(T) -> Return<T>;

    fn respond_map_any<Func>(self, map: Func) -> HttpResult
    where
        Func: FnOnce(Self) -> Return<T>;
}

pub trait StructResponse: Sized {
    fn into_response(self) -> HttpResponse;

    fn respond(self) -> Result<HttpResponse, AppMessage>;

    fn respond_msg(self, suc: &str) -> Result<HttpResponse, AppMessage>;
}

pub trait OptionResultResponse<T> {
    fn is_empty(&self) -> bool;

    fn is_error_or_empty(&self) -> bool;

    fn get_error_result(self) -> AppResult<T>;

    fn send_error(self) -> HttpResult;

    fn send_entity(self) -> HttpResult;

    fn send_response(self) -> HttpResult;
}
