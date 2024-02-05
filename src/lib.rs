#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![deny(warnings)]
#![deny(rust_2018_idioms)]
#![deny(dead_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # hyper_middleware
//!
//! `hyper_middleware` is a complement set of HTTP middlewares with a small Hyper Service to get started with a HTTP server easily.
//! This module only supports Hyper `0.14`.
//!
//! ## Features
//!
//! - Compact [Middleware & Handler System][`middleware`] inspired by [The Iron Framework](https://github.com/iron/iron).
//! - Simple [Hyper Service][`hyper::service::Service`] with [Remote Address][`hyper::server::conn::AddrStream`] access.
//! - Convenient [`Error`] and [`Result`] types powered by [anyhow](https://github.com/dtolnay/anyhow).
//! - `Async` support via [async-trait](https://github.com/dtolnay/async-trait).
//! - Macros to facilitate HTTP response errors or error casting.
//!
//! Check it out [`middleware`] module for more details.
//!

pub mod error;
pub mod http;
pub mod middleware;
pub mod remote_addr;
pub mod service;

pub use error::{Context, Error, Result};
pub use http::*;
pub use middleware::*;
pub use remote_addr::*;
pub use service::*;

// Re-export crates
pub use async_recursion::*;
pub use async_trait::*;
