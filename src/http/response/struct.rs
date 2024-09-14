use ntex::http::StatusCode;
use ntex::web::HttpResponse;
use serde::Serialize;

use crate::helpers::responder::Responder;
use crate::http::response::defs::StructResponse;
use crate::prelude::HttpResult;

impl<T: Serialize> StructResponse for T {
    fn into_response(self) -> HttpResponse {
        Responder::success(self, None, StatusCode::OK)
    }

    fn respond(self) -> HttpResult {
        Ok(self.into_response())
    }

    fn respond_msg(self, suc: &str) -> HttpResult {
        Ok(Responder::ok(self, suc))
    }
}
