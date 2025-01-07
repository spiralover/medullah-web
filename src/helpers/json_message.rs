use serde::Serialize;

use crate::helpers::responder::JsonResponse;
use crate::helpers::time::current_timestamp;

pub struct JsonMessage;

impl JsonMessage {
    pub fn make<T: Serialize>(
        data: T,
        code: &str,
        success: bool,
        message: Option<String>,
    ) -> JsonResponse<T> {
        JsonResponse {
            data,
            success,
            message,
            code: code.to_string(),
            timestamp: current_timestamp(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::ResponseCodeContract;
    use crate::enums::ResponseCode;
    use serde_json::json;

    #[test]
    fn test_ok() {
        let data = json!({"key": "value"});
        let message = Some("Operation successful".to_string());

        let response =
            JsonMessage::make(data.clone(), ResponseCode::Ok.code(), true, message.clone());

        assert!(response.success);
        assert_eq!(response.code, ResponseCode::Ok.code());
        assert!(response.timestamp > 0); // Check if timestamp is a positive value
        assert_eq!(response.data, data);
        assert_eq!(response.message, message);
    }

    #[test]
    fn test_failure() {
        let data = json!({"error": "something went wrong"});
        let message = Some("Operation failed".to_string());

        let response = JsonMessage::make(
            data.clone(),
            ResponseCode::InternalServerError.code(),
            false,
            message.clone(),
        );

        assert!(!response.success);
        assert_eq!(response.code, ResponseCode::InternalServerError.code());
        assert!(response.timestamp > 0); // Check if timestamp is a positive value
        assert_eq!(response.data, data);
        assert_eq!(response.message, message);
    }
}
