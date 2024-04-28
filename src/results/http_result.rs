use ntex::http::error::BlockingError;
use ntex::web::HttpResponse;
use serde::Serialize;

use crate::enums::app_message::AppMessage;
use crate::helpers::responder::json_success;
use crate::prelude::IntoAppResult;
use crate::results::{AppResult, HttpResult};

pub trait ErroneousResponse {
    fn send_result(self) -> HttpResult;

    fn send_result_msg(self, msg: &str) -> HttpResult;
}

pub trait NtexBlockingResultResponder {
    fn respond(self) -> HttpResult;

    fn respond_msg(self, suc: &str) -> HttpResult;
}

pub trait StructResponse: Sized {
    fn send_response(self) -> HttpResponse;

    fn send_struct_result(self) -> Result<HttpResponse, AppMessage>;
}

pub trait ErroneousOptionResponse<T> {
    fn is_empty(&self) -> bool;

    fn is_error_or_empty(&self) -> bool;

    fn get_error_result(self) -> AppResult<T>;

    fn send_error(self) -> HttpResult;

    fn send_entity(self) -> HttpResult;

    fn send_response(self) -> HttpResult;
}

impl<T: Serialize> StructResponse for T {
    fn send_response(self) -> HttpResponse {
        json_success(self, None)
    }

    fn send_struct_result(self) -> HttpResult {
        Ok(self.send_response())
    }
}

impl ErroneousResponse for Result<AppMessage, AppMessage> {
    fn send_result(self) -> HttpResult {
        if self.is_err() {
            return Err(self.err().unwrap());
        }

        Ok(self.unwrap().into_response())
    }

    fn send_result_msg(self, _msg: &str) -> HttpResult {
        self.send_result()
    }
}

impl<T: Serialize> ErroneousOptionResponse<T> for AppResult<T> {
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
        Ok(json_success(self.unwrap(), None))
    }

    fn send_response(self) -> HttpResult {
        if self.is_error_or_empty() {
            return self.send_error();
        }

        self.send_entity()
    }
}

impl<T: Serialize> ErroneousResponse for AppResult<T> {
    fn send_result(self) -> HttpResult {
        if self.is_err() {
            return Err(self.err().unwrap());
        }

        Ok(json_success(self.unwrap(), None))
    }

    fn send_result_msg(self, msg: &str) -> HttpResult {
        if self.is_err() {
            return Err(self.err().unwrap());
        }

        Ok(json_success(self.unwrap(), Some(msg.to_string())))
    }
}

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
        <Result<T, AppMessage> as ErroneousResponse>::send_result(self.into_app_result())
    }

    fn respond_msg(self, msg: &str) -> HttpResult {
        <Result<T, AppMessage> as ErroneousResponse>::send_result_msg(self.into_app_result(), msg)
    }
}

impl NtexBlockingResultResponder for Result<AppMessage, BlockingError<AppMessage>> {
    fn respond(self) -> HttpResult {
        <Result<AppMessage, AppMessage> as ErroneousResponse>::send_result(self.into_app_result())
    }

    fn respond_msg(self, msg: &str) -> HttpResult {
        <Result<AppMessage, AppMessage> as ErroneousResponse>::send_result_msg(
            self.into_app_result(),
            msg,
        )
    }
}
