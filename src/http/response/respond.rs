use crate::contracts::ResponseCodeContract;
use crate::enums::ResponseCode;
use crate::http::response::defs::{NtexBlockingResultResponder, ResultResponse};
use crate::prelude::IntoAppResult;
use crate::prelude::{AppMessage, AppResult, HttpResult};
use ntex::http::error::BlockingError;
use serde::Serialize;

impl<T> NtexBlockingResultResponder for AppResult<T>
where
    T: Sized + Serialize,
{
    fn respond_code<C: ResponseCodeContract>(self, msg: &str, code: C) -> HttpResult {
        self.send_result_msg(code, msg)
    }

    fn respond_msg(self, msg: &str) -> HttpResult {
        self.send_result_msg(ResponseCode::Ok, msg)
    }

    fn respond(self) -> HttpResult {
        self.send_result(ResponseCode::Ok)
    }
}

impl<T> NtexBlockingResultResponder for Result<T, BlockingError<AppMessage>>
where
    T: Serialize + Sized,
{
    fn respond_code<C: ResponseCodeContract>(self, msg: &str, code: C) -> HttpResult {
        <Result<T, AppMessage> as ResultResponse>::send_result_msg(
            self.into_app_result(),
            code,
            msg,
        )
    }

    fn respond_msg(self, msg: &str) -> HttpResult {
        <Result<T, AppMessage> as ResultResponse>::send_result_msg(
            self.into_app_result(),
            ResponseCode::Ok,
            msg,
        )
    }

    fn respond(self) -> HttpResult {
        <Result<T, AppMessage> as ResultResponse>::send_result(
            self.into_app_result(),
            ResponseCode::Ok,
        )
    }
}

impl NtexBlockingResultResponder for Result<AppMessage, AppMessage> {
    fn respond_code<C: ResponseCodeContract>(self, msg: &str, code: C) -> HttpResult {
        self.send_result_msg(code.clone(), msg)
    }

    fn respond_msg(self, msg: &str) -> HttpResult {
        self.send_result_msg(ResponseCode::Ok, msg)
    }

    fn respond(self) -> HttpResult {
        self.send_result(ResponseCode::Ok)
    }
}

impl NtexBlockingResultResponder for Result<AppMessage, BlockingError<AppMessage>> {
    fn respond_code<C: ResponseCodeContract>(self, msg: &str, code: C) -> HttpResult {
        <Result<AppMessage, AppMessage> as ResultResponse>::send_result_msg(
            self.into_app_result(),
            code,
            msg,
        )
    }

    fn respond_msg(self, msg: &str) -> HttpResult {
        <Result<AppMessage, AppMessage> as ResultResponse>::send_result_msg(
            self.into_app_result(),
            ResponseCode::Ok,
            msg,
        )
    }

    fn respond(self) -> HttpResult {
        <Result<AppMessage, AppMessage> as ResultResponse>::send_result(
            self.into_app_result(),
            ResponseCode::Ok,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::json::JsonEmpty;
    use ntex::http::error::BlockingError;
    use ntex::http::StatusCode;
    use serde_json::json;

    #[test]
    fn test_respond() {
        let data = json!({"key": "value"});
        let app_result: AppResult<_> = Ok(data.clone());

        let result = app_result.respond();
        match result {
            Ok(response) => {
                assert_eq!(response.status(), StatusCode::OK);
            }
            Err(e) => panic!("Expected Ok, but got Err: {:?}", e),
        }
    }

    #[test]
    fn test_respond_msg() {
        let data = json!({"key": "value"});
        let app_result: AppResult<_> = Ok(data.clone());

        let result = app_result.respond_msg("Success");
        match result {
            Ok(response) => {
                assert_eq!(response.status(), StatusCode::OK);
            }
            Err(e) => panic!("Expected Ok, but got Err: {:?}", e),
        }
    }

    #[test]
    fn test_respond_error() {
        let error = BlockingError::Canceled;
        let result: Result<JsonEmpty, BlockingError<AppMessage>> = Err(error);

        let result = result.respond();
        match result {
            Ok(_) => panic!("Expected Err, but got OK"),
            Err(e) => {
                assert_eq!(e.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }

    #[test]
    fn test_respond_msg_error() {
        let error = BlockingError::Error(AppMessage::WarningMessage("invalid"));
        let result: Result<JsonEmpty, BlockingError<AppMessage>> = Err(error);

        let result = result.respond_msg("Error occurred");
        match result {
            Ok(_) => panic!("Expected Err, but got Ok"),
            Err(e) => {
                assert_eq!(e.status_code(), StatusCode::BAD_REQUEST);
            }
        }
    }
}
