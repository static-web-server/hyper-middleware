// #![deny(missing_docs)]

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
//!

pub mod middleware;
pub mod remote_addr;
pub mod service;
pub mod types;

pub use middleware::*;
pub use remote_addr::*;
pub use service::*;
pub use types::*;
