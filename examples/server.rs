#![forbid(unsafe_code)]
#![deny(warnings)]
#![deny(rust_2018_idioms)]
#![deny(dead_code)]

use hyper::{header, Server, StatusCode};
use hyper_middleware::{
    async_trait, AfterMiddleware, BeforeMiddleware, Body, Error, Handler, Middlewares, Request,
    Response, Result, Service,
};
use std::{net::SocketAddr, path::PathBuf};

struct Config {
    pub root: PathBuf,
}

struct Application {
    opts: Config,
}

#[async_trait]
impl Handler for Application {
    async fn handle(&self, req: &mut Request) -> Result<Response> {
        // Access the Hyper incoming Request
        println!("Application::handle() - URI Path: {}", req.uri().path());

        // Access the custom app options
        println!(
            "Application::handle() - Config Root: {}",
            self.opts.root.display()
        );

        // Access the Remote Address
        let remote_addr = req.extensions().get::<SocketAddr>().unwrap();
        println!("Application::handle() - Remote Addr: {}", remote_addr);

        // Create a Hyper Response and send it back to the middlewares chain
        Ok(Response::new(Body::from("¡Hola!")))
    }
}

struct FirstMiddleware {}

#[async_trait]
impl BeforeMiddleware for FirstMiddleware {
    async fn before(&self, req: &mut Request) -> Result {
        println!("FirstMiddleware::before()");

        // Access the Hyper incoming Request
        println!("FirstMiddleware::before() - URI Path: {}", req.uri().path());

        Ok(())
    }

    async fn catch(&self, _: &mut Request, err: Error) -> Result {
        Err(err)
    }
}

struct SecondMiddleware {}

#[async_trait]
impl AfterMiddleware for SecondMiddleware {
    async fn after(&self, _: &mut Request, mut res: Response) -> Result<Response> {
        println!("SecondMiddleware::after()");

        // Mutate the Hyper Response at convenience
        // and send it back to other middlewares on the chain
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            "text/html; charset=utf-8".parse().unwrap(),
        );

        Ok(res)
    }

    async fn catch(&self, _: &mut Request, err: Error) -> Result<Response> {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(err.to_string()))
            .unwrap())
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result {
    // 0. Define some app options (example only)
    let opts = Config {
        root: std::env::current_dir().unwrap(),
    };

    // 1. Create a custom middleware chain and plug in some custom middlewares
    let mut middlewares = Middlewares::new(Application { opts });
    middlewares.link_before(FirstMiddleware {});
    middlewares.link_after(SecondMiddleware {});

    // 2. Create a Hyper service and set the current handler with its middlewares
    let service = Service::new(middlewares);

    // 3. Finally just run server using the service already created
    let addr = ([127, 0, 0, 1], 8787).into();
    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
