use crate::enums::app_message::AppMessage;

pub mod app_result;
#[cfg(feature = "redis")]
pub mod redis_result;

pub type AppResult<T> = Result<T, AppMessage>;

#[cfg(feature = "redis")]
pub type RedisResult<T> = Result<T, redis::RedisError>;

#[cfg(feature = "database")]
pub type AppPaginationResult<T> = AppResult<crate::database::pagination::PageData<T>>;

pub type HttpResult = Result<ntex::web::HttpResponse, AppMessage>;
