use std::fmt::{Display, Formatter};

use ntex::http::{Response, StatusCode};
use ntex::web::HttpResponse;
use serde::{Deserialize, Serialize};

use crate::helpers::json::json_empty;
use crate::helpers::json_message::JsonMessage;

pub struct Responder;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonResponse<T: Serialize> {
    pub code: u16,
    pub success: bool,
    pub status: String,
    pub timestamp: u64,
    pub message: Option<String>,
    pub data: T,
}

impl<T: Serialize> Display for JsonResponse<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}

impl Responder {
    pub fn ok<T: Serialize>(data: T, msg: &str) -> Response {
        Self::respond(JsonMessage::ok(data, Some(msg.to_string())), StatusCode::OK)
    }

    pub fn success<T: Serialize>(data: T, message: Option<String>, status: StatusCode) -> Response {
        Self::respond(JsonMessage::success(data, message, status), status)
    }

    pub fn failure<T: Serialize>(data: T, message: Option<String>, status: StatusCode) -> Response {
        Self::respond(JsonMessage::failure(data, message, status), status)
    }

    pub fn ok_message(msg: &str) -> Response {
        Self::message(msg, StatusCode::OK)
    }

    pub fn bad_req_message(msg: &str) -> Response {
        Self::message(msg, StatusCode::BAD_REQUEST)
    }

    pub fn not_found_message(msg: &str) -> Response {
        Self::message(msg, StatusCode::NOT_FOUND)
    }

    pub fn entity_not_found_message(entity: &str) -> Response {
        let msg = format!("Such {} does not exists", entity);
        Self::not_found_message(&msg)
    }

    pub fn internal_server_error_message(msg: &str) -> Response {
        Self::message(msg, StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub fn internal_server_error() -> Response {
        Self::internal_server_error_message("Internal Server Error")
    }

    pub fn message(msg: &str, status: StatusCode) -> Response {
        Self::respond(
            JsonMessage::failure(json_empty(), Some(msg.to_owned()), status),
            status,
        )
    }

    fn respond<T: Serialize>(data: T, status: StatusCode) -> Response {
        Self::make_response(data, status)
    }

    pub fn redirect(url: &'static str) -> Response {
        HttpResponse::Found()
            .header(
                ntex::http::header::HeaderName::from_static("Location"),
                ntex::http::header::HeaderValue::from_static(url),
            )
            .finish()
            .into_body()
    }

    fn make_response<T: Serialize>(data: T, status: StatusCode) -> Response {
        HttpResponse::build(status).json(&data)
    }
}
