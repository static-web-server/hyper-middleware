use hyper::StatusCode;
use std::fmt;
use thiserror::Error as BaseError;

/// Just a `anyhow::Result` type alias.
pub type Result<T = (), E = Error> = anyhow::Result<T, E>;

/// Just re-export some `anyhow` stuff.
pub use anyhow::anyhow;
pub use anyhow::bail;
pub use anyhow::Context;

/// Custom `Error` with HTTP status code support.
#[derive(BaseError, Debug)]
pub struct Error {
    #[source]
    source: anyhow::Error,
    status: Option<StatusCode>,
}

impl Error {
    /// Construct an ad-hoc an HTTP `Error` from another error type.
    pub fn from_err(source: anyhow::Error, status: Option<StatusCode>) -> Self {
        Self { source, status }
    }

    /// Construct an ad-hoc an HTTP `Error` from a string error.
    pub fn from_str(source: &str, status: Option<StatusCode>) -> Self {
        Self {
            source: anyhow::anyhow!(source.to_owned()),
            status,
        }
    }

    /// Gets a reference to the `Error` source.
    pub fn source(&self) -> &anyhow::Error {
        &self.source
    }

    /// Gets the HTTP status code associated with the error.
    pub fn status(&self) -> Option<&StatusCode> {
        self.status.as_ref()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.source, f)
    }
}

impl From<anyhow::Error> for Error {
    /// Converts whatever error type into a HTTP `Error`.
    fn from(err: anyhow::Error) -> Self {
        Self {
            source: err,
            status: None,
        }
    }
}

impl From<&str> for Error {
    /// Converts a string error into a HTTP `Error`.
    fn from(err: &str) -> Self {
        Self {
            source: anyhow::anyhow!(err.to_owned()),
            status: None,
        }
    }
}

/// Construct an ad-hoc `Error` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), None)
    }}
}

// 4xx
/// Construct an `Error` with `StatusCode::BAD_REQUEST` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! bad_request {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::BAD_REQUEST))
    }}
}

/// Construct an `Error` with `StatusCode::UNAUTHORIZED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! unauthorized {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::UNAUTHORIZED))
    }}
}

/// Construct an `Error` with `StatusCode::PAYMENT_REQUIRED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! payment_required {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::PAYMENT_REQUIRED))
    }}
}

/// Construct an `Error` with `StatusCode::FORBIDDEN` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! forbidden {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::FORBIDDEN))
    }}
}

/// Construct an `Error` with `StatusCode::NOT_FOUND` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! not_found {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::NOT_FOUND))
    }}
}

/// Construct an `Error` with `StatusCode::METHOD_NOT_ALLOWED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! method_not_allowed {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::METHOD_NOT_ALLOWED))
    }}
}

/// Construct an `Error` with `StatusCode::NOT_ACCEPTABLE` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! not_acceptable {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::NOT_ACCEPTABLE))
    }}
}

/// Construct an `Error` with `StatusCode::PROXY_AUTHENTICATION_REQUIRED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! proxy_authentication_required {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::PROXY_AUTHENTICATION_REQUIRED))
    }}
}

/// Construct an `Error` with `StatusCode::REQUEST_TIMEOUT` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! request_timeout {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::REQUEST_TIMEOUT))
    }}
}

/// Construct an `Error` with `StatusCode::CONFLICT` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! conflict {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::CONFLICT))
    }}
}

/// Construct an `Error` with `StatusCode::GONE` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! gone {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::GONE))
    }}
}

/// Construct an `Error` with `StatusCode::LENGTH_REQUIRED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! length_required {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::LENGTH_REQUIRED))
    }}
}

/// Construct an `Error` with `StatusCode::PRECONDITION_FAILED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! precondition_failed {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::PRECONDITION_FAILED))
    }}
}

/// Construct an `Error` with `StatusCode::PAYLOAD_TOO_LARGE` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! payload_too_large {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::PAYLOAD_TOO_LARGE))
    }}
}

/// Construct an `Error` with `StatusCode::URI_TOO_LONG` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! uri_too_long {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::URI_TOO_LONG))
    }}
}

/// Construct an `Error` with `StatusCode::UNSUPPORTED_MEDIA_TYPE` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! unsupported_media_type {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::UNSUPPORTED_MEDIA_TYPE))
    }}
}

/// Construct an `Error` with `StatusCode::RANGE_NOT_SATISFIABLE` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! range_not_satisfiable {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::RANGE_NOT_SATISFIABLE))
    }}
}

/// Construct an `Error` with `StatusCode::EXPECTATION_FAILED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! expectation_failed {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::EXPECTATION_FAILED))
    }}
}

//  50x
/// Construct an `Error` with `StatusCode::INTERNAL_SERVER_ERROR` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! internal_server_error {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::INTERNAL_SERVER_ERROR))
    }}
}

/// Construct an `Error` with `StatusCode::NOT_IMPLEMENTED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! not_implemented {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::NOT_IMPLEMENTED))
    }}
}

/// Construct an `Error` with `StatusCode::BAD_GATEWAY` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! bad_gateway {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::BAD_GATEWAY))
    }}
}

/// Construct an `Error` with `StatusCode::SERVICE_UNAVAILABLE` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! service_unavailable {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::SERVICE_UNAVAILABLE))
    }}
}

/// Construct an `Error` with `StatusCode::GATEWAY_TIMEOUT` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! gateway_timeout {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::GATEWAY_TIMEOUT))
    }}
}

/// Construct an `Error` with `StatusCode::HTTP_VERSION_NOT_SUPPORTED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_version_not_supported {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::HTTP_VERSION_NOT_SUPPORTED))
    }}
}

/// Construct an `Error` with `StatusCode::VARIANT_ALSO_NEGOTIATES` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! variant_also_negotiates {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::VARIANT_ALSO_NEGOTIATES))
    }}
}

/// Construct an `Error` with `StatusCode::INSUFFICIENT_STORAGE` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! insufficient_storage {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::INSUFFICIENT_STORAGE))
    }}
}

/// Construct an `Error` with `StatusCode::LOOP_DETECTED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! loop_detected {
    ($($arg:tt)*) => {{
        Error::from_err(anyhow::anyhow!($($arg)*), Some(StatusCode::LOOP_DETECTED))
    }}
}
