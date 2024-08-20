use std::fmt::{Debug, Display, Formatter};
use std::io;

use log::error;
use ntex::http::error::BlockingError;
use ntex::http::StatusCode;
use ntex::web::{HttpRequest, WebResponseError};

#[cfg(feature = "feat-reqwest")]
use crate::helpers::reqwest::ReqwestResponseError;
use crate::helpers::responder::Responder;

pub enum AppMessage {
    InvalidUUID,
    UnAuthorized,
    Forbidden,
    InternalServerError,
    InternalServerErrorMessage(&'static str),
    IoError(io::Error),
    Redirect(&'static str),
    SuccessMessage(&'static str),
    SuccessMessageString(String),
    WarningMessage(&'static str),
    WarningMessageString(String),
    HttpClientError(String, String),
    #[cfg(feature = "feat-hmac")]
    HmacError(hmac::digest::InvalidLength),
    UnAuthorizedMessage(&'static str),
    UnAuthorizedMessageString(String),
    ForbiddenMessage(&'static str),
    ForbiddenMessageString(String),
    #[cfg(feature = "feat-validator")]
    FormValidationError(validator::ValidationErrors),
    EntityNotFound(String),
    #[cfg(feature = "reqwest")]
    ReqwestError(reqwest::Error),
    #[cfg(feature = "reqwest")]
    ReqwestResponseError(ReqwestResponseError),
    #[cfg(feature = "feat-mailer")]
    MailerError(reqwest::Error),
    SerdeError(serde_json::Error),
    SerdeError500(serde_json::Error),
    #[cfg(feature = "feat-base64")]
    Base64Error(base64::DecodeError),
    JoinError(tokio::task::JoinError),
    #[cfg(feature = "feat-jwt")]
    JwtError(jsonwebtoken::errors::Error),
    #[cfg(feature = "feat-crypto")]
    ArgonError(argon2::Error),
    FromUtf8Error(std::string::FromUtf8Error),
    ChronoParseError(chrono::ParseError),
    #[cfg(feature = "feat-rabbitmq")]
    RabbitmqError(lapin::Error),
    #[cfg(feature = "feat-redis")]
    RedisError(redis::RedisError),
    #[cfg(feature = "feat-redis")]
    RedisPoolError(deadpool::managed::PoolError<redis::RedisError>),
    #[cfg(feature = "feat-rabbitmq")]
    RmqPoolError(deadpool::managed::PoolError<lapin::Error>),
    ErrorMessage(String, StatusCode),
    PayloadError(ntex::http::error::PayloadError),
    BlockingNtexErrorInnerBoxed(BlockingError<Box<Self>>),
    BlockingNtexErrorOuterBoxed(Box<BlockingError<Self>>),
    BlockingNtexIoError(BlockingError<io::Error>),
    BlockingErrorCanceled,
    #[cfg(feature = "feat-database")]
    R2d2Error(r2d2::Error),
    #[cfg(feature = "feat-database")]
    DatabaseError(diesel::result::Error),
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
        AppMessage::Forbidden => {
            String::from("You don't have sufficient permissions to access requested resource(s)")
        }
        AppMessage::Redirect(url) => format!("Redirecting to '{}'...", url),
        AppMessage::EntityNotFound(entity) => format!("Such {} does not exits", entity),
        #[cfg(feature = "feat-database")]
        AppMessage::R2d2Error(error) => error.to_string(),
        #[cfg(feature = "feat-rabbitmq")]
        AppMessage::RmqPoolError(error) => error.to_string(),
        #[cfg(feature = "feat-jwt")]
        AppMessage::JwtError(err) => err.to_string(),
        #[cfg(feature = "feat-crypto")]
        AppMessage::ArgonError(err) => err.to_string(),
        AppMessage::HttpClientError(msg, _) => msg.to_owned(),
        AppMessage::IoError(error) => error.to_string(),
        AppMessage::SerdeError(error) => error.to_string(),
        AppMessage::SerdeError500(error) => error.to_string(),
        #[cfg(feature = "feat-rabbitmq")]
        AppMessage::RabbitmqError(error) => error.to_string(),
        #[cfg(feature = "feat-redis")]
        AppMessage::RedisError(error) => error.to_string(),
        #[cfg(feature = "feat-redis")]
        AppMessage::RedisPoolError(error) => error.to_string(),
        AppMessage::JoinError(error) => error.to_string(),
        #[cfg(feature = "reqwest")]
        AppMessage::ReqwestError(error) => error.to_string(),
        #[cfg(feature = "reqwest")]
        AppMessage::ReqwestResponseError(error) => error.body().to_owned(),
        #[cfg(feature = "feat-mailer")]
        AppMessage::MailerError(error) => error.to_string(),
        #[cfg(feature = "feat-base64")]
        AppMessage::Base64Error(error) => error.to_string(),
        AppMessage::FromUtf8Error(error) => error.to_string(),
        AppMessage::ChronoParseError(error) => error.to_string(),
        AppMessage::BlockingNtexErrorInnerBoxed(error) => error.to_string(),
        AppMessage::BlockingNtexErrorOuterBoxed(error) => error.to_string(),
        AppMessage::BlockingNtexIoError(error) => error.to_string(),
        AppMessage::PayloadError(error) => error.to_string(),
        AppMessage::WarningMessage(message) => message.to_string(),
        AppMessage::WarningMessageString(message) => message.to_string(),
        AppMessage::SuccessMessage(message) => message.to_string(),
        AppMessage::SuccessMessageString(message) => message.to_string(),
        AppMessage::ErrorMessage(message, _) => message.clone(),
        #[cfg(feature = "feat-database")]
        AppMessage::DatabaseError(err) => match err {
            diesel::result::Error::NotFound => String::from("Such entity not found"),
            diesel::result::Error::DatabaseError(err, _) => match err {
                diesel::result::DatabaseErrorKind::UniqueViolation => {
                    "conflicted with existing entity".to_string()
                }
                _ => "something went wrong".to_string(),
            },
            _ => {
                error!("database error: {:?}", err);
                String::from("Something went wrong")
            }
        },
        AppMessage::UnAuthorizedMessage(message) => message.to_string(),
        AppMessage::UnAuthorizedMessageString(message) => message.to_string(),
        AppMessage::ForbiddenMessage(message) => message.to_string(),
        AppMessage::ForbiddenMessageString(message) => message.to_string(),
        AppMessage::InternalServerErrorMessage(message) => message.to_string(),
        #[cfg(feature = "feat-hmac")]
        AppMessage::HmacError(message) => message.to_string(),
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
        AppMessage::ForbiddenMessage(message) => message.to_string(),
        AppMessage::ForbiddenMessageString(message) => message.to_owned(),
        AppMessage::InternalServerErrorMessage(message) => message.to_string(),
        #[cfg(feature = "feat-jwt")]
        AppMessage::JwtError(_) => "failed to authenticate your jwt token".to_string(),
        _ => {
            error!("[middleware-level-error] {:?}", app);
            String::from("Something isn't right, our engineers are on it")
        }
    }
}

fn send_response(message: &AppMessage) -> ntex::web::HttpResponse {
    match message {
        AppMessage::EntityNotFound(entity) => Responder::entity_not_found_message(entity),
        AppMessage::Redirect(url) => ntex::web::HttpResponse::Found()
            .header(
                ntex::http::header::HeaderName::from_static("Location"),
                ntex::http::header::HeaderValue::from_static(url),
            )
            .finish()
            .into_body(),
        AppMessage::IoError(message) => {
            log::error!("IO Error: {}", message);
            Responder::internal_server_error()
        }
        #[cfg(feature = "feat-database")]
        AppMessage::R2d2Error(message) => {
            log::error!("R2d2 Error: {}", message);
            Responder::internal_server_error()
        }
        #[cfg(feature = "feat-jwt")]
        AppMessage::JwtError(message) => {
            log::error!("Jwt Error: {}", message);
            Responder::message("invalid jwt token", StatusCode::UNAUTHORIZED)
        }
        #[cfg(feature = "feat-crypto")]
        AppMessage::ArgonError(message) => {
            log::error!("Argon Error: {}", message);
            Responder::internal_server_error()
        }
        #[cfg(feature = "feat-rabbitmq")]
        AppMessage::RabbitmqError(message) => {
            log::error!("Rabbitmq Error: {}", message);
            Responder::internal_server_error()
        }
        #[cfg(feature = "feat-redis")]
        AppMessage::RedisError(message) => {
            log::error!("Redis Error: {}", message);
            Responder::internal_server_error()
        }
        #[cfg(feature = "feat-redis")]
        AppMessage::RedisPoolError(message) => {
            log::error!("Redis Pool Error: {}", message);
            Responder::internal_server_error()
        }
        #[cfg(feature = "feat-rabbitmq")]
        AppMessage::RmqPoolError(message) => {
            log::error!("RabbitMQ Pool Error: {}", message);
            Responder::internal_server_error()
        }
        #[cfg(feature = "reqwest")]
        AppMessage::ReqwestError(message) => {
            log::error!("Http Client(Reqwest) Error: {}", message);
            Responder::internal_server_error()
        }
        #[cfg(feature = "reqwest")]
        AppMessage::ReqwestResponseError(err) => {
            log::error!("Http Client(Reqwest) Error[{}]: {}", err.code(), err.body());
            Responder::internal_server_error()
        }
        AppMessage::FromUtf8Error(message) => {
            log::error!("Utf8 Conversion Error: {:?}", message);
            Responder::internal_server_error()
        }
        #[cfg(feature = "feat-base64")]
        AppMessage::Base64Error(message) => {
            log::error!("Base64 Error: {:?}", message);
            Responder::internal_server_error()
        }
        AppMessage::SerdeError(message) => {
            log::error!("Serde Error: {:?}", message);
            Responder::message(&message.to_string(), StatusCode::BAD_REQUEST)
        }
        AppMessage::SerdeError500(message) => {
            log::error!("Serde Error: {}", message);
            Responder::internal_server_error()
        }
        AppMessage::BlockingNtexErrorInnerBoxed(message) => {
            log::error!("Blocking Error: {}", message);
            Responder::internal_server_error()
        }
        AppMessage::BlockingNtexErrorOuterBoxed(message) => {
            log::error!("Blocking Error: {}", message);
            Responder::internal_server_error()
        }
        AppMessage::BlockingNtexIoError(message) => {
            log::error!("Blocking IO Error: {}", message);
            Responder::internal_server_error()
        }
        AppMessage::PayloadError(message) => {
            log::error!("Payload Extraction Error: {}", message);
            Responder::internal_server_error()
        }
        AppMessage::InternalServerErrorMessage(message) => {
            log::error!("Internal Server Error: {}", message);
            Responder::internal_server_error()
        }
        AppMessage::SuccessMessage(message) => Responder::ok_message(message),
        AppMessage::SuccessMessageString(message) => Responder::ok_message(message),
        AppMessage::ErrorMessage(message, status) => Responder::message(message, *status),
        AppMessage::UnAuthorized => {
            Responder::message(&message.message(), StatusCode::UNAUTHORIZED)
        }
        AppMessage::UnAuthorizedMessage(message) => {
            Responder::message(message, StatusCode::UNAUTHORIZED)
        }
        AppMessage::UnAuthorizedMessageString(message) => {
            Responder::message(message, StatusCode::UNAUTHORIZED)
        }
        AppMessage::Forbidden => Responder::message(&message.message(), StatusCode::FORBIDDEN),
        AppMessage::ForbiddenMessage(message) => Responder::message(message, StatusCode::FORBIDDEN),
        AppMessage::ForbiddenMessageString(message) => {
            Responder::message(message, StatusCode::FORBIDDEN)
        }
        AppMessage::ChronoParseError(error) => {
            let message = error.to_string();
            log::error!("Failed To Parse DateTime: {}", message);
            Responder::message(&message, StatusCode::BAD_REQUEST)
        }
        #[cfg(feature = "feat-validator")]
        AppMessage::FormValidationError(e) => Responder::failure(
            e,
            Some(String::from("Validation Error")),
            StatusCode::BAD_REQUEST,
        ),
        #[cfg(feature = "feat-database")]
        AppMessage::DatabaseError(err) => match err {
            diesel::result::Error::NotFound => {
                Responder::not_found_message("Such entity not found")
            }
            diesel::result::Error::DatabaseError(err, _) => {
                error!("database error: {:?}", err);
                match err {
                    diesel::result::DatabaseErrorKind::UniqueViolation => {
                        Responder::message(&message.message(), StatusCode::CONFLICT)
                    }
                    _ => Responder::internal_server_error(),
                }
            }
            _ => {
                error!("database error: {:?}", err);
                Responder::internal_server_error_message(&message.message())
            }
        },
        #[cfg(feature = "feat-hmac")]
        AppMessage::HmacError(err) => {
            error!("{:?}", err);
            Responder::internal_server_error()
        }
        _ => Responder::bad_req_message(get_message(message).as_str()),
    }
}

pub fn get_status_code(status: &AppMessage) -> StatusCode {
    #[cfg(feature = "feat-database")]
    use diesel::result::Error as DieselError;

    match status {
        AppMessage::InvalidUUID => StatusCode::BAD_REQUEST,
        AppMessage::SuccessMessage(_msg) => StatusCode::OK,
        AppMessage::SuccessMessageString(_msg) => StatusCode::OK,
        AppMessage::WarningMessage(_msg) => StatusCode::BAD_REQUEST,
        AppMessage::WarningMessageString(_msg) => StatusCode::BAD_REQUEST,
        AppMessage::EntityNotFound(_msg) => StatusCode::NOT_FOUND,
        #[cfg(feature = "feat-database")]
        AppMessage::DatabaseError(DieselError::NotFound) => StatusCode::NOT_FOUND,
        #[cfg(feature = "feat-database")]
        AppMessage::DatabaseError(DieselError::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => StatusCode::CONFLICT,
        AppMessage::HttpClientError(_msg, _code) => StatusCode::INTERNAL_SERVER_ERROR,
        #[cfg(feature = "feat-jwt")]
        AppMessage::JwtError(_) => StatusCode::UNAUTHORIZED,
        #[cfg(feature = "feat-crypto")]
        AppMessage::ArgonError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        #[cfg(feature = "feat-database")]
        AppMessage::R2d2Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
        #[cfg(feature = "feat-rabbitmq")]
        AppMessage::RmqPoolError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
        #[cfg(feature = "feat-redis")]
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
        AppMessage::Forbidden => StatusCode::FORBIDDEN,
        AppMessage::ForbiddenMessage(_) => StatusCode::FORBIDDEN,
        AppMessage::ForbiddenMessageString(_) => StatusCode::FORBIDDEN,
        AppMessage::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        AppMessage::InternalServerErrorMessage(_) => StatusCode::INTERNAL_SERVER_ERROR,
        #[cfg(feature = "feat-hmac")]
        AppMessage::HmacError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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

#[cfg(feature = "feat-jwt")]
impl From<jsonwebtoken::errors::Error> for AppMessage {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        AppMessage::JwtError(value)
    }
}

#[cfg(feature = "feat-crypto")]
impl From<argon2::Error> for AppMessage {
    fn from(value: argon2::Error) -> Self {
        AppMessage::ArgonError(value)
    }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for AppMessage {
    fn from(value: reqwest::Error) -> Self {
        AppMessage::ReqwestError(value)
    }
}

impl From<ntex::http::error::PayloadError> for AppMessage {
    fn from(value: ntex::http::error::PayloadError) -> Self {
        AppMessage::PayloadError(value)
    }
}

#[cfg(feature = "feat-database")]
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

impl From<BlockingError<AppMessage>> for AppMessage {
    fn from(value: BlockingError<AppMessage>) -> Self {
        AppMessage::BlockingNtexErrorOuterBoxed(Box::new(value))
    }
}

impl From<BlockingError<Box<AppMessage>>> for AppMessage {
    fn from(value: BlockingError<Box<AppMessage>>) -> Self {
        AppMessage::BlockingNtexErrorInnerBoxed(value)
    }
}

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

#[cfg(feature = "feat-redis")]
impl From<redis::RedisError> for AppMessage {
    fn from(value: redis::RedisError) -> Self {
        AppMessage::RedisError(value)
    }
}

#[cfg(feature = "feat-rabbitmq")]
impl From<deadpool::managed::PoolError<lapin::Error>> for AppMessage {
    fn from(value: deadpool::managed::PoolError<lapin::Error>) -> Self {
        AppMessage::RmqPoolError(value)
    }
}

#[cfg(feature = "feat-hmac")]
impl From<hmac::digest::InvalidLength> for AppMessage {
    fn from(value: hmac::digest::InvalidLength) -> Self {
        AppMessage::HmacError(value)
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

#[cfg(feature = "feat-database")]
impl From<diesel::result::Error> for AppMessage {
    fn from(value: diesel::result::Error) -> Self {
        AppMessage::DatabaseError(value)
    }
}

#[cfg(feature = "feat-database")]
impl From<AppMessage> for diesel::result::Error {
    fn from(value: AppMessage) -> Self {
        use diesel::result::Error as DieselError;

        match value {
            AppMessage::EntityNotFound(_) => diesel::result::Error::NotFound,
            AppMessage::DatabaseError(DieselError::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                x,
            )) => DieselError::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, x),
            AppMessage::DatabaseError(err) => err,
            _ => panic!("unhandled app message: {:?}", value),
        }
    }
}

impl AppMessage {
    pub fn http_result(&self) -> crate::results::HttpResult {
        Ok(self.http_response())
    }

    pub fn http_response(&self) -> ntex::web::HttpResponse {
        send_response(self)
    }

    pub fn status_code(&self) -> StatusCode {
        get_status_code(self)
    }

    pub fn message(&self) -> String {
        get_message(self)
    }
}

impl WebResponseError for AppMessage {
    fn status_code(&self) -> StatusCode {
        let code = self.status_code();
        log::info!("[error-code] {}", code);
        code
    }

    fn error_response(&self, _: &HttpRequest) -> ntex::web::HttpResponse {
        log::info!("[error-body] {}", self);
        self.http_response()
    }
}
