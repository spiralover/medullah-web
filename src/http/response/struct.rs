use ntex::http::StatusCode;
use ntex::web::HttpResponse;
use serde::Serialize;

use crate::helpers::responder::Responder;
use crate::http::response::defs::StructResponse;
use crate::prelude::HttpResult;

impl<T: Serialize> StructResponse for T {
    fn into_response(self, suc: &str) -> HttpResponse {
        Responder::success(self, Some(suc.to_string()), StatusCode::OK)
    }

    fn respond(self, suc: &str) -> HttpResult {
        Ok(self.into_response(suc))
    }
}
