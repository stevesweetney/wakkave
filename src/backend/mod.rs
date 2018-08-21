pub mod token;
pub mod websocket;
pub mod database;
pub mod server;
pub mod chatserver;

use actix::prelude::*;
use self::database::executor::DbExecutor;
use self::chatserver::ChatServer;

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

    #[fail(display = "Password is incorrect")]
    IncorrectPassword, 
}
