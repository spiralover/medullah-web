use crate::enums::app_message::AppMessage;

pub mod app_result;
#[cfg(feature = "feat-redis")]
pub mod redis_result;

pub type AppResult<T> = Result<T, AppMessage>;

#[cfg(feature = "feat-redis")]
pub type RedisResult<T> = Result<T, redis::RedisError>;

#[cfg(feature = "feat-database")]
pub type AppPaginationResult<T> = AppResult<crate::database::pagination::PageData<T>>;

#[cfg(feature = "feat-ntex")]
pub type HttpResult = Result<ntex::web::HttpResponse, AppMessage>;
