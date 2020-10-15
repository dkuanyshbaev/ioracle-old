use rocket::http::Status;
use rocket::request::Request;
use rocket::response::Responder;
use rocket_contrib::databases::diesel::result::Error as DieselError;
use std::convert::From;
use std::{error, fmt};

#[derive(Debug)]
pub enum IOracleError {
    NotFound,
    InternalServerError,
}

pub type IOracleResult<T> = Result<T, IOracleError>;

impl From<DieselError> for IOracleError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => IOracleError::NotFound,
            _ => IOracleError::InternalServerError,
        }
    }
}

impl From<csv::Error> for IOracleError {
    fn from(error: csv::Error) -> Self {
        match error {
            _ => IOracleError::InternalServerError,
        }
    }
}

impl fmt::Display for IOracleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IOracleError::NotFound => write!(f, "NotFound"),
            IOracleError::InternalServerError => write!(f, "InternalServerError"),
        }
    }
}

impl error::Error for IOracleError {
    fn description(&self) -> &str {
        match *self {
            IOracleError::NotFound => "Record not found",
            IOracleError::InternalServerError => "Internal server error",
        }
    }
}

impl<'r> Responder<'r> for IOracleError {
    fn respond_to(self, _: &Request) -> rocket::response::Result<'r> {
        match self {
            IOracleError::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        }
    }
}

// impl<'r> Responder<'r, 'static> for IOracleError {
//     fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
//         match self {
//             IOracleError::NotFound => Err(Status::NotFound),
//             _ => Err(Status::InternalServerError),
//         }
//     }
// }
