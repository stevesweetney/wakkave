pub mod token;
pub mod websocket;
pub mod database;
pub mod server;

use actix::prelude::*;
use self::database::executor::DbExecutor;

pub struct State {
    pub db: Addr<Syn, DbExecutor>,
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
}
