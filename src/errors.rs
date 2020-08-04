// use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::Responder;
// use std::convert::From;
use std::{error, fmt};

#[derive(Debug)]
pub enum IOracleError {
    NotFound,
    InternalServerError,
    BadRequest,
}

impl fmt::Display for IOracleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IOracleError::NotFound => write!(f, "NotFound"),
            IOracleError::InternalServerError => write!(f, "InternalServerError"),
            IOracleError::BadRequest => write!(f, "BadRequest"),
        }
    }
}

impl error::Error for IOracleError {
    fn description(&self) -> &str {
        match *self {
            IOracleError::NotFound => "Record not found",
            IOracleError::InternalServerError => "Internal server error",
            IOracleError::BadRequest => "Bad Request",
        }
    }
}

// impl From<DieselError> for KinderError {
//     fn from(error: DieselError) -> Self {
//         match error {
//             DieselError::NotFound => KinderError::NotFound,
//             _ => KinderError::InternalServerError,
//         }
//     }
// }

impl<'r> Responder<'r> for IOracleError {
    fn respond_to(self, _: &Request) -> rocket::response::Result<'r> {
        match self {
            IOracleError::NotFound => Err(Status::NotFound),
            IOracleError::BadRequest => Err(Status::BadRequest),
            _ => Err(Status::InternalServerError),
        }
    }
}
