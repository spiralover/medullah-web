use std::fmt::{Display, Formatter};

use crate::contracts::ResponseCodeContract;
use crate::enums::ResponseCode;
use crate::helpers::json::json_empty;
use crate::helpers::json_message::JsonMessage;
use ntex::http::{Response, StatusCode};
use ntex::web::HttpResponse;
use serde::{Deserialize, Serialize};

pub struct Responder;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonResponse<T: Serialize> {
    pub code: String,
    pub success: bool,
    pub timestamp: u64,
    pub message: Option<String>,
    pub data: T,
}

#[derive(Debug, Serialize)]
pub struct SeJsonResponse<T> {
    pub code: String,
    pub success: bool,
    pub timestamp: u64,
    pub message: Option<String>,
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct DeJsonResponse<T> {
    pub code: String,
    pub success: bool,
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
    pub fn send_msg<C, D>(data: D, code: C, msg: &str) -> Response
    where
        C: ResponseCodeContract,
        D: Serialize,
    {
        Self::respond(
            JsonMessage::make(data, code.code(), code.success(), Some(msg.to_string())),
            code.status(),
        )
    }

    pub fn send<C, D>(data: D, code: C) -> Response
    where
        C: ResponseCodeContract,
        D: Serialize,
    {
        Self::respond(
            JsonMessage::make(data, code.code(), code.success(), None),
            code.status(),
        )
    }

    pub fn ok_message(msg: &str) -> Response {
        Self::message(msg, ResponseCode::Ok)
    }

    pub fn success_message(msg: &str) -> Response {
        Self::ok_message(msg)
    }

    pub fn warning_message(msg: &str) -> Response {
        Self::bad_req_message(msg)
    }

    pub fn bad_req_message(msg: &str) -> Response {
        Self::message(msg, ResponseCode::BadRequest)
    }

    pub fn not_found_message(msg: &str) -> Response {
        Self::message(msg, ResponseCode::NotFound)
    }

    pub fn entity_not_found_message(entity: &str) -> Response {
        let msg = format!("Such {} does not exists", entity);
        Self::not_found_message(&msg)
    }

    pub fn internal_server_error_message(msg: &str) -> Response {
        Self::message(msg, ResponseCode::InternalServerError)
    }

    pub fn not_found() -> Response {
        Self::not_found_message("Not Found")
    }

    pub fn internal_server_error() -> Response {
        Self::internal_server_error_message("Internal Server Error")
    }

    pub fn message<C: ResponseCodeContract>(msg: &str, code: C) -> Response {
        let message = JsonMessage::make(
            json_empty(),
            code.code(),
            code.success(),
            Some(msg.to_owned()),
        );

        Self::respond(message, code.status())
    }

    /// Send a response without the standard response wrapper
    ///
    /// # Arguments
    ///
    /// * `data`: Any item that implements serde::Serialize
    /// * `status`: A http status code to respond with
    ///
    /// returns: Response<Body>
    ///
    pub fn respond<T: Serialize>(data: T, status: StatusCode) -> Response {
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
    use ntex::http::StatusCode;
    use ntex::util::BytesMut;
    use serde_json::json;

    use super::*;

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
        let response = Responder::send_msg(data.clone(), ResponseCode::Ok, "Success");

        assert_eq!(response.status(), StatusCode::OK);
        let resp_body = collect_raw_body(response).await;
        let body: serde_json::Value = serde_json::from_str(&resp_body).unwrap();
        assert_eq!(body["code"], "000");
        assert_eq!(body["success"], true);
        assert_eq!(body["message"], "Success");
        assert_eq!(body["data"], data);
    }

    #[tokio::test]
    async fn test_created() {
        let data = json!({"key": "value"});
        let response = Responder::send_msg(data.clone(), ResponseCode::Created, "Success");

        assert_eq!(response.status(), StatusCode::CREATED);
        let resp_body = collect_raw_body(response).await;
        let body: serde_json::Value = serde_json::from_str(&resp_body).unwrap();
        assert_eq!(body["code"], "001");
        assert_eq!(body["success"], true);
        assert_eq!(body["message"], "Success");
        assert_eq!(body["data"], data);
    }

    #[tokio::test]
    async fn test_failure() {
        let data = json!({"key": "value"});
        let response = Responder::send_msg(data.clone(), ResponseCode::NotFound, "Failure");

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let resp_body = collect_raw_body(response).await;
        let body: serde_json::Value = serde_json::from_str(&resp_body).unwrap();
        assert_eq!(body["code"], "008");
        assert_eq!(body["success"], false);
        assert_eq!(body["message"], "Failure");
        assert_eq!(body["data"], data);
    }

    #[tokio::test]
    async fn test_redirect() {
        let url = "http://example.com";
        let response = Responder::redirect(url);

        assert_eq!(response.status(), StatusCode::FOUND);
        assert_eq!(
            response
                .headers()
                .get("Location")
                .unwrap()
                .to_str()
                .unwrap(),
            url
        );
    }

    #[tokio::test]
    async fn test_internal_server_error() {
        let response = Responder::internal_server_error();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let resp_body = collect_raw_body(response).await;
        let body: serde_json::Value = serde_json::from_str(&resp_body).unwrap();
        assert_eq!(body["code"], "010");
        assert_eq!(body["success"], false);
        assert_eq!(body["message"], "Internal Server Error");
        assert_eq!(body["data"], serde_json::to_value(json_empty()).unwrap()); // assuming `json_empty()` returns an empty object
    }
}
