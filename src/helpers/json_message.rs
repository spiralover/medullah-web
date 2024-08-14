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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use ntex::http::StatusCode;

    #[test]
    fn test_ok() {
        let data = json!({"key": "value"});
        let message = Some("Operation successful".to_string());

        let response = JsonMessage::ok(data.clone(), message.clone());

        assert!(response.success);
        assert_eq!(response.code, StatusCode::OK.as_u16());
        assert_eq!(response.status, StatusCode::OK.to_string());
        assert!(response.timestamp > 0); // Check if timestamp is a positive value
        assert_eq!(response.data, data);
        assert_eq!(response.message, message);
    }

    #[test]
    fn test_success() {
        let data = json!({"key": "value"});
        let message = Some("Operation successful".to_string());
        let status = StatusCode::CREATED;

        let response = JsonMessage::success(data.clone(), message.clone(), status);

        assert!(response.success);
        assert_eq!(response.code, status.as_u16());
        assert_eq!(response.status, status.to_string());
        assert!(response.timestamp > 0); // Check if timestamp is a positive value
        assert_eq!(response.data, data);
        assert_eq!(response.message, message);
    }

    #[test]
    fn test_failure() {
        let data = json!({"error": "something went wrong"});
        let message = Some("Operation failed".to_string());
        let status = StatusCode::INTERNAL_SERVER_ERROR;

        let response = JsonMessage::failure(data.clone(), message.clone(), status);

        assert!(!response.success);
        assert_eq!(response.code, status.as_u16());
        assert_eq!(response.status, status.to_string());
        assert!(response.timestamp > 0); // Check if timestamp is a positive value
        assert_eq!(response.data, data);
        assert_eq!(response.message, message);
    }
}
