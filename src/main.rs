extern crate actix_web;
use actix_web::{server, App, HttpRequest};

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn create_user(_req: HttpRequest) -> & 'static str {
    "Thank you for creating a user!"
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(index))
        .resource("/users/new", |r| r.f(create_user)))
            .bind("127.0.0.1:8088")
            .unwrap()
            .run();
}