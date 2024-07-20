use ntex::web::HttpResponse;
use serde::Serialize;
use crate::helpers::responder::json_success;
use crate::http::response::defs::StructResponse;
use crate::prelude::HttpResult;

impl<T: Serialize> StructResponse for T {
    fn send_response(self) -> HttpResponse {
        json_success(self, None)
    }

    fn send_struct_result(self) -> HttpResult {
        Ok(self.send_response())
    }
}
