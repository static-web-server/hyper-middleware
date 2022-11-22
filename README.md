# Hyper Middleware

[![hyper-middleware crate](https://img.shields.io/crates/v/hyper-middleware.svg)](https://crates.io/crates/hyper-middleware)
[![Released API docs](https://docs.rs/hyper-middleware/badge.svg)](https://docs.rs/hyper-middleware)
[![hyper-middleware crate license](https://img.shields.io/crates/l/hyper-middleware)](./LICENSE-MIT)

> A compact HTTP middleware and handler system for [Hyper](https://github.com/hyperium/hyper) `0.14.x`.<br>
> **NOTE:** This crate is still under active development.

## Features

- Compact Middleware and Handler System inspired by [The Iron Framework](https://github.com/iron/iron).
- Simple [Hyper Service](https://docs.rs/hyper/latest/hyper/service/trait.Service.html) with convenient __Remote Address__ access.
- Convenient `Error` and `Result` types powered by [anyhow](https://github.com/dtolnay/anyhow).

## Example

[examples/server.rs](examples/server.rs)

```rust
#![deny(warnings)]

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
        Ok(Response::new(Body::from("¡Hola!")))
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

To run the example just type:

```
cargo run --example server
```

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in current work by you, as defined in the Apache-2.0 license, shall be dual licensed as described below, without any additional terms or conditions.

Feel free to send some [Pull request](https://github.com/static-web-server/hyper-middleware/pulls) or file an [issue](https://github.com/static-web-server/hyper-middleware/issues).

## License

This work is primarily distributed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

© 2022-present [Jose Quintana](https://joseluisq.net)
