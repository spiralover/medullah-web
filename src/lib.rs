use std::sync::OnceLock;

use crate::app_state::MedullahState;

pub mod app_state;
pub mod enums;
pub mod results;

#[cfg(feature = "database")]
pub mod database;

#[cfg(feature = "redis")]
pub mod redis;

pub mod helpers;

pub mod app_setup;
pub mod env_logger;
pub mod http;
pub mod macros;
#[cfg(feature = "rabbitmq")]
pub mod rabbitmq;
pub mod services;
pub mod tokio;
pub mod contracts;

pub static MEDULLAH: OnceLock<MedullahState> = OnceLock::new();

pub mod prelude {
    pub use crate::app_state::MedullahState;
    pub use crate::enums::app_message::AppMessage;
    pub use crate::helpers::once_lock::OnceLockHelper;
    #[cfg(feature = "rabbitmq")]
    pub use crate::rabbitmq::RabbitMQ;
    #[cfg(feature = "redis")]
    pub use crate::redis::Redis;
    pub use crate::results::HttpResult;
    pub use crate::results::{app_result::IntoAppResult, AppResult};
    pub use crate::MEDULLAH;
}
