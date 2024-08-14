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

    pub fn success_message(msg: &str) -> Response {
        Self::ok_message(msg)
    }

    pub fn warning_message(msg: &str) -> Response {
        Self::bad_req_message(msg)
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

    pub fn not_found() -> Response {
        Self::not_found_message("Not Found")
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
            .header(ntex::http::header::LOCATION, url)
            .finish()
            .into_body()
    }

    fn make_response<T: Serialize>(data: T, status: StatusCode) -> Response {
        HttpResponse::build(status).json(&data)
    }
}

#[cfg(test)]
mod tests {
    use futures_util::StreamExt;
    use super::*;
    use ntex::http::StatusCode;
    use ntex::util::BytesMut;
    use serde_json::json;

    async fn collect_raw_body(mut response: Response) -> String {
        let mut buffer = BytesMut::new();
        let mut body = response.take_body();

        while let Some(chunk) = body.next().await {
            match chunk {
                Ok(data) => buffer.extend_from_slice(&data),
                Err(e) => {
                    eprintln!("Error reading body: {:?}", e);
                    break;
                }
            }
        }

        // Convert buffer to Bytes for further use
        String::from_utf8_lossy(buffer.freeze().as_ref()).to_string()
    }
    #[tokio::test]
    async fn test_ok() {
        let data = json!({"key": "value"});
        let response = Responder::ok(data.clone(), "Success");

        assert_eq!(response.status(), StatusCode::OK);
        let resp_body = collect_raw_body(response).await;
        let body: serde_json::Value = serde_json::from_str(&resp_body).unwrap();
        assert_eq!(body["code"], 200);
        assert_eq!(body["success"], true);
        assert_eq!(body["status"], "200 OK");
        assert_eq!(body["message"], "Success");
        assert_eq!(body["data"], data);
    }

    #[tokio::test]
    async fn test_success() {
        let data = json!({"key": "value"});
        let response = Responder::success(data.clone(), Some("Success".to_string()), StatusCode::CREATED);

        assert_eq!(response.status(), StatusCode::CREATED);
        let resp_body = collect_raw_body(response).await;
        let body: serde_json::Value = serde_json::from_str(&resp_body).unwrap();
        assert_eq!(body["code"], 201);
        assert_eq!(body["success"], true);
        assert_eq!(body["status"], "201 Created");
        assert_eq!(body["message"], "Success");
        assert_eq!(body["data"], data);
    }

    #[tokio::test]
    async fn test_failure() {
        let data = json!({"key": "value"});
        let response = Responder::failure(data.clone(), Some("Failure".to_string()), StatusCode::BAD_REQUEST);

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let resp_body = collect_raw_body(response).await;
        let body: serde_json::Value = serde_json::from_str(&resp_body).unwrap();
        assert_eq!(body["code"], 400);
        assert_eq!(body["success"], false);
        assert_eq!(body["status"], "400 Bad Request");
        assert_eq!(body["message"], "Failure");
        assert_eq!(body["data"], data);
    }

    #[tokio::test]
    async fn test_message() {
        let response = Responder::message("Error Message", StatusCode::NOT_FOUND);

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let resp_body = collect_raw_body(response).await;
        let body: serde_json::Value = serde_json::from_str(&resp_body).unwrap();
        assert_eq!(body["code"], 404);
        assert_eq!(body["success"], false);
        assert_eq!(body["status"], "404 Not Found");
        assert_eq!(body["message"], "Error Message");
        assert_eq!(body["data"], serde_json::to_value(json_empty()).unwrap());  // assuming `json_empty()` returns an empty object
    }

    #[tokio::test]
    async fn test_redirect() {
        let url = "http://example.com";
        let response = Responder::redirect(url);

        assert_eq!(response.status(), StatusCode::FOUND);
        assert_eq!(
            response.headers().get("Location").unwrap().to_str().unwrap(),
            url
        );
    }

    #[tokio::test]
    async fn test_internal_server_error() {
        let response = Responder::internal_server_error();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let resp_body = collect_raw_body(response).await;
        let body: serde_json::Value = serde_json::from_str(&resp_body).unwrap();
        assert_eq!(body["code"], 500);
        assert_eq!(body["success"], false);
        assert_eq!(body["status"], "500 Internal Server Error");
        assert_eq!(body["message"], "Internal Server Error");
        assert_eq!(body["data"], serde_json::to_value(json_empty()).unwrap());  // assuming `json_empty()` returns an empty object
    }
}
