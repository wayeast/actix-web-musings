extern crate actix_web;

use actix_web::{pred, server, App, HttpRequest};

fn index(req: &HttpRequest) -> String {
    format!("headers: {:?}\n", req.headers())
}

fn main() {
    server::new(|| App::new()
                .filter(pred::Header("Content-Type", "kra-kra"))
                .resource("/", |r| r.f(index)),
    ).bind("127.0.0.1:8088")
        .unwrap()
        .run()
}
