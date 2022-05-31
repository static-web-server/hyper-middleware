# Hyper Middleware

> A compact middleware and handler system for [Hyper](https://github.com/hyperium/hyper) `0.14.x`.

## Features

- Compact Middleware and Handler System inspired by [The Iron Framework](https://github.com/iron/iron).
- Simple Hyper Service with convenient __Remote Address__ access.
- Error and return types powered by [anyhow](https://github.com/dtolnay/anyhow).

## Example

[src/bin/server.rs](src/bin/server.rs)

```rust
#![deny(warnings)]

use hyper::{Server, StatusCode};
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
        // Access Hyper incoming Request
        println!("URI Path: {}", req.uri().path());

        // Access custom app options
        println!("Config Root: {}", self.opts.root.display());

        // Access the Remote Address
        println!(
            "Remote Addr: {}",
            req.extensions().get::<SocketAddr>().unwrap()
        );

        // Send a response back to the middlewares chain
        Ok(Response::builder().body(Body::from("¡Hola!")).unwrap())
    }
}

struct FirstMiddleware {}

impl BeforeMiddleware for FirstMiddleware {
    fn before(&self, _: &mut Request) -> Result<()> {
        println!("First Middleware called!");
        Ok(())
    }

    fn catch(&self, _: &mut Request, err: Error) -> Result<()> {
        Err(err)
    }
}

struct SecondMiddleware {}

impl AfterMiddleware for SecondMiddleware {
    fn after(&self, _: &mut Request, res: Response) -> Result<Response> {
        println!("Second Middleware called!");
        Ok(res)
    }

    fn catch(&self, _: &mut Request, err: Error) -> Result<Response> {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(err.to_string()))
            .unwrap())
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 32)]
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

    // 3. Create a Hyper service and set the current handler with its middlewares
    let service = Service::new(handler);

    // 4. Finally just run server using the service already created
    let addr = ([127, 0, 0, 1], 8787).into();
    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
```

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in current work by you, as defined in the Apache-2.0 license, shall be dual licensed as described below, without any additional terms or conditions.

Feel free to send some [Pull request](https://github.com/static-web-server/hyper-middleware/pulls) or [issue](https://github.com/static-web-server/hyper-middleware/issues).


## License

This work is primarily distributed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

© 2022-present [Jose Quintana](https://joseluisq.net)
