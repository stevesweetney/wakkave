use actix::prelude::*;
use actix_web::*;
use diesel::{
    self,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use failure::Error;
use bcrypt::{self, DEFAULT_COST};

use super::models::{Session, NewUser, User};
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
        use super::schema::sessions::dsl::*;
        diesel::insert_into(sessions)
            .values(&Session {id: msg.id})
            .get_result::<Session>(&self.0.get()?)
            .map_err(|_| ServerError::InsertToken.into())
    }
}

pub struct UpdateSession {
    pub old_id: String,
    pub new_id: String,
}

impl Message for UpdateSession {
    type Result = Result<Session, Error>;
}

impl Handler<UpdateSession> for DbExecutor {
    type Result = Result<Session, Error>;

    fn handle(&mut self, msg: UpdateSession, _: &mut Self::Context) -> Self::Result {
        use super::schema::sessions::dsl::*;
        diesel::update(sessions.filter(id.eq(&msg.old_id)))
            .set(id.eq(&msg.new_id))
            .get_result::<Session>(&self.0.get()?)
            .map_err(|_| ServerError::UpdateToken.into())
    }
}

pub struct CreateUser {
    pub username: String,
    pub password: String,
}

impl Message for CreateUser {
    type Result = Result<User, Error>;
}

impl Handler<CreateUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        use super::schema::users::dsl::*;
        diesel::insert_into(users)
            .values(&NewUser {
                username: msg.username, password: bcrypt::hash(&msg.password, DEFAULT_COST)?
            })
            .get_result::<User>(&self.0.get()?)
            .map_err(|_| ServerError::CreateUser.into())
    }
}