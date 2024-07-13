use std::fmt::{Display, Formatter};

use ntex::http::StatusCode;
use ntex::web::HttpResponse;
use serde::{Deserialize, Serialize};

use crate::helpers::json::{json_empty, JsonEmpty};
use crate::helpers::time::current_timestamp;

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

pub fn map_empty_json<F: Serialize>(_f: F) -> JsonEmpty {
    json_empty()
}

pub fn json<T: Serialize>(data: T, status: StatusCode) -> HttpResponse {
    HttpResponse::build(status).json(&data)
}

pub fn json_success<T: Serialize>(data: T, message: Option<String>) -> HttpResponse {
    json(
        JsonResponse {
            success: true,
            code: StatusCode::OK.as_u16(),
            status: StatusCode::OK.to_string(),
            timestamp: current_timestamp(),
            data,
            message,
        },
        StatusCode::OK,
    )
}

pub fn json_error<T: Serialize>(
    data: T,
    status: StatusCode,
    message: Option<String>,
) -> HttpResponse {
    json(
        JsonResponse {
            success: false,
            code: status.as_u16(),
            status: status.to_string(),
            timestamp: current_timestamp(),
            data,
            message,
        },
        status,
    )
}

pub fn json_error_struct<T: Serialize>(
    data: T,
    status: StatusCode,
    message: Option<String>,
) -> JsonResponse<T> {
    JsonResponse {
        success: false,
        code: status.as_u16(),
        status: status.to_string(),
        timestamp: current_timestamp(),
        data,
        message,
    }
}

pub fn json_error_message(message: &str) -> HttpResponse {
    json_error_message_status(message, StatusCode::BAD_REQUEST)
}

pub fn json_unauthorized_message(message: &str) -> HttpResponse {
    json_error(
        json_empty(),
        StatusCode::UNAUTHORIZED,
        Some(message.to_string()),
    )
}

pub fn json_error_message_status(message: &str, status: StatusCode) -> HttpResponse {
    json_error(json_empty(), status, Some(message.to_string()))
}

pub fn json_success_message(message: &str) -> HttpResponse {
    json_success(json_empty(), Some(message.to_string()))
}

pub fn json_not_found_response(message: Option<&str>) -> HttpResponse {
    json_error_message_status(message.unwrap_or("Not Found"), StatusCode::NOT_FOUND)
}

pub fn json_entity_not_found_response(entity: &str) -> HttpResponse {
    json_not_found_response(Some(format!("Such {} does not exists", entity).as_str()))
}
