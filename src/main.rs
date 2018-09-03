extern crate actix_web;

use actix_web::*;

fn handler(_: &HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

fn main() {
    server::new(|| {
        App::new()
            .handler("/handler", handler)
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}

#[cfg(test)]
mod tests {
    use super::handler;
    use actix_web::*;

    #[test]
    fn it_finally_works() {
        let resp = test::TestRequest::default()
            .run(&handler)
            .unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
