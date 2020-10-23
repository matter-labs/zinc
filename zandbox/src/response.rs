//!
//! The Zandbox server daemon response.
//!

use std::marker::PhantomData;

use actix_web::http::StatusCode;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use futures::future;
use serde::Serialize;

///
/// The Zandbox server daemon response.
///
#[derive(Debug, Serialize)]
pub struct Response<T, E>
where
    T: serde::Serialize,
    E: serde::Serialize + actix_web::ResponseError,
{
    /// The HTTP status code.
    #[serde(skip_serializing)]
    code: StatusCode,
    /// The optional data payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    /// The unused error type parameter marker.
    _pd: PhantomData<E>,
}

impl<T, E> Default for Response<T, E>
where
    T: serde::Serialize,
    E: serde::Serialize + actix_web::ResponseError,
{
    fn default() -> Self {
        Self::new(StatusCode::OK)
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
    pub fn new(code: StatusCode) -> Self {
        Self {
            code,
            data: None,
            _pd: PhantomData::default(),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_with_data(code: StatusCode, data: T) -> Self {
        Self {
            code,
            data: Some(data),
            _pd: PhantomData::default(),
        }
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
        future::ok(match self.data {
            Some(data) => HttpResponse::build(self.code).json(data),
            None => HttpResponse::new(self.code),
        })
    }
}
