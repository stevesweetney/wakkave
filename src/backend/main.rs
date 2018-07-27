extern crate actix_web;
extern crate wakkave;

use actix_web::{ ws, server, App, HttpRequest }; 
use wakkave::backend::Ws;

fn create_user(_req: HttpRequest) -> & 'static str {
    "Thank you for creating a user!"
}

fn main() {
    server::new(|| App::new()
        .resource("/", |r| r.f(|req| ws::start(req, Ws::new())))
        .resource("/users/new", |r| r.f(create_user)))
            .bind("127.0.0.1:8088")
            .unwrap()
            .run();
}