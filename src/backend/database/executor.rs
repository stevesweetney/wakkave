use actix::prelude::*;
use actix_web::*;
use diesel::{
    self,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use failure::Error;

use super::{models::Session, schema};
use backend::ServerError;

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub struct CreateSession {
    pub id: String,
}

impl Message for CreateSession {
    type Result = Result<Session, Error>;
}

impl Handler<CreateSession> for DbExecutor {
    type Result = Result<Session, Error>;

    fn handle(&mut self, msg: CreateSession, _: &mut Self::Context) -> Self::Result {
        use self::schema::sessions;

        diesel::insert_into(sessions::table)
            .values(&Session {id: msg.id})
            .get_result::<Session>(&self.0.get()?)
            .map_err(|_| ServerError::InsertToken.into())
    }
}