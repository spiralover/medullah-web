use ntex::http::StatusCode;
use serde::Serialize;

use crate::helpers::responder::Responder;
use crate::http::response::defs::{OptionResultResponse, ResultResponse};
use crate::prelude::{AppMessage, AppResult, HttpResult};

impl ResultResponse for Result<AppMessage, AppMessage> {
    fn send_result(self) -> HttpResult {
        match self {
            Ok(data) => Ok(data.http_result().unwrap()),
            Err(err) => Err(err),
        }
    }

    fn send_result_msg(self, _msg: &str) -> HttpResult {
        self.send_result()
    }
}

impl<T: Serialize> OptionResultResponse<T> for AppResult<T> {
    fn is_empty(&self) -> bool {
        matches!(self, Err(AppMessage::EntityNotFound(..)))
    }

    fn is_error_or_empty(&self) -> bool {
        self.as_ref().is_err() || self.is_empty()
    }

    fn get_error_result(self) -> AppResult<T> {
        if self.is_err() {
            return Err(self.err().unwrap());
        }

        // let entity = self.
        panic!("Cannot acquire error on successful database action")
    }

    fn send_error(self) -> HttpResult {
        if self.is_err() {
            return Err(self.err().unwrap());
        }

        Err(AppMessage::WarningMessage("Internal Server Error :)"))
    }

    fn send_entity(self) -> HttpResult {
        Ok(Responder::success(self.unwrap(), None, StatusCode::OK))
    }

    fn send_response(self) -> HttpResult {
        match self.is_error_or_empty() {
            true => self.send_error(),
            false => self.send_entity(),
        }
    }
}

impl<T: Serialize> ResultResponse for AppResult<T> {
    fn send_result(self) -> HttpResult {
        match self {
            Ok(data) => Ok(Responder::success(data, None, StatusCode::OK)),
            Err(err) => Err(err),
        }
    }

    fn send_result_msg(self, msg: &str) -> HttpResult {
        match self {
            Ok(data) => Ok(Responder::success(
                data,
                Some(msg.to_string()),
                StatusCode::OK,
            )),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ntex::http::StatusCode;
    use serde_json::json;

    #[test]
    fn test_result_response_send_result_ok() {
        let result: Result<AppMessage, AppMessage> = Ok(AppMessage::EntityNotFound("".to_string()));

        let response = result.send_result();
        match response {
            Ok(responder) => {
                assert_eq!(responder.status(), StatusCode::NOT_FOUND);
            }
            Err(e) => panic!("Expected Ok, but got Err: {:?}", e),
        }
    }

    #[test]
    fn test_result_response_send_result_err() {
        let err = AppMessage::EntityNotFound("app".to_string());
        let result: Result<AppMessage, AppMessage> = Err(err);

        let response = result.send_result();
        match response {
            Ok(_) => panic!("Expected Err, but got Ok"),
            Err(e) => {
                // Verify that the error was correctly propagated
                assert_eq!(e.status_code(), StatusCode::NOT_FOUND);
            }
        }
    }

    #[test]
    fn test_option_result_response_is_empty() {
        let result: AppResult<()> = Err(AppMessage::EntityNotFound("".to_string())); // Assuming this represents an entity not found error

        assert!(result.is_empty());
    }

    #[test]
    fn test_option_result_response_is_error_or_empty() {
        let result_empty: AppResult<()> = Err(AppMessage::EntityNotFound("".to_string())); // Assuming this represents an entity not found error
        let result_error: AppResult<()> = Err(AppMessage::EntityNotFound("".to_string()));
        let result_ok: AppResult<()> = Ok(());

        assert!(result_empty.is_error_or_empty());
        assert!(result_error.is_error_or_empty());
        assert!(!result_ok.is_error_or_empty());
    }

    #[test]
    fn test_option_result_response_get_error_result() {
        let err = AppMessage::BlockingErrorCanceled;
        let result: AppResult<()> = Err(err);

        let error_result = result.get_error_result();
        match error_result {
            Err(e) => {
                assert_eq!(e.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
            }
            Ok(_) => panic!("Expected Err, but got Ok"),
        }
    }

    #[test]
    fn test_option_result_response_send_error() {
        let err = AppMessage::Forbidden;
        let result: AppResult<()> = Err(err);

        let response = result.send_error();
        match response {
            Err(e) => {
                // Verify that the error was correctly propagated
                assert_eq!(e.status_code(), StatusCode::FORBIDDEN);
            }
            Ok(_) => panic!("Expected Err, but got Ok"),
        }
    }

    #[test]
    fn test_option_result_response_send_entity() {
        let data = json!({"key": "value"});
        let result: AppResult<serde_json::Value> = Ok(data.clone());

        let result = result.send_entity();
        match result {
            Ok(responder) => {
                assert_eq!(responder.status(), StatusCode::OK);
            }
            Err(e) => panic!("Expected Ok, but got Err: {:?}", e),
        }
    }

    #[test]
    fn test_option_result_response_send_response_error_or_empty() {
        let result: AppResult<()> = Err(AppMessage::InternalServerError);

        let response = result.send_response();
        match response {
            Err(e) => {
                assert_eq!(e.status_code(), AppMessage::InternalServerError.status_code());
            }
            Ok(_) => panic!("Expected Err, but got Ok"),
        }
    }

    #[test]
    fn test_option_result_response_send_response_ok() {
        let data = json!({"key": "value"});
        let result: AppResult<serde_json::Value> = Ok(data.clone());

        let response = result.send_response();
        match response {
            Ok(responder) => {
                assert_eq!(responder.status(), StatusCode::OK);
            }
            Err(e) => panic!("Expected Ok, but got Err: {:?}", e),
        }
    }
}
