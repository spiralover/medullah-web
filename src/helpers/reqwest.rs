use reqwest::StatusCode;
use std::fmt::Display;

use crate::prelude::AppMessage;

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
}

impl Display for ReqwestResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body.clone())
    }
}
