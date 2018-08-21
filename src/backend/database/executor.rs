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

pub struct DeleteSession {
    pub session_id: String,
}

impl Message for DeleteSession {
    type Result = Result<(), Error>;
}

impl Handler<DeleteSession> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: DeleteSession, _: &mut Self::Context) -> Self::Result {
        use super::schema::sessions::dsl::*;
        diesel::delete(sessions.filter(id.eq(&msg.session_id)))
            .execute(&self.0.get()?)
            .map(|_| ())
            .map_err(|_| ServerError::RemoveToken.into())
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

pub struct FindUser {
    pub username: String,
    pub password: String,
}

impl Message for FindUser {
    type Result = Result<Option<User>, Error>;
}

impl Handler<FindUser> for DbExecutor {
    type Result = Result<Option<User>, Error>;

    fn handle(&mut self, msg: FindUser, _: &mut Self::Context) -> Self::Result {
        use super::schema::users::dsl::*;
        let user = users
            .filter(username.eq(msg.username))
            .select((id, username, password, karma, streak))
            .first::<User>(&self.0.get()?)
            .optional()
            .map_err(|_| ServerError::FindUser)?;
        
        match user {
            Some(u) => {
                if bcrypt::verify(&msg.password, &u.password)? {
                    Ok(Some(u))
                } else {
                    Err(ServerError::IncorrectPassword.into())
                }
            },
            None => Ok(None)
        }
    }
}

pub struct FindUserID {
    pub user_id: i32,
}

impl Message for FindUserID {
    type Result = Result<Option<User>, Error>;
}

impl Handler<FindUserID> for DbExecutor {
    type Result = Result<Option<User>, Error>;

    fn handle(&mut self, msg: FindUserID, _: &mut Self::Context) -> Self::Result {
        use super::schema::users::dsl::*;
        let user = users
            .filter(id.eq(msg.user_id))
            .select((id, username, password, karma, streak))
            .first::<User>(&self.0.get()?)
            .optional()
            .map_err(|_| ServerError::FindUser)?;
        
        match user {
            Some(u) => {
                Ok(Some(u))
            },
            None => Ok(None)
        }
    }
}