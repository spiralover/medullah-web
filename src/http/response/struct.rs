use ntex::http::StatusCode;
use ntex::web::HttpResponse;
use serde::Serialize;

use crate::helpers::responder::Responder;
use crate::http::response::defs::StructResponse;
use crate::prelude::HttpResult;

impl<T: Serialize> StructResponse for T {
    fn send_response(self) -> HttpResponse {
        Responder::success(self, None, StatusCode::OK)
    }

    fn send_struct_result(self) -> HttpResult {
        Ok(self.send_response())
    }
}
