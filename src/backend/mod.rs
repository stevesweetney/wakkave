use actix::prelude::*;
use actix_web::ws::{ Message, ProtocolError, WebsocketContext };

pub struct Ws;

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