use chrono::NaiveDateTime;
use ntex::http::header;
use ntex::web::types::Query;
use ntex::web::HttpRequest;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::enums::app_message::AppMessage;

pub type TheQueryParams = Query<QueryParams>;

#[derive(Deserialize, Clone, Default)]
pub struct QueryParams {
    pub(super) search: Option<String>,
    pub(super) limit: Option<i64>,
    pub(super) page: Option<i64>,
    pub(super) per_page: Option<i64>,
    pub status: Option<String>,
    pub stage: Option<String>,
    pub network_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct IdPathParam {
    pub id: String,
}

#[derive(Deserialize)]
pub struct IdAsUuid {
    pub id: Uuid,
}

#[cfg(feature = "feat-validator")]
#[derive(Deserialize, validator::Validate)]
pub struct ReasonPayload {
    #[validate(length(min = 3, max = 1500))]
    pub reason: String,
}

impl QueryParams {
    pub fn search(&self) -> Option<String> {
        self.search.clone()
    }

    pub fn search_query(&self) -> String {
        self.search.clone().unwrap_or(String::from(""))
    }

    pub fn search_query_like(&self) -> String {
        format!("%{}%", self.search_query())
    }

    pub fn limit(&self) -> i64 {
        let limit = self.limit.unwrap_or(10);
        match limit > 50 {
            true => 50,
            false => limit,
        }
    }

    pub fn curr_page(&self) -> i64 {
        self.page.unwrap_or(1)
    }

    pub fn per_page(&self) -> i64 {
        let limit = self.per_page.unwrap_or(10);
        match limit > 50 {
            true => 50,
            false => limit,
        }
    }
}

pub fn get_ip_and_ua(req: &HttpRequest) -> (Option<String>, Option<String>) {
    let user_agent = req
        .headers()
        .get(header::USER_AGENT)
        .map(|u| u.to_str().unwrap().to_string());

    let ip_address = req
        .connection_info()
        .remote()
        .map(|v| v.to_string())
        .unwrap_or(req.peer_addr().map(|s| s.to_string()).unwrap());

    (Some(ip_address), user_agent)
}

#[allow(dead_code)]
pub fn date_from_unsafe_input(date: &str, field_name: &str) -> Result<NaiveDateTime, AppMessage> {
    NaiveDateTime::parse_from_str(format!("{} 00:00:00", date).as_str(), "%Y-%m-%d %H:%M:%S")
        .map_err(|e| {
            AppMessage::WarningMessageString(format!(
                "Invalid {} input value({}), please make sure it's valid date; {}",
                field_name, date, e
            ))
        })
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HttpHeaderItem {
    pub name: String,
    pub value: String,
}
