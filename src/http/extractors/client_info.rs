use crate::helpers::request::RequestHelper;
use crate::prelude::AppMessage;
use ntex::http::Payload;
use ntex::web::{FromRequest, HttpRequest};

pub struct ClientInfo {
    pub ip: Option<String>,
    pub ua: Option<String>,
}

impl ClientInfo {
    pub fn into_parts(self) -> (Option<String>, Option<String>) {
        (self.ip, self.ua)
    }
}

impl<Err> FromRequest<Err> for ClientInfo {
    type Error = AppMessage;

    async fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Result<Self, Self::Error> {
        Ok(ClientInfo {
            ip: req.ip(),
            ua: req.user_agent(),
        })
    }
}
