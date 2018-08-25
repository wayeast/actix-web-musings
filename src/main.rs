extern crate actix_web;

use actix_web::{http, server, App, HttpRequest};
use std::cell::Cell;

// NB: testing this with curl requests may look funny.  On my machine,
//  actix-web starts 8 server threads.  Since each thread has its own
//  state, I had to submit 8 curl requests before seeing count increment.
struct State {
    counter: Cell<usize>,
}

fn index(req: &HttpRequest<State>) -> String {
    let current_count = req.state().counter.get();
    let updated_count = current_count + 1;
    req.state().counter.set(updated_count);

    format!("Current count: {}", updated_count)
}

fn main() {
    server::new(|| {
        App::with_state(State { counter: Cell::new(0) })
            .resource("/", |r| r.method(http::Method::GET).f(index))
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
}
