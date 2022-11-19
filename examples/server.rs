#![forbid(unsafe_code)]
#![deny(warnings)]
#![deny(rust_2018_idioms)]
#![deny(dead_code)]

use hyper::{header, Server, StatusCode};
use std::{net::SocketAddr, path::PathBuf};

use hyper_middleware::{
    AfterMiddleware, BeforeMiddleware, Body, Chain, Error, Handler, Request, Response, Result,
    Service,
};

struct Config {
    pub root: PathBuf,
}

struct Application {
    opts: Config,
}

impl Handler for Application {
    fn handle(&self, req: &mut Request) -> Result<Response> {
        // Access the Hyper incoming Request
        println!("Handler - URI Path: {}", req.uri().path());

        // Access the custom app options
        println!("Config Root: {}", self.opts.root.display());

        // Access the Remote Address
        println!(
            "Remote Addr: {}",
            req.extensions().get::<SocketAddr>().unwrap()
        );

        // Create a Hyper Response and send it back to the middlewares chain
        Ok(Response::builder().body(Body::from("¡Hola!")).unwrap())
    }
}

struct FirstMiddleware {}

impl BeforeMiddleware for FirstMiddleware {
    fn before(&self, req: &mut Request) -> Result {
        println!("First Middleware called!");

        // Access the Hyper incoming Request
        println!("First - URI Path: {}", req.uri().path());

        Ok(())
    }

    fn catch(&self, _: &mut Request, err: Error) -> Result {
        Err(err)
    }
}

struct SecondMiddleware {}

impl AfterMiddleware for SecondMiddleware {
    fn after(&self, _: &mut Request, mut res: Response) -> Result<Response> {
        println!("Second Middleware called!");

        // Mutate the Hyper Response at convenience
        // and send it back to other middlewares on the chain
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            "text/html; charset=utf-8".parse().unwrap(),
        );

        Ok(res)
    }

    fn catch(&self, _: &mut Request, err: Error) -> Result<Response> {
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

    // 1. Create a custom middleware chain
    let mut handler = Chain::new(Application { opts });

    // 2. Plug in some custom middlewares
    handler.link_before(FirstMiddleware {});
    handler.link_after(SecondMiddleware {});

    // 3. Create an Hyper service and set the current handler with its middlewares
    let service = Service::new(handler);

    // 4. Finally just run server using the service already created
    let addr = ([127, 0, 0, 1], 8787).into();
    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
