use chrono::NaiveDate;
use chrono::NaiveDateTime;
use ntex::http::header;
use ntex::web::types::Query;
use ntex::web::HttpRequest;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::enums::app_message::AppMessage;

pub type TheQueryParams = Query<QueryParams>;

/// Represents common query parameters used for filtering, pagination, and sorting in API requests.
#[derive(Deserialize, Clone, Default)]
pub struct QueryParams {
    /// Search term for filtering results based on relevant text.
    ///
    /// Example: `?search=example`
    pub search: Option<String>,

    /// The maximum number of results to return.
    ///
    /// Example: `?limit=50`
    pub limit: Option<i64>,

    /// The current page number for paginated results.
    ///
    /// Example: `?page=2`
    pub page: Option<i64>,

    /// Number of results per page.
    ///
    /// Example: `?per_page=20`
    pub per_page: Option<i64>,

    /// Filter results based on their status.
    ///
    /// Example: `?status=active`
    pub status: Option<String>,

    /// Filter results based on their stage in a process or workflow.
    ///
    /// Example: `?stage=pending`
    pub stage: Option<String>,

    /// Specifies the column to be used for sorting the results.
    ///
    /// Example: `?order_col=created_at`
    pub order_col: Option<String>,

    /// Specifies the sorting direction: `asc` (ascending) or `desc` (descending).
    ///
    /// Example: `?order_dir=desc`
    pub order_dir: Option<String>,

    /// Filters results by a start date (inclusive). Expected format: `YYYY-MM-DD`.
    ///
    /// Example: `?start_date=2024-01-01`
    pub start_date: Option<NaiveDate>,

    /// Filters results by an end date (inclusive). Expected format: `YYYY-MM-DD`.
    ///
    /// Example: `?end_date=2024-12-31`
    pub end_date: Option<NaiveDate>,
}

#[derive(Deserialize)]
pub struct IdPathParam {
    pub id: String,
}

#[derive(Deserialize)]
pub struct IdAsUuid {
    pub id: Uuid,
}

#[cfg(feature = "validator")]
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
        self.limit.unwrap_or(10).min(150)
    }

    pub fn curr_page(&self) -> i64 {
        self.page.unwrap_or(1)
    }

    pub fn per_page(&self) -> i64 {
        self.per_page.unwrap_or(10).min(150)
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
