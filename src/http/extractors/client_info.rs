use crate::helpers::http::get_ip_and_ua;
use ntex::http::Payload;
use ntex::web::{FromRequest, HttpRequest};

use crate::prelude::AppMessage;

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
        let (ip_address, user_agent) = get_ip_and_ua(req);

        Ok(ClientInfo {
            ip: ip_address,
            ua: user_agent,
        })
    }
}
