use actix::prelude::*;
use actix_web::*;
use bcrypt::{self, DEFAULT_COST};
use diesel::{
    self,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use failure::Error;

use super::models::{NewPost, NewUser, Post, Session, User, Vote};
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
            .values(&Session { id: msg.id })
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
                username: msg.username,
                password: bcrypt::hash(&msg.password, DEFAULT_COST)?,
            }).get_result::<User>(&self.0.get()?)
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
            }
            None => Ok(None),
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
            Some(u) => Ok(Some(u)),
            None => Ok(None),
        }
    }
}

pub struct CreatePost {
    content: String,
    user_id: i32,
}

impl Message for CreatePost {
    type Result = Result<(Post, String), Error>;
}

impl Handler<CreatePost> for DbExecutor {
    type Result = Result<(Post, String), Error>;

    fn handle(&mut self, msg: CreatePost, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;
        let post = {
            use super::schema::posts::dsl::*;
            diesel::insert_into(posts)
                .values(&NewPost {
                    content: msg.content.clone(),
                    user_id: msg.user_id,
                }).get_result::<Post>(&conn)
                .map_err(|_| ServerError::InsertPost)?
        };

        let name = {
            use super::schema::users::dsl::*;
            users
                .filter(id.eq(msg.user_id))
                .select(username)
                .first::<String>(&self.0.get()?)
                .map_err(|_| ServerError::FindUser)?
        };

        Ok((post, name))
    }
}

pub struct UserVote {
    pub post_id: i32,
    pub user_id: i32,
    // 1 will represent an upvote, -1 will represent a downvote
    pub up_or_down: i16,
}

impl Message for UserVote {
    type Result = Result<(), Error>;
}

impl Handler<UserVote> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: UserVote, _: &mut Self::Context) -> Self::Result {
        use super::schema::votes::dsl::*;
        diesel::insert_into(votes)
            .values(&Vote {
                post_id: msg.post_id,
                user_id: msg.user_id,
                up_or_down: msg.up_or_down,
            }).on_conflict((post_id, user_id))
            .do_update()
            .set(up_or_down.eq(msg.up_or_down))
            .execute(&self.0.get()?)
            .map_err(|_| ServerError::InsertVote.into())
            .map(|_| ())
    }
}
