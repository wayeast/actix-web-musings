extern crate actix_web;
extern crate futures;

use actix_web::*;
use futures::future::{Future, result};

fn index(_: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {

    result(Ok(HttpResponse::Ok()
              .content_type("text/html")
              .body(format!("Hello!"))))
           .responder()
}

fn index2(_: &HttpRequest) -> Box<Future<Item=&'static str, Error=Error>> {
    result(Ok("Welcome!"))
        .responder()
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/async1", |r| r.route().a(index))
            .resource("/async2", |r| r.route().a(index2))
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
