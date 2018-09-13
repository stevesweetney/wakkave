extern crate capnp;
extern crate serde;
extern crate wakkave;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate failure;

extern crate actix;

extern crate actix_web;

#[macro_use]
extern crate diesel;

extern crate r2d2;

extern crate jsonwebtoken;

extern crate uuid;

extern crate time;

extern crate futures;

extern crate bcrypt;

pub mod chatserver;
pub mod database;
pub mod server;
pub mod token;
pub mod websocket;

pub use wakkave::protocol_capnp;

use self::chatserver::ChatServer;
use self::database::executor::DbExecutor;
use actix::prelude::*;

pub struct State {
    pub db: Addr<DbExecutor>,
    pub chat: Addr<ChatServer>,
}

#[derive(Debug, Fail)]
pub enum ServerError {
    #[fail(display = "unable to create token")]
    CreateToken,

    #[fail(display = "Invalid Token")]
    VerifyToken,

    #[fail(display = "unable to insert token in the database")]
    InsertToken,

    #[fail(display = "unable to insert post in the database")]
    InsertPost,

    #[fail(display = "unable to update token in the database")]
    UpdateToken,

    #[fail(display = "unable to remove token from the database")]
    RemoveToken,

    #[fail(display = "unable to add user to chat")]
    JoinChat,

    #[fail(display = "unable to add a new user to the database")]
    CreateUser,

    #[fail(display = "unable to find user in the database")]
    FindUser,

    #[fail(display = "unable to insert vote in the database")]
    InsertVote,

    #[fail(display = "Password is incorrect")]
    IncorrectPassword,

    #[fail(display = "Invalid Vote")]
    InvalidVote,
}
