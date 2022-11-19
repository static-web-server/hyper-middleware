use hyper::server::conn::AddrStream;
use hyper::service::Service as HyperService;
use std::convert::Infallible;
use std::future::{ready, Ready};
use std::task::{Context, Poll};

use self::handler_service::{HandlerService, HandlerServiceBuilder};
use crate::middleware::Handler;

/// Hyper Service entry point which hosts a `Handler`.
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

impl<H> HyperService<&AddrStream> for Service<H>
where
    H: Handler,
{
    type Response = HandlerService<H>;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, conn: &AddrStream) -> Self::Future {
        ready(Ok(self.builder.build(conn.remote_addr())))
    }
}

mod handler_service {
    use std::future::Future;
    use std::net::SocketAddr;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::task::{Context, Poll};

    use crate::middleware::Handler;
    use crate::service::HyperService;
    use crate::types::{Error, Request, Response, Result};

    pub struct HandlerService<H> {
        handler: Arc<H>,
        remote_addr: SocketAddr,
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
            req.extensions_mut().insert(self.remote_addr);
            let res = self.handler.handle(&mut req);
            Box::pin(async { res })
        }
    }

    pub struct HandlerServiceBuilder<H> {
        handler: Arc<H>,
    }

    impl<H> HandlerServiceBuilder<H>
    where
        H: Handler,
    {
        pub fn new(handler: H) -> Self {
            Self {
                handler: Arc::new(handler),
            }
        }

        pub fn build(&self, remote_addr: SocketAddr) -> HandlerService<H> {
            HandlerService {
                handler: self.handler.clone(),
                remote_addr,
            }
        }
    }
}
