use std::sync::OnceLock;

use crate::app_state::AppState;

pub mod app_state;
pub mod enums;
pub mod results;

#[cfg(feature = "feat-database")]
pub mod database;

pub mod redis;

pub mod helpers;

pub mod app_setup;
pub mod env_logger;
#[cfg(feature = "feat-ntex")]
pub mod http;
pub mod services;
pub mod tokio;

pub static APP: OnceLock<AppState> = OnceLock::new();

pub mod prelude {
    pub use crate::app_state::AppState;
    pub use crate::enums::app_message::AppMessage;
    pub use crate::helpers::once_lock::OnceLockHelper;
    pub use crate::redis::client::Redis;
    #[cfg(feature = "feat-ntex")]
    pub use crate::results::HttpResult;
    pub use crate::results::{app_result::IntoAppResult, AppResult};
    pub use crate::APP;
}
