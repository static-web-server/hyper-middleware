//! The custom error module.
//!
//! This module provides a custom [`Error`][`super::Error`] type with HTTP Status functionality as well as useful macros.
//! The `Error` type implements other several common error types
//! to ease conversions while consuming the input value via the [`From`] trait.
//!
//! Additionally, when used in HTTP contexts, the `Error` type can be associated to an [HTTP Status Code][`http::StatusCode`]
//! via the [`Error::with_status`][`super::Error::with_status`] method.

use hyper::StatusCode;
use std::fmt;
use thiserror::Error as ThisError;

pub mod macros;

pub type Result<T = (), E = Error> = anyhow::Result<T, E>;
pub use anyhow::Context;

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

impl From<std::io::Error> for Error {
    /// Converts an error type that implements [`std::io::Error`] into an HTTP [`Error`].
    fn from(source: std::io::Error) -> Self {
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
