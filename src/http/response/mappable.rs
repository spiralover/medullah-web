use ntex::http::error::BlockingError;
use ntex::http::StatusCode;
use serde::Serialize;

use crate::helpers::responder::Responder;
use crate::http::response::defs::{MappableResponse, Return};
use crate::prelude::AppMessage;
use crate::results::HttpResult;

impl<T: Serialize> MappableResponse<T> for Result<T, BlockingError<AppMessage>> {
    fn respond_map<Func>(self, func: Func) -> HttpResult
    where
        Func: FnOnce(T) -> Return<T>,
    {
        match self {
            Ok(data) => format_return(func(data)),
            Err(err) => {
                let msg = AppMessage::BlockingNtexErrorOuterBoxed(Box::new(err));
                msg.http_result()
            }
        }
    }

    fn respond_map_any<Func>(self, map: Func) -> HttpResult
    where
        Func: FnOnce(Self) -> Return<T>,
    {
        format_return(map(self))
    }
}

fn format_return<T: Serialize>(ret: Return<T>) -> HttpResult {
    match ret {
        Return::Ok(item, msg) => Ok(Responder::ok(item, msg)),
        Return::Message(msg) => msg.http_result(),
        Return::Response(data, msg, status) => {
            let code = status.as_u16();
            Ok(match (200..300).contains(&code) {
                true => Responder::success(data, Some(msg.to_string()), status),
                false => Responder::failure(data, Some(msg.to_string()), status),
            })
        }
        Return::BadRequest(item, msg) => Ok(Responder::failure(
            item,
            Some(msg.to_string()),
            StatusCode::BAD_REQUEST,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::json::JsonEmpty;
    use ntex::http::StatusCode;
    use serde_json::json;

    #[test]
    fn test_respond_map_ok() {
        let data = json!({"key": "value"});
        let result: Result<_, BlockingError<AppMessage>> = Ok(data.clone());

        let response = result.respond_map(|data| Return::Ok(data, "Success"));

        match response {
            Ok(responder) => {
                assert_eq!(responder.status(), StatusCode::OK);
            }
            Err(e) => panic!("Expected Ok, but got Err: {:?}", e),
        }
    }

    #[test]
    fn test_respond_map_err() {
        let error = BlockingError::Canceled;
        let result: Result<JsonEmpty, BlockingError<AppMessage>> = Err(error);

        let response = result.respond_map(|data| Return::Ok(data, "Success"));

        match response {
            Ok(responder) => {
                assert_eq!(responder.status(), StatusCode::INTERNAL_SERVER_ERROR);
            }
            Err(e) => panic!("Expected Ok, but got Err: {:?}", e),
        }
    }

    #[test]
    fn test_respond_map_any() {
        let data = json!({"key": "value"});
        let result: Result<_, BlockingError<AppMessage>> = Ok(data.clone());

        let response = result.respond_map_any(|res| Return::Ok(res.unwrap(), "Success"));

        match response {
            Ok(responder) => {
                assert_eq!(responder.status(), StatusCode::OK);
            }
            Err(e) => panic!("Expected Ok, but got Err: {:?}", e),
        }
    }
}
