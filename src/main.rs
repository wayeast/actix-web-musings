extern crate actix_web;
use actix_web::{server, App, HttpResponse};

fn main() {
    let svr = server::new(|| {
        vec![
            App::new()
                .prefix("/app1")
                .resource("/", |r| r.f(|_| HttpResponse::Ok())),
            App::new()
                .prefix("/app2")
                .resource("/", |r| r.f(|_| HttpResponse::Ok())),
            App::new().resource("/", |r| r.f(|_| HttpResponse::Ok())),
        ]
    });

    svr.bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
