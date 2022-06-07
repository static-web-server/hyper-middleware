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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.source, f)
    }
}

impl Error {
    /// Construct an ad-hoc an HTTP `Error` from another error type.
    pub fn from_err(error: anyhow::Error, status: Option<StatusCode>) -> Self {
        Self {
            source: error,
            status,
        }
    }

    /// Construct an ad-hoc an HTTP `Error` from a string error.
    pub fn from_str(error: &str, status: Option<StatusCode>) -> Self {
        Self {
            source: anyhow::anyhow!(error.to_owned()),
            status,
        }
    }

    /// Gets a reference to the `Error` source.
    pub fn source(&self) -> &anyhow::Error {
        &self.source
    }

    /// Gets the HTTP status code associated with the error.
    pub fn status(&self) -> Option<StatusCode> {
        self.status
    }

    /// Sets the HTTP status code associated with the error.
    pub fn status_mut(&mut self) -> &mut Option<StatusCode> {
        &mut self.status
    }
}

impl From<anyhow::Error> for Error {
    /// Converts whatever error into an HTTP `Error`.
    fn from(error: anyhow::Error) -> Self {
        Self {
            source: error,
            status: None,
        }
    }
}

/// Construct an ad-hoc `Error` from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        Error::from(anyhow::anyhow!($($arg)*))
    }}
}
