//! Set of HTTP types aliases for convenience.

/// A [`hyper::Body`] type alias.
pub type Body = hyper::Body;

/// A [`hyper::Request<Body>`] type alias with defaults.
pub type Request<T = Body> = hyper::Request<T>;

/// A [`hyper::Response<Body>`] type alias with defaults.
pub type Response<T = Body> = hyper::Response<T>;
