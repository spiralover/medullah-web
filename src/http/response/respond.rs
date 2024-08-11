use crate::http::response::defs::{NtexBlockingResultResponder, ResultResponse};
use crate::prelude::{AppMessage, AppResult, HttpResult, IntoAppResult};
use ntex::http::error::BlockingError;
use serde::Serialize;

impl<T> NtexBlockingResultResponder for AppResult<T>
where
    T: Sized + Serialize,
{
    fn respond(self) -> HttpResult {
        self.send_result()
    }

    fn respond_msg(self, suc: &str) -> HttpResult {
        self.send_result_msg(suc)
    }
}

impl<T> NtexBlockingResultResponder for Result<T, BlockingError<AppMessage>>
where
    T: Serialize + Sized,
{
    fn respond(self) -> HttpResult {
        <Result<T, AppMessage> as ResultResponse>::send_result(self.into_app_result())
    }

    fn respond_msg(self, msg: &str) -> HttpResult {
        <Result<T, AppMessage> as ResultResponse>::send_result_msg(self.into_app_result(), msg)
    }
}

impl NtexBlockingResultResponder for Result<AppMessage, BlockingError<AppMessage>> {
    fn respond(self) -> HttpResult {
        <Result<AppMessage, AppMessage> as ResultResponse>::send_result(self.into_app_result())
    }

    fn respond_msg(self, msg: &str) -> HttpResult {
        <Result<AppMessage, AppMessage> as ResultResponse>::send_result_msg(
            self.into_app_result(),
            msg,
        )
    }
}
