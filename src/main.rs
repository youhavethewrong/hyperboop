extern crate hyper;

use hyper::{Body, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

static TEXT: &str = "Hello, World!";

fn main() {
    let port = 3000;
    let addr = ([127, 0, 0, 1], port).into();

    println!("Listening on localhost:3000.");
    let new_svc = || {
        service_fn_ok(|_req|{
            Response::new(Body::from(TEXT))
        })
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
