// TODO: #![deny(missing_docs)]

//! # hyper_middleware
//!
//! `hyper_middleware` is a complement set of middlewares with a small Hyper service to get started an HTTP server easily.
//! This module only supports Hyper `0.14.x`.
//!
//! ## Features
//!
//! - Compact Middleware and Handler System inspired by [The Iron Framework](https://github.com/iron/iron).
//! - Simple Hyper Service with convenient __Remote Address__ access.
//! - Error and return types powered by [anyhow](https://github.com/dtolnay/anyhow).

pub mod middleware;
pub mod service;
pub mod types;

#[macro_use]
pub mod error;

pub use error::*;
pub use middleware::*;
pub use service::*;
pub use types::*;
