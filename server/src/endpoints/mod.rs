pub mod users;
use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};

/// An enum that represents an error in the endpoint
#[derive(Debug, Display, Error)]
pub enum EndpointError {
    #[display(fmt = "Internal error. Please try again later.")]
    InternalError,

    #[display(fmt = "Bad request. {}", explanation)]
    BadClientData { explanation: &'static str },
}

impl error::ResponseError for EndpointError {
    /// Formulating an HttpResponse based on the error
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            EndpointError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            EndpointError::BadClientData { .. } => StatusCode::BAD_REQUEST,
        }
    }
}
