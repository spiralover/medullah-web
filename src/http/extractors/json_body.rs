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
    pub fn raw(self) -> String {
        self.json
    }

    pub fn json<T: DeserializeOwned>(self) -> AppResult<T> {
        serde_json::from_str::<T>(&self.json).into_app_result()
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
