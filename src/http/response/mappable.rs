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
                msg.into_http_result()
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
        Return::Success(item, msg) => Ok(Responder::ok(item, msg)),
        Return::Message(msg) => msg.into_http_result(),
        Return::Failure(item, msg) => Ok(Responder::failure(
            item,
            Some(msg.to_string()),
            StatusCode::BAD_REQUEST,
        )),
    }
}
