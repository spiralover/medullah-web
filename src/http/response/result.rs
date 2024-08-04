use ntex::http::StatusCode;
use serde::Serialize;

use crate::helpers::responder::Responder;
use crate::http::response::defs::{OptionResultResponse, ResultResponse};
use crate::prelude::{AppMessage, AppResult, HttpResult};

impl ResultResponse for Result<AppMessage, AppMessage> {
    fn send_result(self) -> HttpResult {
        match self {
            Ok(data) => Ok(data.into_response()),
            Err(err) => Err(err),
        }
    }

    fn send_result_msg(self, _msg: &str) -> HttpResult {
        self.send_result()
    }
}

impl<T: Serialize> OptionResultResponse<T> for AppResult<T> {
    fn is_empty(&self) -> bool {
        if let Err(AppMessage::EntityNotFound(..)) = self {
            return true;
        }

        false
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
