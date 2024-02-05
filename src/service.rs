//! The Hyper service module.
//!
//! It provides a [Hyper Service][`hyper::service::Service`] implementation intended to work with
//! the `hyper::server::Builder`.
//!
//! The service allows to bind a [`Middlewares`][`super::Middlewares`] of middlewares.
//!
//! ## Example
//!
//! ```rust
//! use hyper::Server;
//! use hyper_middleware::{
//!     async_trait, Body, Handler, Request, Response, Middlewares, Result, Service,
//! };
//!
//! struct Application {}
//!
//! #[async_trait]
//! impl Handler for Application {
//!     async fn handle(&self, _req: &mut Request) -> Result<Response> {
//!         // Create a response and send it back to the middlewares chain
//!         Ok(Response::new(Body::from("¡Hola!")))
//!     }
//! }
//!
//! #[tokio::main(flavor = "multi_thread")]
//! async fn main() -> Result {
//!     let mut middlewares = Middlewares::new(Application {});
//!
//!     let service = Service::new(middlewares);
//!
//!     let addr = ([127, 0, 0, 1], 8087).into();
//!     let server = Server::bind(&addr).serve(service);
//!
//!     println!("Listening on http://{}", addr);
//!
//!     // server.await?;
//!
//!     Ok(())
//! }
//! ```

use hyper::service::Service as HyperService;
use std::convert::Infallible;
use std::future::{ready, Ready};
use std::task::{Context, Poll};

use self::handler_service::{HandlerService, HandlerServiceBuilder};
use crate::middleware::Handler;
use crate::remote_addr::RemoteAddr;

/// A [Hyper Service][`hyper::service::Service`] entry point which hosts a [`Handler`].
pub struct Service<H> {
    builder: HandlerServiceBuilder<H>,
}

impl<H> Service<H>
where
    H: Handler,
{
    /// Create a new Service instance which will be used when create a Hyper server.
    pub fn new(handler: H) -> Self {
        Self {
            builder: HandlerServiceBuilder::new(handler),
        }
    }
}

impl<H, T> HyperService<&T> for Service<H>
where
    H: Handler,
    T: RemoteAddr + Send + 'static,
{
    type Response = HandlerService<H>;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, conn: &T) -> Self::Future {
        ready(Ok(self.builder.build(conn.remote_addr())))
    }
}

mod handler_service {
    use std::future::Future;
    use std::net::SocketAddr;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::task::{Context, Poll};

    use crate::error::{Error, Result};
    use crate::http::{Request, Response};
    use crate::middleware::Handler;
    use crate::service::HyperService;

    pub struct HandlerService<H> {
        handler: Arc<H>,
        remote_addr: Option<SocketAddr>,
    }

    impl<H> HyperService<Request> for HandlerService<H>
    where
        H: Handler,
    {
        type Response = Response;
        type Error = Error;
        type Future = Pin<Box<dyn Future<Output = Result<Response>> + Send + 'static>>;

        fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<()>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, mut req: Request) -> Self::Future {
            if let Some(remote_addr) = self.remote_addr {
                req.extensions_mut().insert(remote_addr);
            }
            let handler = self.handler.clone();
            Box::pin(async move { handler.handle(&mut req).await })
        }
    }

    pub struct HandlerServiceBuilder<H> {
        handler: Arc<H>,
    }

    impl<H> HandlerServiceBuilder<H>
    where
        H: Handler + Send + Sync + 'static,
    {
        pub fn new(handler: H) -> Self {
            Self {
                handler: Arc::new(handler),
            }
        }

        pub fn build(&self, remote_addr: Option<SocketAddr>) -> HandlerService<H> {
            HandlerService {
                handler: self.handler.clone(),
                remote_addr,
            }
        }
    }
}
