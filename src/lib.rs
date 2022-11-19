// #![deny(missing_docs)]

//! # hyper_middleware
//!
//! `hyper_middleware` is a complement set of HTTP middlewares with a small Hyper Service to get started with a HTTP server easily.
//! This module only supports Hyper `0.14.x`.
//!
//! ## Features
//!
//! - Compact Middleware and Handler System inspired by [The Iron Framework](https://github.com/iron/iron).
//! - Simple [Hyper Service](https://docs.rs/hyper/latest/hyper/service/trait.Service.html) with convenient __Remote Address__ access.
//! - Convenient `Error` and `Result` types powered by [anyhow](https://github.com/dtolnay/anyhow).

mod middleware;
mod service;
mod types;

pub use middleware::*;
pub use service::*;
pub use types::*;
