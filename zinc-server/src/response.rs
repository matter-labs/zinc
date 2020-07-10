//!
//! The Zinc server response.
//!

use actix_web::http::StatusCode;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use futures::future;
use serde_derive::Serialize;

///
/// The Zinc server response.
///
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Response<T, E>
where
    T: serde::Serialize,
    E: serde::Serialize + actix_web::ResponseError,
{
    /// The success data variant.
    Success {
        #[serde(skip_serializing)]
        code: StatusCode,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<T>,
    },
    /// The error data variant.
    Error(E),
}

impl<T, E> Default for Response<T, E>
where
    T: serde::Serialize,
    E: serde::Serialize + actix_web::ResponseError,
{
    fn default() -> Self {
        Self::success(StatusCode::OK)
    }
}

impl<T, E> Response<T, E>
where
    T: serde::Serialize,
    E: serde::Serialize + actix_web::ResponseError,
{
    ///
    /// A shortcut constructor.
    ///
    pub fn success(code: StatusCode) -> Self {
        Self::Success { code, data: None }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn success_with_data(code: StatusCode, data: T) -> Self {
        Self::Success {
            code,
            data: Some(data),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn error(error: E) -> Self {
        Self::Error(error)
    }
}

impl<T, E> Responder for Response<T, E>
where
    T: serde::Serialize,
    E: serde::Serialize + actix_web::ResponseError + 'static,
{
    type Error = E;
    type Future = future::Ready<Result<HttpResponse, E>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        match self {
            Self::Success { code, data } => match data {
                Some(data) => future::ok(HttpResponse::build(code).json(data)),
                None => future::ok(HttpResponse::new(code)),
            },
            Self::Error(error) => future::err(error),
        }
    }
}
