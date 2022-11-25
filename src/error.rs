//! Custom HTTP Error module.

use hyper::StatusCode;
use std::fmt;
use thiserror::Error as ThisError;

/// Represents an HTTP Error.
#[derive(ThisError, Debug)]
pub struct Error {
    source: anyhow::Error,
    status: Option<StatusCode>,
}

impl Error {
    /// Returns the underlying error.
    pub fn source(self) -> anyhow::Error {
        self.source
    }

    /// Returns an HTTP Status Code reference associated with the underlying error.
    pub fn status(&self) -> Option<&StatusCode> {
        self.status.as_ref()
    }

    /// Adds/updates the current HTTP Status Code.
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = Some(status);
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.source, f)
    }
}

impl From<hyper::Error> for Error {
    /// Converts a [`hyper::Error`] type into an HTTP [`Error`].
    fn from(source: hyper::Error) -> Self {
        Self {
            source: anyhow::anyhow!(source),
            status: None,
        }
    }
}

impl From<anyhow::Error> for Error {
    /// Converts whatever error type that implements [`std::error::Error`] into an HTTP [`Error`].
    fn from(source: anyhow::Error) -> Self {
        Self {
            source,
            status: None,
        }
    }
}

impl From<&str> for Error {
    /// Converts a string error into an HTTP [`Error`].
    fn from(source: &str) -> Self {
        Self {
            source: anyhow::anyhow!(source.to_owned()),
            status: None,
        }
    }
}

#[macro_export]
macro_rules! http_error {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*))
    }}
}

// 4xx
/// Construct an `Error` with `StatusCode::BAD_REQUEST` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! bad_request {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::BAD_REQUEST))
    }}
}

/// Construct an `Error` with `StatusCode::UNAUTHORIZED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! unauthorized {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::UNAUTHORIZED))
    }}
}

/// Construct an `Error` with `StatusCode::PAYMENT_REQUIRED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! payment_required {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::PAYMENT_REQUIRED))
    }}
}

/// Construct an `Error` with `StatusCode::FORBIDDEN` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! forbidden {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::FORBIDDEN))
    }}
}

/// Construct an `Error` with `StatusCode::NOT_FOUND` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! not_found {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::NOT_FOUND))
    }}
}

/// Construct an `Error` with `StatusCode::METHOD_NOT_ALLOWED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! method_not_allowed {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::METHOD_NOT_ALLOWED))
    }}
}

/// Construct an `Error` with `StatusCode::NOT_ACCEPTABLE` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! not_acceptable {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::NOT_ACCEPTABLE))
    }}
}

/// Construct an `Error` with `StatusCode::PROXY_AUTHENTICATION_REQUIRED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! proxy_authentication_required {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::PROXY_AUTHENTICATION_REQUIRED))
    }}
}

/// Construct an `Error` with `StatusCode::REQUEST_TIMEOUT` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! request_timeout {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::REQUEST_TIMEOUT))
    }}
}

/// Construct an `Error` with `StatusCode::CONFLICT` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! conflict {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::CONFLICT))
    }}
}

/// Construct an `Error` with `StatusCode::GONE` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! gone {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::GONE))
    }}
}

/// Construct an `Error` with `StatusCode::LENGTH_REQUIRED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! length_required {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::LENGTH_REQUIRED))
    }}
}

/// Construct an `Error` with `StatusCode::PRECONDITION_FAILED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! precondition_failed {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::PRECONDITION_FAILED))
    }}
}

/// Construct an `Error` with `StatusCode::PAYLOAD_TOO_LARGE` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! payload_too_large {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::PAYLOAD_TOO_LARGE))
    }}
}

/// Construct an `Error` with `StatusCode::URI_TOO_LONG` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! uri_too_long {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::URI_TOO_LONG))
    }}
}

/// Construct an `Error` with `StatusCode::UNSUPPORTED_MEDIA_TYPE` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! unsupported_media_type {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::UNSUPPORTED_MEDIA_TYPE))
    }}
}

/// Construct an `Error` with `StatusCode::RANGE_NOT_SATISFIABLE` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! range_not_satisfiable {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::RANGE_NOT_SATISFIABLE))
    }}
}

/// Construct an `Error` with `StatusCode::EXPECTATION_FAILED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! expectation_failed {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::EXPECTATION_FAILED))
    }}
}

//  50x
/// Construct an `Error` with `StatusCode::INTERNAL_SERVER_ERROR` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! internal_server_error {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::INTERNAL_SERVER_ERROR))
    }}
}

/// Construct an `Error` with `StatusCode::NOT_IMPLEMENTED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! not_implemented {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::NOT_IMPLEMENTED))
    }}
}

/// Construct an `Error` with `StatusCode::BAD_GATEWAY` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! bad_gateway {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::BAD_GATEWAY))
    }}
}

/// Construct an `Error` with `StatusCode::SERVICE_UNAVAILABLE` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! service_unavailable {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::SERVICE_UNAVAILABLE))
    }}
}

/// Construct an `Error` with `StatusCode::GATEWAY_TIMEOUT` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! gateway_timeout {
    ($($arg:tt)*) => {{
        Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::GATEWAY_TIMEOUT))
    }}
}

/// Construct an `Error` with `StatusCode::HTTP_VERSION_NOT_SUPPORTED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_version_not_supported {
    ($($arg:tt)*) => {{
        Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::HTTP_VERSION_NOT_SUPPORTED))
    }}
}

/// Construct an `Error` with `StatusCode::VARIANT_ALSO_NEGOTIATES` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! variant_also_negotiates {
    ($($arg:tt)*) => {{
        Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::VARIANT_ALSO_NEGOTIATES))
    }}
}

/// Construct an `Error` with `StatusCode::INSUFFICIENT_STORAGE` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! insufficient_storage {
    ($($arg:tt)*) => {{
        Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::INSUFFICIENT_STORAGE))
    }}
}

/// Construct an `Error` with `StatusCode::LOOP_DETECTED` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! loop_detected {
    ($($arg:tt)*) => {{
        Error::from(anyhow::anyhow!($($arg)*)).with_status(Some(StatusCode::LOOP_DETECTED))
    }}
}
