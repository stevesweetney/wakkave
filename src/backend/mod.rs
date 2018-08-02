pub mod token;
pub mod database;

use actix::prelude::*;
use actix_web::{
    ws::{ Message, ProtocolError, WebsocketContext },
    Binary,
};

use capnp::{
    message::{ Builder, HeapAllocator, ReaderOptions },
    serialize_packed
};

use backend::token::Token;

use protocol_capnp::{request, response};

use std::default::Default;

use failure::Error;

#[derive(Debug, Fail)]
pub enum ServerError {
    #[fail(display = "unable to create token")]
    CreateToken,

   #[fail(display = "Invalid Token")]
    VerifyToken, 

    #[fail(display = "unable to insert token in the database")]
    InsertToken, 
}

pub struct Ws {
    data: Vec<u8>,
    builder: Builder<HeapAllocator>,
}

impl Default for Ws {
    fn default() -> Self {
        Self::new()
    }
}


impl Actor for Ws {
    type Context = WebsocketContext<Self>;
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
        }
    }

    fn handle_request(&mut self, data: &Binary, ctx: &mut WebsocketContext<Self>) {
        let reader = serialize_packed::read_message(&mut data.as_ref(), ReaderOptions::new())
            .expect("Error reading message");

        let request = reader.get_root::<request::Reader>()
            .expect("Error getting message root");

        match request.which() {
            Ok(request::Login(data)) => {
                match data.which() {
                    Ok(request::login::Credentials(data)) => {
                        if let Err(e) = self.handle_request_login_credentials(data) {
                            println!("Error: {:?}", e);
                        }

                        self.send(ctx);
                    }
                    Ok(request::login::Token(data)) => {
                        println!("{}", data.unwrap());
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

    fn send(&self, ctx: &mut WebsocketContext<Self>) {
        ctx.binary(self.data.clone());
    }

    fn handle_request_login_credentials(&mut self, data: request::login::credentials::Reader) -> Result<(), Error> {
        let name = data.get_username()?;
        let password = data.get_password()?;
        println!("Name: {} \nPassword: {}", name, password);

        self.builder
            .init_root::<response::Builder>()
            .init_login()
            .set_token(&Token::create(name)?);

        self.write()
    }
}