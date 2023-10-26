//! A high level HTTP client library based on compio.

#![warn(missing_docs)]

mod client;
pub use client::*;

mod request;
pub use request::*;

mod response;
pub use response::*;

mod into_url;
pub use into_url::*;

mod stream;
pub(crate) use stream::*;

mod service;
pub(crate) use service::*;

mod util;

use thiserror::Error;

/// The error type used in `compio-http`.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// The request is timeout.
    #[error("request timeout")]
    Timeout,
    /// Bad scheme.
    #[error("bad scheme: {0}")]
    BadScheme(url::Url),
    /// IO error occurs.
    #[error("system error: {0}")]
    System(#[from] std::io::Error),
    /// HTTP related parse error.
    #[error("`http` error: {0}")]
    Http(#[from] http::Error),
    /// Hyper error.
    #[error("`hyper` error: {0}")]
    Hyper(#[from] hyper::Error),
    /// URL parse error.
    #[error("url parse error: {0}")]
    UrlParse(#[from] url::ParseError),
    /// URL encoding error.
    #[error("url encode error: {0}")]
    UrlEncoded(#[from] serde_urlencoded::ser::Error),
    /// JSON serialization error.
    #[cfg(feature = "json")]
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

/// The result type used in `compio-http`.
pub type Result<T> = std::result::Result<T, Error>;
