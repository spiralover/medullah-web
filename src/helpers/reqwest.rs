use crate::prelude::{AppMessage, IntoAppResult};
use crate::results::AppResult;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::fmt::Display;

pub struct ReqwestResponseError {
    body: String,
    status: StatusCode,
}

impl ReqwestResponseError {
    pub fn create(status: StatusCode, body: String) -> ReqwestResponseError {
        ReqwestResponseError { status, body }
    }

    pub fn make(status: StatusCode, body: String) -> AppMessage {
        AppMessage::ReqwestResponseError(ReqwestResponseError { status, body })
    }

    pub fn code(&self) -> &StatusCode {
        &self.status
    }

    pub fn body(&self) -> &String {
        &self.body
    }

    pub fn deserialize<T: DeserializeOwned>(&self) -> AppResult<T> {
        serde_json::from_str::<T>(&self.body).into_app_result()
    }
}

impl Display for ReqwestResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body.clone())
    }
}
