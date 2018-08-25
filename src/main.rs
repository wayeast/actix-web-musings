extern crate actix_web;

use actix_web::{http, server, App, HttpRequest};
use std::sync::{Arc, Mutex};

struct State {
    counter: Arc<Mutex<usize>>,
}

fn index(req: &HttpRequest<State>) -> String {
    let mut count = req.state().counter.lock().unwrap();
    *count += 1;

    format!("Current count: {}", count)
}

fn main() {
    let count = Arc::new(Mutex::new(0_usize));

    server::new(move || {
        App::with_state(State { counter: Arc::clone(&count) })
            .resource("/", |r| r.method(http::Method::GET).f(index))
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
}
