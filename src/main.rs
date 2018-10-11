extern crate hyper;

use hyper::{Body, Method, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

static TEXT: &str = "Hello, World!";

fn square_root(square: i32) -> f32 {
    let delta = 0.00001;
    let mut last_guess = 0.0;
    let mut current_guess = 1.0;
    let mut difference = current_guess - last_guess as f32;

    while difference.abs() > delta {
        last_guess = current_guess;
        current_guess = (last_guess + (square as f32 / last_guess) ) / 2.0;
        difference = current_guess - last_guess;
    }

    return current_guess;
}

fn main() {
    let port = 3000;
    let addr = ([127, 0, 0, 1], port).into();

    println!("Listening on localhost:3000.");
    let new_svc = || {
        service_fn_ok(|req|{
            println!("{:?}", req);
            match req.method() {
                &Method::GET => Response::new(Body::from(TEXT)),
                &Method::POST => Response::new(Body::from(format!("{}", square_root(17)))),
                _ => Response::new(Body::from("WTF is that supposed to be?"))
            }

        })
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
