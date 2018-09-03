#![feature(custom_attribute)]
#![feature(generators)]

extern crate actix_web;
extern crate futures_await as futures;

use actix_web::*;
use futures::prelude::*;
use futures::future::ok;

fn handler(req: &HttpRequest) -> impl Responder {
    match req.method() {
        &http::Method::GET => HttpResponse::Ok(),
        _ => unimplemented!(),
    }
}

#[async]
fn async_handler1(_req: HttpRequest) -> Result<HttpResponse, error::Error> {
    Ok(HttpResponse::Ok().finish())
}

fn async_handler2(_req: HttpRequest) -> Box<Future<Item = HttpResponse, Error = error::Error>> {
    Box::new(ok(HttpResponse::Ok().finish()))
}

fn async_handler3(req: HttpRequest) -> Box<Future<Item = HttpResponse, Error = error::Error>> {
    Box::new(handler3_doer(req).map(|_| HttpResponse::Ok().finish()))
}

#[async]
fn handler3_doer(_req: HttpRequest) -> Result<& 'static str, error::Error> {
    Ok("kewl")
}

fn main() {
    server::new(|| {
        App::new()
            .handler("/handler", handler)
            // .route("/handler1", http::Method::GET, async_handler1)
            .route("/handler2", http::Method::GET, async_handler2)
            .route("/handler3", http::Method::GET, async_handler3)
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}

#[cfg(test)]
mod tests {
    use super::{async_handler1, async_handler2, async_handler3, handler, handler3_doer};
    use actix_web::*;
    use std::str;

    #[test]
    fn it_finally_works() {
        let resp = test::TestRequest::default()
            .run(&handler)
            .unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[test]
    fn yieeewww() {
        let resp = test::TestRequest::default()
            .run_async(&async_handler1)
            .unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[test]
    fn whuuuuh() {
        let resp = test::TestRequest::default()
            .run_async(&async_handler2)
            .unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[test]
    fn well_ill_be_damned() {
        let resp = test::TestRequest::default()
            .run_async(&async_handler3)
            .unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[test]
    fn well_ill_be_damned_again() {
        let resp = test::TestRequest::default()
            .run_async(&handler3_doer)
            .unwrap();
        match resp.body() {
            Body::Binary(bin) => {
                if let Binary::Slice(content) = bin {
                    assert_eq!(str::from_utf8(content).unwrap(), "kewl");
                }
            },
            _ => assert!(false),
        }
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
