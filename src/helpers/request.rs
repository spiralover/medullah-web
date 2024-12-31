use log::debug;
use ntex::util::Bytes;
use ntex::web::HttpRequest;
use serde::de::DeserializeOwned;
use serde_json::{json, Map, Value};

use crate::app_state::MedullahState;
use crate::helpers::http::get_ip_and_ua;
use crate::http::extractors::client_info::ClientInfo;
use crate::results::app_result::IntoAppResult;
use crate::results::AppResult;

pub trait RequestHelper {
    fn app(&self) -> &MedullahState;

    #[cfg(feature = "database")]
    fn db_pool(&self) -> &crate::database::DBPool;

    fn client_info(&self) -> ClientInfo;

    fn get_headers(&self) -> Map<String, Value>;

    fn json<T: DeserializeOwned>(bytes: Bytes) -> AppResult<T>;
}

impl RequestHelper for HttpRequest {
    fn app(&self) -> &MedullahState {
        self.app_state::<MedullahState>().unwrap()
    }

    #[cfg(feature = "database")]
    fn db_pool(&self) -> &crate::database::DBPool {
        self.app().database()
    }

    fn client_info(&self) -> ClientInfo {
        let (ip_address, user_agent) = get_ip_and_ua(self);

        ClientInfo {
            ip: ip_address,
            ua: user_agent,
        }
    }

    fn get_headers(&self) -> Map<String, Value> {
        let mut headers_json_object = Map::new();

        for (name, value) in self.headers().iter() {
            headers_json_object.insert(name.to_string(), json!(value.to_str().unwrap()));
        }

        headers_json_object
    }

    fn json<T: DeserializeOwned>(bytes: Bytes) -> AppResult<T> {
        let raw = String::from_utf8(bytes.to_vec())?;
        debug!("[json-body]: {}", raw);
        serde_json::from_str::<T>(&raw).into_app_result()
    }
}
