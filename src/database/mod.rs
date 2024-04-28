use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use diesel::{r2d2, PgConnection, QueryResult};
use r2d2::PooledConnection;
use serde::Serialize;

use crate::enums::app_message::AppMessage;
use crate::prelude::AppMessage::EntityNotFound;
use crate::prelude::AppResult;
use crate::results::app_result::AppOptionalResult;

pub mod pagination;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub trait Model: Serialize {
    type Entity;

    fn into_shareable(self) -> Self::Entity;
}

pub trait OptionalResult<'a, T> {
    fn optional(self) -> AppOptionalResult<T>;
    fn required(self, entity: &'a str) -> AppResult<T>;
    fn exists(self) -> AppResult<bool>;
}

pub trait DatabaseConnectionHelper {
    fn connection(&self) -> AppResult<PooledConnection<ConnectionManager<PgConnection>>>;
}

impl DatabaseConnectionHelper for DBPool {
    fn connection(&self) -> AppResult<PooledConnection<ConnectionManager<PgConnection>>> {
        self.get().map_err(AppMessage::R2d2Error)
    }
}

impl<'a, T> OptionalResult<'a, T> for QueryResult<T> {
    fn optional(self) -> AppResult<Option<T>> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(AppMessage::DatabaseError(e)),
        }
    }

    fn required(self, entity: &'a str) -> AppResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(Error::NotFound) => Err(EntityNotFound(entity.to_string())),
            Err(e) => Err(AppMessage::DatabaseError(e)),
        }
    }

    fn exists(self) -> AppResult<bool> {
        match self {
            Ok(_) => Ok(true),
            Err(Error::NotFound) => Ok(false),
            Err(e) => Err(AppMessage::DatabaseError(e)),
        }
    }
}

impl From<Error> for AppMessage {
    fn from(value: Error) -> Self {
        match value {
            Error::InvalidCString(err) => AppMessage::DatabaseErrorMessage(err.to_string()),
            Error::DatabaseError(x, y) => AppMessage::DatabaseErrorKind(x, y),
            Error::NotFound => AppMessage::DatabaseEntityNotFound,
            Error::QueryBuilderError(err) => AppMessage::DatabaseQueryBuilderError(err),
            Error::DeserializationError(err) => AppMessage::DatabaseDeserializationError(err),
            Error::SerializationError(err) => AppMessage::DatabaseDeserializationError(err),
            Error::RollbackErrorOnCommit {
                commit_error,
                rollback_error,
            } => AppMessage::DatabaseRollbackErrorOnCommit {
                commit_error,
                rollback_error,
            },
            Error::RollbackTransaction => AppMessage::DatabaseRollbackTransaction,
            Error::AlreadyInTransaction => AppMessage::DatabaseAlreadyInTransaction,
            Error::NotInTransaction => AppMessage::DatabaseNotInTransaction,
            Error::BrokenTransactionManager => AppMessage::DatabaseBrokenTransactionManager,
            _ => AppMessage::InternalServerError,
        }
    }
}
