use actix::prelude::*;
use actix_web::*;
use bcrypt::{self, DEFAULT_COST};
use diesel::{
    self,
    dsl::{now, IntervalDsl},
    pg::expression::dsl::any,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use failure::Error;
use std::cmp::Ordering;

use super::models::{NewPost, NewUser, Post, Session, User, Vote};
use ServerError;

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
    pub content: String,
    pub user_id: i32,
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

pub struct FetchPosts {
    pub user_id: i32,
}

impl Message for FetchPosts {
    type Result = Result<Vec<(Post, Option<Vote>)>, Error>;
}

impl Handler<FetchPosts> for DbExecutor {
    type Result = Result<Vec<(Post, Option<Vote>)>, Error>;

    fn handle(&mut self, msg: FetchPosts, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;
        let posts_lists: Vec<Post> = {
            use super::schema::posts::dsl::*;
            posts.filter(valid.eq(true)).load::<Post>(&conn)?
        };

        // let votes_lists: Vec<Vec<Vote>> = {
        //     use super::schema::votes::dsl::*;
        //     Vote::belonging_to(&posts_lists)
        //         .filter(user_id.eq(msg.user_id))
        //         .load::<Vote>(&conn)?
        //         .grouped_by(&posts_lists)
        // };
        {
            use super::schema::votes::dsl::*;
            Ok(posts_lists
                .into_iter()
                .map(|post| {
                    let res = votes
                        .filter(user_id.eq(msg.user_id))
                        .filter(post_id.eq(post.id))
                        .first::<Vote>(&conn)
                        .optional();
                    (post, res.unwrap_or(None))
                }).collect::<Vec<_>>())
        }
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

pub struct UpdateKarma;

impl Message for UpdateKarma {
    type Result = Result<((Vec<Post>, Vec<User>)), Error>;
}

impl Handler<UpdateKarma> for DbExecutor {
    type Result = Result<((Vec<Post>, Vec<User>)), Error>;

    fn handle(&mut self, _msg: UpdateKarma, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;

        let invalid_posts: Vec<Post> = {
            use super::schema::posts::dsl::*;
            diesel::update(
                posts
                    .filter(created_at.lt(now - 61_i32.minutes()))
                    .filter(valid.eq(true))
            ).set(valid.eq(false))
            .get_results::<Post>(&conn)?
        };

        let mut users_to_update: Vec<User> = Vec::new();

        let grouped_votes: Vec<Vec<Vote>> = Vote::belonging_to(&invalid_posts)
            .load::<Vote>(&conn)?
            .grouped_by(&invalid_posts);
        for mut group in grouped_votes {
            let (up_v, down_v): (Vec<Vote>, Vec<Vote>) =
                group.into_iter().partition(|v| v.up_or_down == 1);

            let groups: Option<(Vec<Vote>, Vec<Vote>)> = match up_v.len().cmp(&down_v.len()) {
                Ordering::Greater => Some((up_v, down_v)),
                Ordering::Less => Some((down_v, up_v)),
                Ordering::Equal => None,
            };

            if let Some((winners, losers)) = groups {
                use super::schema::users::dsl::*;

                let losers = losers.into_iter().map(|l| l.user_id).collect::<Vec<_>>();

                let mut losers = diesel::update(users.filter(id.eq(any(&losers))))
                    .set((streak.eq(0), karma.eq(karma - 10)))
                    .get_results::<User>(&conn)?;

                users_to_update.append(&mut losers);

                let winners = winners.into_iter().map(|w| w.user_id).collect::<Vec<_>>();

                let mut winners = diesel::update(users.filter(id.eq(any(&winners))))
                    .set((streak.eq(streak + 1), karma.eq(karma + 10)))
                    .get_results::<User>(&conn)?;

                users_to_update.append(&mut winners);
            }
        }

        users_to_update.sort_unstable_by_key(|u| u.id);
        users_to_update.dedup_by_key(|u| u.id);
        Ok((invalid_posts, users_to_update))
    }
}
