use actix::{prelude::*, fut};
use actix_web::{
    ws::{ Message, ProtocolError, WebsocketContext },
    Binary,
};

use capnp::{
    self,
    message::{ Builder, HeapAllocator, ReaderOptions },
    serialize_packed,
    text
};

use backend::{
    State, 
    token::Token,
    database::executor::{CreateSession, UpdateSession},
    chatserver,
};

use protocol_capnp::{request, response};

use std::default::Default;

use failure::Error;
use futures::future::Future;

pub struct Ws {
    data: Vec<u8>,
    builder: Builder<HeapAllocator>,
    id: Option<String>,
}

impl Default for Ws {
    fn default() -> Self {
        Self::new()
    }
}


impl Actor for Ws {
    type Context = WebsocketContext<Self, State>;

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        // notify the chat server
        if let Some(ref id) = self.id {
            ctx.state().chat.do_send(chatserver::Disconnect { id: id.to_owned() });
        }

        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<chatserver::ServerMessage> for Ws {
    type Result = ();

    fn handle(&mut self, msg: chatserver::ServerMessage, ctx: &mut Self::Context) {
        ctx.binary(msg.0);
    }
}


impl StreamHandler<Message, ProtocolError> for Ws {
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        match msg {
            Message::Text(text) => {
                ctx.text(text);
            },
            Message::Binary(bin) => {
                self.handle_request(&bin, ctx);
            }
            Message::Close(_reason) => {
                ctx.stop();
            },
            _ => (),
        };
    }
}

impl Ws {
    pub fn new() -> Self {
        Ws {
            data: Vec::new(),
            builder: Builder::new_default(),
            id: None,
        }
    }

    fn handle_request(&mut self, data: &Binary, ctx: &mut WebsocketContext<Self, State>) {
        let reader = serialize_packed::read_message(&mut data.as_ref(), ReaderOptions::new())
            .expect("Error reading message");

        let request = reader.get_root::<request::Reader>()
            .expect("Error getting message root");

        match request.which() {
            Ok(request::Login(data)) => {
                match data.which() {
                    Ok(request::login::Credentials(data)) => {
                        match self.handle_request_login_credentials(data, ctx) {
                            Ok(()) => self.connect_to_chat(ctx),
                            Err(e) => println!("Error: {:?}", e),
                        }

                        self.send(ctx);
                    }
                    Ok(request::login::Token(data)) => {
                         match self.handle_request_login_token(data, ctx) {
                            Ok(()) => self.connect_to_chat(ctx),
                            Err(e) => println!("Error: {:?}", e),
                        }

                        self.send(ctx);
                    }
                    Err(::capnp::NotInSchema(_)) => (),
                }
            }
            Ok(request::Logout(_data)) => (),
            Err(::capnp::NotInSchema(_)) => (),
        }
    }

    fn write(&mut self) -> Result<(), Error> {
        self.data.clear();

        serialize_packed::write_message(&mut self.data, &self.builder)?;
        Ok(())
    }

    fn send(&self, ctx: &mut WebsocketContext<Self, State>) {
        ctx.binary(self.data.clone());
    }

    fn connect_to_chat(&self, ctx: &mut WebsocketContext<Self, State>) {
        let addr = ctx.address();
        ctx.state()
            .chat
            .send(chatserver::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = Some(res),
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ok(())
            })
            .wait(ctx);
    }

    fn handle_request_login_credentials(&mut self, data: request::login::credentials::Reader, ctx: &mut WebsocketContext<Self, State>) -> Result<(), Error> {
        let name = data.get_username()?;
        let password = data.get_password()?;
        println!("Name: {} \nPassword: {}", name, password);

        let token = ctx.state().db.send(CreateSession {
            id: Token::create(name)?,
        }).wait()??;

        self.builder
            .init_root::<response::Builder>()
            .init_login()
            .set_token(&token.id);

        self.write()
    }

    fn handle_request_login_token(&mut self, data: Result<text::Reader, capnp::Error>, ctx: &mut WebsocketContext<Self, State>) -> Result<(), Error> {
        let token = data?;
        println!("Renewing Token: {} \n", token);


        let new_token = ctx.state().db.send(UpdateSession {
            old_id: token.to_string(),
            new_id: Token::verify(token)?,
        }).wait()??;

        self.builder
            .init_root::<response::Builder>()
            .init_login()
            .set_token(&new_token.id);

        self.write()
    }
}