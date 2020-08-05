use rocket_contrib::databases::rusqlite::Error as RusqliteError;
// use rocket::http::Status;
// use rocket::request::Request;
// use rocket::response::Responder;
use std::convert::From;
// use std::{error, fmt};

#[derive(Debug)]
pub enum IOracleError {
    NotFound,
    InternalServerError,
}

// pub type IOracleResult<T> = Result<T, IOracleError>;

impl From<RusqliteError> for IOracleError {
    fn from(error: RusqliteError) -> Self {
        match error {
            RusqliteError::QueryReturnedNoRows => IOracleError::NotFound,
            _ => IOracleError::InternalServerError,
        }
    }
}

// impl fmt::Display for KinderError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             KinderError::NotFound => write!(f, "NotFound"),
//             KinderError::InternalServerError => write!(f, "InternalServerError"),
//             KinderError::BadRequest => write!(f, "BadRequest"),
//             KinderError::Unauthorized => write!(f, "Unauthorized"),
//         }
//     }
// }
//
// impl error::Error for KinderError {
//     fn description(&self) -> &str {
//         match *self {
//             KinderError::NotFound => "Record not found",
//             KinderError::InternalServerError => "Internal server error",
//             KinderError::BadRequest => "Bad Request",
//             KinderError::Unauthorized => "Unauthorized",
//         }
//     }
// }

// impl<'r> Responder<'r> for KinderError {
//     fn respond_to(self, _: &Request) -> rocket::response::Result<'r> {
//         match self {
//             KinderError::NotFound => Err(Status::NotFound),
//             KinderError::BadRequest => Err(Status::BadRequest),
//             KinderError::Unauthorized => Err(Status::Unauthorized),
//             _ => Err(Status::InternalServerError),
//         }
//     }
// }
