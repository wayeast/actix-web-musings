extern crate actix_web;

use actix_web::{server, App, HttpResponse};

// NB: this example, copied directly from the actix-web getting started
//  guide, is intended to show how all App instances in a server need to
//  be of the same type (eg. App<State1>).  Try removing the `.boxed()`
//  directives to see what happens.  Also, try removing one and leaving
//  the other.
struct State1;
struct State2;

fn main() {
    server::new(|| {
        vec![
            App::with_state(State1)
                .prefix("/app1")
                .resource("/", |r| r.f(|_| HttpResponse::Ok()))
                .boxed(),
            App::with_state(State2)
                .prefix("/app2")
                .resource("/", |r| r.f(|_| HttpResponse::Ok()))
                .boxed(),
                ]
    }).bind("127.0.0.1:8088")
        .unwrap()
        .run()
}
