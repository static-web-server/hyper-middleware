/// Just an `hyper::Body` type alias.
pub type Body = hyper::Body;

/// Just an `hyper::Request<Body>` type alias.
pub type Request<T = Body> = hyper::Request<T>;

/// Just an `hyper::Response<Body>` type alias.
pub type Response<T = Body> = hyper::Response<T>;
