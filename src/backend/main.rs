extern crate actix;
extern crate actix_web;

use actix::prelude::*;
use actix_web::{
    ws::{
        Message, ProtocolError, WebsocketContext
    },
    ws,server, App, HttpRequest, 
};

struct Ws;

impl Actor for Ws {
    type Context = WebsocketContext<Self>;
}

impl StreamHandler<Message, ProtocolError> for Ws {
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        match msg {
            Message::Text(text) => {
                self.handle_request(&text, ctx);
            },
            Message::Close(reason) => {
                ctx.stop();
            },
            _ => (),
        };
    }
}

impl Ws {
    fn handle_request(&self, text: &str, ctx: &mut WebsocketContext<Self>) {
        match text {
            "Login" => ctx.text("You have sucessfully logged in!"),
            "Logout" => ctx.text("You have sucessfully logged out"),
            _ => (),
        }
    }
}

fn create_user(_req: HttpRequest) -> & 'static str {
    "Thank you for creating a user!"
}

fn main() {
    server::new(|| App::new()
        .resource("/", |r| r.f(|req| ws::start(req, Ws)))
        .resource("/users/new", |r| r.f(create_user)))
            .bind("127.0.0.1:8088")
            .unwrap()
            .run();
}