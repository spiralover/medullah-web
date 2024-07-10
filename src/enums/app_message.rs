use std::fmt::{Debug, Display, Formatter};
use std::io;

use log::error;
use ntex::http::error::BlockingError;
#[cfg(feature = "feat-ntex")]
use ntex::http::StatusCode;
#[cfg(feature = "feat-ntex")]
use ntex::web::{HttpRequest, WebResponseError};
use crate::helpers::reqwest::ReqwestResponseError;

#[cfg(feature = "feat-ntex")]
use crate::helpers::responder::json_error_message;
#[cfg(feature = "feat-ntex")]
use crate::helpers::responder::{
    json_entity_not_found_response, json_error_message_status, json_success_message,
};

pub enum AppMessage {
    InvalidUUID,
    UnAuthorized,
    InternalServerError,
    InternalServerErrorMessage(&'static str),
    IoError(io::Error),
    Redirect(&'static str),
    SuccessMessage(&'static str),
    SuccessMessageString(String),
    WarningMessage(&'static str),
    WarningMessageString(String),
    HttpClientError(String, String),
    UnAuthorizedMessage(&'static str),
    UnAuthorizedMessageString(String),
    #[cfg(feature = "feat-validator")]
    FormValidationError(validator::ValidationErrors),
    EntityNotFound(String),
    #[cfg(feature = "reqwest")]
    ReqwestError(reqwest::Error),
    #[cfg(feature = "reqwest")]
    ReqwestResponseError(ReqwestResponseError),
    #[cfg(feature = "feat-mailer")]
    MailerError(reqwest::Error),
    #[cfg(feature = "feat-nerve")]
    NerveError(reqwest::Error),
    SerdeError(serde_json::Error),
    SerdeError500(serde_json::Error),
    #[cfg(feature = "feat-base64")]
    Base64Error(base64::DecodeError),
    JoinError(tokio::task::JoinError),
    #[cfg(feature = "feat-crypto")]
    JwtError(jsonwebtoken::errors::Error),
    FromUtf8Error(std::string::FromUtf8Error),
    ChronoParseError(chrono::ParseError),
    #[cfg(feature = "feat-rabbitmq")]
    RabbitmqError(lapin::Error),
    RedisError(redis::RedisError),
    RedisPoolError(mobc::Error<redis::RedisError>),
    #[cfg(feature = "feat-ntex")]
    ErrorMessage(String, StatusCode),
    #[cfg(feature = "feat-ntex")]
    PayloadError(ntex::http::error::PayloadError),
    #[cfg(feature = "feat-ntex")]
    BlockingNtexErrorInnerBoxed(BlockingError<Box<Self>>),
    #[cfg(feature = "feat-ntex")]
    BlockingNtexErrorOuterBoxed(Box<BlockingError<Self>>),
    #[cfg(feature = "feat-ntex")]
    BlockingNtexIoError(BlockingError<io::Error>),
    #[cfg(feature = "feat-ntex")]
    BlockingErrorCanceled,
    R2d2Error(r2d2::Error),
    #[cfg(feature = "feat-database")]
    DatabaseEntityNotFound,
    #[cfg(feature = "feat-database")]
    DatabaseError(diesel::result::Error),
    #[cfg(feature = "feat-database")]
    DatabaseErrorKind(
        diesel::result::DatabaseErrorKind,
        Box<dyn diesel::result::DatabaseErrorInformation + Send + Sync>,
    ),
    #[cfg(feature = "feat-database")]
    DatabaseRollbackErrorOnCommit {
        rollback_error: Box<diesel::result::Error>,
        commit_error: Box<diesel::result::Error>,
    },
    #[cfg(feature = "feat-database")]
    DatabaseErrorMessage(String),
    #[cfg(feature = "feat-database")]
    DatabaseInvalidCString(std::ffi::NulError),
    #[cfg(feature = "feat-database")]
    DatabaseQueryBuilderError(Box<dyn serde::de::StdError + Send + Sync>),
    #[cfg(feature = "feat-database")]
    DatabaseDeserializationError(Box<dyn serde::de::StdError + Send + Sync>),
    #[cfg(feature = "feat-database")]
    DatabaseSerializationError(Box<dyn serde::de::StdError + Send + Sync>),
    #[cfg(feature = "feat-database")]
    DatabaseRollbackTransaction,
    #[cfg(feature = "feat-database")]
    DatabaseAlreadyInTransaction,
    #[cfg(feature = "feat-database")]
    DatabaseNotInTransaction,
    #[cfg(feature = "feat-database")]
    DatabaseBrokenTransactionManager,
}

fn format_message(status: &AppMessage, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(get_message(status).as_str())
}

fn get_message(status: &AppMessage) -> String {
    match status {
        AppMessage::InvalidUUID => String::from("Invalid unique identifier"),
        AppMessage::UnAuthorized => {
            String::from("You are not authorized to access requested resource(s)")
        }
        AppMessage::Redirect(url) => format!("Redirecting to '{}'...", url),
        AppMessage::EntityNotFound(entity) => format!("Such {} does not exits", entity),
        AppMessage::R2d2Error(error) => error.to_string(),
        #[cfg(feature = "feat-database")]
        AppMessage::DatabaseEntityNotFound => String::from("Such entity does not exits"),
        #[cfg(feature = "feat-database")]
        AppMessage::DatabaseErrorMessage(err) => err.to_owned(),
        #[cfg(feature = "feat-crypto")]
        AppMessage::JwtError(err) => err.to_string(),
        AppMessage::HttpClientError(msg, _) => msg.to_owned(),
        AppMessage::IoError(error) => error.to_string(),
        AppMessage::SerdeError(error) => error.to_string(),
        AppMessage::SerdeError500(error) => error.to_string(),
        #[cfg(feature = "feat-rabbitmq")]
        AppMessage::RabbitmqError(error) => error.to_string(),
        AppMessage::RedisError(error) => error.to_string(),
        AppMessage::RedisPoolError(error) => error.to_string(),
        AppMessage::JoinError(error) => error.to_string(),
        #[cfg(feature = "reqwest")]
        AppMessage::ReqwestError(error) => error.to_string(),
        #[cfg(feature = "reqwest")]
        AppMessage::ReqwestResponseError(error) => error.body().to_owned(),
        #[cfg(feature = "feat-mailer")]
        AppMessage::MailerError(error) => error.to_string(),
        #[cfg(feature = "feat-nerve")]
        AppMessage::NerveError(error) => error.to_string(),
        #[cfg(feature = "feat-base64")]
        AppMessage::Base64Error(error) => error.to_string(),
        AppMessage::FromUtf8Error(error) => error.to_string(),
        AppMessage::ChronoParseError(error) => error.to_string(),
        #[cfg(feature = "feat-ntex")]
        AppMessage::BlockingNtexErrorInnerBoxed(error) => error.to_string(),
        #[cfg(feature = "feat-ntex")]
        AppMessage::BlockingNtexErrorOuterBoxed(error) => error.to_string(),
        #[cfg(feature = "feat-ntex")]
        AppMessage::BlockingNtexIoError(error) => error.to_string(),
        #[cfg(feature = "feat-ntex")]
        AppMessage::PayloadError(error) => error.to_string(),
        AppMessage::WarningMessage(message) => message.to_string(),
        AppMessage::WarningMessageString(message) => message.to_string(),
        AppMessage::SuccessMessage(message) => message.to_string(),
        AppMessage::SuccessMessageString(message) => message.to_string(),
        #[cfg(feature = "feat-ntex")]
        AppMessage::ErrorMessage(message, _) => message.clone(),
        #[cfg(feature = "feat-database")]
        AppMessage::DatabaseError(message) => message.to_string(),
        AppMessage::UnAuthorizedMessage(message) => message.to_string(),
        AppMessage::UnAuthorizedMessageString(message) => message.to_string(),
        AppMessage::InternalServerErrorMessage(message) => message.to_string(),
        #[cfg(feature = "feat-validator")]
        AppMessage::FormValidationError(e) => String::from(e.to_string().as_str()),
        _ => String::from("Internal Server Error"),
    }
}

pub fn get_middleware_level_message(app: &AppMessage) -> String {
    match app {
        AppMessage::WarningMessage(message) => message.to_string(),
        AppMessage::WarningMessageString(message) => message.to_owned(),
        AppMessage::SuccessMessage(message) => message.to_string(),
        AppMessage::SuccessMessageString(message) => message.to_owned(),
        AppMessage::UnAuthorizedMessage(message) => message.to_string(),
        AppMessage::UnAuthorizedMessageString(message) => message.to_owned(),
        AppMessage::InternalServerErrorMessage(message) => message.to_string(),
        _ => {
            error!("[middleware-level-error] {:?}", app);
            String::from("Something isn't right, our engineers are on it")
        }
    }
}

#[cfg(feature = "feat-ntex")]
pub fn send_response(status: &AppMessage) -> ntex::web::HttpResponse {
    match status {
        AppMessage::EntityNotFound(entity) => json_entity_not_found_response(entity),
        AppMessage::Redirect(url) => ntex::web::HttpResponse::Found()
            .header(
                ntex::http::header::HeaderName::from_static("Location"),
                ntex::http::header::HeaderValue::from_static(url),
            )
            .finish()
            .into_body(),
        AppMessage::IoError(message) => {
            log::error!("IO Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        AppMessage::R2d2Error(message) => {
            log::error!("R2d2 Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        #[cfg(feature = "feat-crypto")]
        AppMessage::JwtError(message) => {
            log::error!("Jwt Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        #[cfg(feature = "feat-rabbitmq")]
        AppMessage::RabbitmqError(message) => {
            log::error!("Rabbitmq Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        AppMessage::RedisError(message) => {
            log::error!("Redis Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        AppMessage::RedisPoolError(message) => {
            log::error!("Redis Pool Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        #[cfg(feature = "reqwest")]
        AppMessage::ReqwestError(message) => {
            log::error!("Http Client(Reqwest) Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        #[cfg(feature = "reqwest")]
        AppMessage::ReqwestResponseError(err) => {
            log::error!("Http Client(Reqwest) Error[{}]: {}", err.code(), err.body());
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        AppMessage::FromUtf8Error(message) => {
            log::error!("Utf8 Conversion Error: {:?}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        #[cfg(feature = "feat-base64")]
        AppMessage::Base64Error(message) => {
            log::error!("Base64 Error: {:?}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        AppMessage::SerdeError(message) => {
            log::error!("Serde Error: {:?}", message);
            json_error_message_status(&message.to_string(), StatusCode::BAD_REQUEST)
        }
        AppMessage::SerdeError500(message) => {
            log::error!("Serde Error: {}", message);
            json_error_message_status(
                "This is strange, something went wrong",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
        AppMessage::DatabaseErrorMessage(message) => {
            log::error!("DB Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        AppMessage::BlockingNtexErrorInnerBoxed(message) => {
            log::error!("Blocking Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        AppMessage::BlockingNtexErrorOuterBoxed(message) => {
            log::error!("Blocking Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        AppMessage::BlockingNtexIoError(message) => {
            log::error!("Blocking IO Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        AppMessage::PayloadError(message) => {
            log::error!("Payload Extraction Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        AppMessage::InternalServerErrorMessage(message) => {
            log::error!("Internal Server Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        AppMessage::SuccessMessage(message) => json_success_message(message),
        AppMessage::SuccessMessageString(message) => json_success_message(message),
        AppMessage::ErrorMessage(message, status) => json_error_message_status(message, *status),
        AppMessage::UnAuthorizedMessage(message) => {
            json_error_message_status(message, StatusCode::UNAUTHORIZED)
        }
        AppMessage::UnAuthorizedMessageString(message) => {
            json_error_message_status(message, StatusCode::UNAUTHORIZED)
        }
        AppMessage::ChronoParseError(error) => {
            let message = error.to_string();
            log::error!("Failed To Parse DateTime: {}", message);
            json_error_message_status(&message, StatusCode::BAD_REQUEST)
        }
        #[cfg(feature = "feat-validator")]
        AppMessage::FormValidationError(e) => crate::helpers::responder::json_error(
            e,
            StatusCode::BAD_REQUEST,
            Some(String::from("Validation Error")),
        ),
        AppMessage::DatabaseError(err) => {
            error!("{:?}", err);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        _ => json_error_message(get_message(status).as_str()),
    }
}

#[cfg(feature = "feat-ntex")]
fn get_status_code(status: &AppMessage) -> StatusCode {
    match status {
        AppMessage::InvalidUUID => StatusCode::BAD_REQUEST,
        AppMessage::SuccessMessage(_msg) => StatusCode::OK,
        AppMessage::SuccessMessageString(_msg) => StatusCode::OK,
        AppMessage::WarningMessage(_msg) => StatusCode::BAD_REQUEST,
        AppMessage::WarningMessageString(_msg) => StatusCode::BAD_REQUEST,
        AppMessage::EntityNotFound(_msg) => StatusCode::NOT_FOUND,
        AppMessage::DatabaseEntityNotFound => StatusCode::NOT_FOUND,
        AppMessage::HttpClientError(_msg, _code) => StatusCode::INTERNAL_SERVER_ERROR,
        #[cfg(feature = "feat-crypto")]
        AppMessage::JwtError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        AppMessage::R2d2Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
        AppMessage::IoError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
        AppMessage::ChronoParseError(_msg) => StatusCode::BAD_REQUEST,
        AppMessage::SerdeError(_msg) => StatusCode::BAD_REQUEST,
        AppMessage::SerdeError500(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
        #[cfg(feature = "reqwest")]
        AppMessage::ReqwestError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
        #[cfg(feature = "reqwest")]
        AppMessage::ReqwestResponseError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
        #[cfg(feature = "feat-validator")]
        AppMessage::FormValidationError(_msg) => StatusCode::BAD_REQUEST,
        AppMessage::RedisError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
        #[cfg(feature = "feat-rabbitmq")]
        AppMessage::RabbitmqError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
        AppMessage::BlockingNtexErrorInnerBoxed(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
        AppMessage::BlockingNtexErrorOuterBoxed(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
        AppMessage::BlockingNtexIoError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
        AppMessage::PayloadError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
        AppMessage::ErrorMessage(_, status) => *status,
        AppMessage::UnAuthorized => StatusCode::UNAUTHORIZED,
        AppMessage::UnAuthorizedMessage(_) => StatusCode::UNAUTHORIZED,
        AppMessage::UnAuthorizedMessageString(_) => StatusCode::UNAUTHORIZED,
        AppMessage::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        AppMessage::InternalServerErrorMessage(_) => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::INTERNAL_SERVER_ERROR, // all database-related errors are 500
    }
}

impl Debug for AppMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_message(self, f)
    }
}

#[cfg(feature = "feat-validator")]
impl From<validator::ValidationErrors> for AppMessage {
    fn from(value: validator::ValidationErrors) -> Self {
        AppMessage::FormValidationError(value)
    }
}

impl Display for AppMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_message(self, f)
    }
}

impl From<io::Error> for AppMessage {
    fn from(value: io::Error) -> Self {
        AppMessage::IoError(value)
    }
}

#[cfg(feature = "feat-crypto")]
impl From<jsonwebtoken::errors::Error> for AppMessage {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        AppMessage::JwtError(value)
    }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for AppMessage {
    fn from(value: reqwest::Error) -> Self {
        AppMessage::ReqwestError(value)
    }
}

#[cfg(feature = "feat-ntex")]
impl From<ntex::http::error::PayloadError> for AppMessage {
    fn from(value: ntex::http::error::PayloadError) -> Self {
        AppMessage::PayloadError(value)
    }
}

impl From<r2d2::Error> for AppMessage {
    fn from(value: r2d2::Error) -> Self {
        AppMessage::R2d2Error(value)
    }
}

impl From<tokio::task::JoinError> for AppMessage {
    fn from(value: tokio::task::JoinError) -> Self {
        AppMessage::JoinError(value)
    }
}

#[cfg(feature = "feat-ntex")]
impl From<BlockingError<AppMessage>> for AppMessage {
    fn from(value: BlockingError<AppMessage>) -> Self {
        AppMessage::BlockingNtexErrorOuterBoxed(Box::new(value))
    }
}

#[cfg(feature = "feat-ntex")]
impl From<BlockingError<Box<AppMessage>>> for AppMessage {
    fn from(value: BlockingError<Box<AppMessage>>) -> Self {
        AppMessage::BlockingNtexErrorInnerBoxed(value)
    }
}

#[cfg(feature = "feat-ntex")]
impl From<BlockingError<io::Error>> for AppMessage {
    fn from(value: BlockingError<io::Error>) -> Self {
        AppMessage::BlockingNtexIoError(value)
    }
}

#[cfg(feature = "feat-rabbitmq")]
impl From<lapin::Error> for AppMessage {
    fn from(value: lapin::Error) -> Self {
        AppMessage::RabbitmqError(value)
    }
}

impl From<redis::RedisError> for AppMessage {
    fn from(value: redis::RedisError) -> Self {
        AppMessage::RedisError(value)
    }
}

impl From<serde_json::Error> for AppMessage {
    fn from(value: serde_json::Error) -> Self {
        AppMessage::SerdeError(value)
    }
}

impl From<chrono::ParseError> for AppMessage {
    fn from(value: chrono::ParseError) -> Self {
        AppMessage::ChronoParseError(value)
    }
}

impl From<std::string::FromUtf8Error> for AppMessage {
    fn from(value: std::string::FromUtf8Error) -> Self {
        AppMessage::FromUtf8Error(value)
    }
}

#[cfg(feature = "feat-base64")]
impl From<base64::DecodeError> for AppMessage {
    fn from(value: base64::DecodeError) -> Self {
        AppMessage::Base64Error(value)
    }
}

#[cfg(feature = "feat-ntex")]
impl AppMessage {
    pub fn into_http_result(self) -> crate::results::HttpResult {
        Ok(send_response(&self))
    }

    pub fn into_response(self) -> ntex::web::HttpResponse {
        send_response(&self)
    }
}

#[cfg(feature = "feat-ntex")]
impl WebResponseError for AppMessage {
    fn status_code(&self) -> StatusCode {
        let code = get_status_code(self);
        log::info!("[error-code] {}", code);
        code
    }

    fn error_response(&self, _: &HttpRequest) -> ntex::web::HttpResponse {
        log::info!("[error-body] {}", self);
        send_response(self)
    }
}
