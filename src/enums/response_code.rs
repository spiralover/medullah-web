use crate::contracts::ResponseCodeContract;
use ntex::http::StatusCode;

#[derive(Clone)]
pub enum ResponseCode {
    Ok,
    Created,
    Accepted,
    NoContent,
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    Conflict,
    InternalServerError,
    ServiceUnavailable,
    NotImplemented,
}

impl ResponseCodeContract for ResponseCode {
    fn code(&self) -> &str {
        match self {
            ResponseCode::Ok => "000",
            ResponseCode::Created => "001",
            ResponseCode::Accepted => "002",
            ResponseCode::NoContent => "003",
            ResponseCode::BadRequest => "004",
            ResponseCode::Unauthorized => "005",
            ResponseCode::PaymentRequired => "006",
            ResponseCode::Forbidden => "007",
            ResponseCode::NotFound => "008",
            ResponseCode::Conflict => "009",
            ResponseCode::InternalServerError => "010",
            ResponseCode::ServiceUnavailable => "011",
            ResponseCode::NotImplemented => "012",
        }
    }

    fn status(&self) -> StatusCode {
        match self {
            ResponseCode::Ok => StatusCode::OK,
            ResponseCode::Created => StatusCode::CREATED,
            ResponseCode::Accepted => StatusCode::ACCEPTED,
            ResponseCode::NoContent => StatusCode::NO_CONTENT,
            ResponseCode::BadRequest => StatusCode::BAD_REQUEST,
            ResponseCode::Unauthorized => StatusCode::UNAUTHORIZED,
            ResponseCode::PaymentRequired => StatusCode::PAYMENT_REQUIRED,
            ResponseCode::Forbidden => StatusCode::FORBIDDEN,
            ResponseCode::NotFound => StatusCode::NOT_FOUND,
            ResponseCode::Conflict => StatusCode::CONFLICT,
            ResponseCode::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseCode::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            ResponseCode::NotImplemented => StatusCode::NOT_IMPLEMENTED,
        }
    }

    fn from_code(code: &str) -> Self {
        match code {
            "000" => ResponseCode::Ok,
            "001" => ResponseCode::Created,
            "002" => ResponseCode::Accepted,
            "003" => ResponseCode::NoContent,
            "004" => ResponseCode::BadRequest,
            "005" => ResponseCode::Unauthorized,
            "006" => ResponseCode::PaymentRequired,
            "007" => ResponseCode::Forbidden,
            "008" => ResponseCode::NotFound,
            "009" => ResponseCode::Conflict,
            "010" => ResponseCode::InternalServerError,
            "011" => ResponseCode::ServiceUnavailable,
            "012" => ResponseCode::NotImplemented,
            _ => panic!("Invalid response code"),
        }
    }

    fn from_status(status: StatusCode) -> Self {
        match status {
            StatusCode::OK => ResponseCode::Ok,
            StatusCode::CREATED => ResponseCode::Created,
            StatusCode::ACCEPTED => ResponseCode::Accepted,
            StatusCode::NO_CONTENT => ResponseCode::NoContent,
            StatusCode::BAD_REQUEST => ResponseCode::BadRequest,
            StatusCode::UNAUTHORIZED => ResponseCode::Unauthorized,
            StatusCode::PAYMENT_REQUIRED => ResponseCode::PaymentRequired,
            StatusCode::FORBIDDEN => ResponseCode::Forbidden,
            StatusCode::NOT_FOUND => ResponseCode::NotFound,
            StatusCode::CONFLICT => ResponseCode::Conflict,
            StatusCode::INTERNAL_SERVER_ERROR => ResponseCode::InternalServerError,
            StatusCode::SERVICE_UNAVAILABLE => ResponseCode::ServiceUnavailable,
            StatusCode::NOT_IMPLEMENTED => ResponseCode::NotImplemented,
            _ => panic!("Invalid status code"),
        }
    }
}
