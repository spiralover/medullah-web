use ntex::http::StatusCode;
use serde::Serialize;

use crate::helpers::responder::JsonResponse;
use crate::helpers::time::current_timestamp;

pub struct JsonMessage;

impl JsonMessage {
    pub fn ok<T: Serialize>(data: T, message: Option<String>) -> JsonResponse<T> {
        Self::base(data, StatusCode::OK, true, message)
    }

    pub fn success<T: Serialize>(
        data: T,
        message: Option<String>,
        status: StatusCode,
    ) -> JsonResponse<T> {
        Self::base(data, status, true, message)
    }

    pub fn failure<T: Serialize>(
        data: T,
        message: Option<String>,
        status: StatusCode,
    ) -> JsonResponse<T> {
        Self::base(data, status, false, message)
    }

    fn base<T: Serialize>(
        data: T,
        status: StatusCode,
        success: bool,
        message: Option<String>,
    ) -> JsonResponse<T> {
        JsonResponse {
            success,
            code: status.as_u16(),
            status: status.to_string(),
            timestamp: current_timestamp(),
            data,
            message,
        }
    }
}
