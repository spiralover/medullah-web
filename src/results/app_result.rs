#[cfg(feature = "feat-database")]
use crate::database::Model;
use crate::enums::app_message::AppMessage;
use crate::helpers::json::{json_empty, JsonEmpty};
#[cfg(feature = "feat-database")]
use crate::results::AppPaginationResult;
use crate::results::AppResult;
#[cfg(feature = "feat-database")]
use diesel::result::Error;
#[cfg(feature = "feat-database")]
use diesel::QueryResult;
use ntex::web::error::BlockingError;
use serde::Serialize;

pub type AppOptionalResult<T> = Result<Option<T>, AppMessage>;

pub trait IntoAppResult<T> {
    fn into_app_result(self) -> AppResult<T>;
}

pub trait IntoEmptyJson {
    fn into_empty_json(self) -> AppResult<JsonEmpty>;
}

impl<T: Serialize> IntoEmptyJson for AppResult<T> {
    fn into_empty_json(self) -> AppResult<JsonEmpty> {
        Ok(json_empty())
    }
}

impl<T> IntoAppResult<T> for serde_json::Result<T> {
    fn into_app_result(self) -> AppResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => Err(AppMessage::SerdeError(e)),
        }
    }
}

#[cfg(feature = "feat-database")]
pub trait IntoShareableResult<S: Serialize, T: Serialize + Model> {
    fn into_shareable_result(self) -> AppResult<S>;
}

#[cfg(feature = "feat-database")]
pub trait IntoShareablePaginationResult<S: Serialize, T: Serialize + Model> {
    fn into_shareable_result(self) -> AppPaginationResult<S>;
}

#[cfg(feature = "feat-database")]
impl<Sha, Ent> IntoShareableResult<Sha, Ent> for AppResult<Ent>
where
    Sha: Serialize,
    Ent: Serialize + Model<Entity = Sha>,
{
    fn into_shareable_result(self) -> AppResult<Sha> {
        self.map(|entity| entity.into_shareable())
    }
}

#[cfg(feature = "feat-database")]
impl<Sha, Ent> IntoShareablePaginationResult<Sha, Ent> for AppPaginationResult<Ent>
where
    Sha: Serialize,
    Ent: Serialize + Model<Entity = Sha>,
{
    fn into_shareable_result(self) -> AppPaginationResult<Sha> {
        self.map(|paged| paged.format(|entity| entity.into_shareable()))
    }
}

#[cfg(feature = "feat-database")]
impl<T> IntoAppResult<T> for QueryResult<T> {
    fn into_app_result(self) -> AppResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(Error::NotFound) => Err(AppMessage::EntityNotFound("".to_string())),
            Err(e) => Err(AppMessage::DatabaseError(e)),
        }
    }
}

impl<T> IntoAppResult<T> for Result<AppResult<T>, BlockingError<AppMessage>> {
    fn into_app_result(self) -> AppResult<T> {
        match self {
            Ok(res) => res,
            Err(_err) => Err(AppMessage::InternalServerError),
        }
    }
}

impl<T> IntoAppResult<T> for Result<T, BlockingError<AppMessage>> {
    fn into_app_result(self) -> AppResult<T> {
        match self {
            Ok(res) => Ok(res),
            Err(err) => Err(match err {
                BlockingError::Error(err) => err,
                BlockingError::Canceled => AppMessage::BlockingErrorCanceled,
            }),
        }
    }
}
