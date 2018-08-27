extern crate actix_web;

use actix_web::{server, App, HttpRequest};

fn index(req: &HttpRequest) -> String {
    format!("path: '{}'\n", req.path())
}

fn main() {
    server::new(|| {
        vec![
            App::new()
                .prefix("/prefixed")
                .resource("/", |r| r.f(index)),
            App::new()
                .resource("/", |r| r.f(index)),
        ]
    }).bind("127.0.0.1:8088")
        .unwrap()
        .run()
}
