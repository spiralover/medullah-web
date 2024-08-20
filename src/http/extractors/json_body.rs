use log::debug;
use ntex::http::Payload;
use ntex::util::BytesMut;
use ntex::web::{FromRequest, HttpRequest};
use serde::de::DeserializeOwned;

use crate::prelude::{AppMessage, AppResult, IntoAppResult};

pub struct JsonBody {
    json: String,
}

impl JsonBody {
    pub fn raw(&self) -> &String {
        &self.json
    }

    pub fn deserialize<T: DeserializeOwned>(&self) -> AppResult<T> {
        serde_json::from_str::<T>(&self.json).into_app_result()
    }

    pub fn json_value(&self) -> AppResult<serde_json::Value> {
        serde_json::from_str(&self.json).into_app_result()
    }
}

impl<Err> FromRequest<Err> for JsonBody {
    type Error = AppMessage;

    async fn from_request(_req: &HttpRequest, payload: &mut Payload) -> AppResult<Self> {
        let mut bytes = BytesMut::new();
        while let Some(item) = ntex::util::stream_recv(payload).await {
            bytes.extend_from_slice(&item?);
        }

        let raw = String::from_utf8(bytes.to_vec())?;
        debug!("[json-body] {}", raw);
        Ok(JsonBody { json: raw })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestStruct {
        field1: String,
        field2: i32,
    }

    #[test]
    fn test_raw() {
        let json_str = r#"{"field1": "value1", "field2": 42}"#.to_string();
        let json_body = JsonBody { json: json_str.clone() };

        assert_eq!(json_body.raw(), &json_str);
    }

    #[test]
    fn test_deserialize_success() {
        let json_str = r#"{"field1": "value1", "field2": 42}"#.to_string();
        let json_body = JsonBody { json: json_str };

        let result: AppResult<TestStruct> = json_body.deserialize();
        assert!(result.is_ok());

        let deserialized = result.unwrap();
        let expected = TestStruct {
            field1: "value1".to_string(),
            field2: 42,
        };

        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_deserialize_failure() {
        let json_str = r#"{"field1": "value1", "field2": "invalid_int"}"#.to_string();
        let json_body = JsonBody { json: json_str };

        let result: AppResult<TestStruct> = json_body.deserialize();
        assert!(result.is_err());
    }

    #[test]
    fn test_json_value_success() {
        let json_str = r#"{"field1": "value1", "field2": 42}"#.to_string();
        let json_body = JsonBody { json: json_str };

        let result = json_body.json_value();
        assert!(result.is_ok());

        let json_value = result.unwrap();
        let expected = json!({
            "field1": "value1",
            "field2": 42
        });

        let parsed_json: serde_json::Value = serde_json::from_str(&json_body.json).unwrap();
        assert_eq!(json_value, expected);
        assert_eq!(json_value, parsed_json);
    }

    #[test]
    fn test_json_value_failure() {
        let json_str = "not_a_json".to_string();
        let json_body = JsonBody { json: json_str };

        let result = json_body.json_value();
        assert!(result.is_err());
    }

    #[test]
    fn test_json_value_string_as_value() {
        let json_str = "\"just_a_string\"".to_string();
        let json_body = JsonBody { json: json_str.clone() };

        let result = json_body.json_value();
        assert!(result.is_ok());

        let json_value = result.unwrap();

        let expected = serde_json::Value::String("just_a_string".to_string());

        assert_eq!(json_value, expected);
    }

    #[test]
    fn test_deserialize_to_map() {
        let json_str = r#"{"key1": "value1", "key2": "value2"}"#.to_string();
        let json_body = JsonBody { json: json_str };

        let result: AppResult<HashMap<String, String>> = json_body.deserialize();
        assert!(result.is_ok());

        let deserialized = result.unwrap();
        let mut expected = HashMap::new();
        expected.insert("key1".to_string(), "value1".to_string());
        expected.insert("key2".to_string(), "value2".to_string());

        assert_eq!(deserialized, expected);
    }
}
