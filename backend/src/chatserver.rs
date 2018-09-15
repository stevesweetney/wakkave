//! ChatServer is an actor. It maintains a list of connected client sessions.
//! Also manages available rooms. Peers can send messages to other peers
//! the same room through ChatServer.

use super::database::{
    executor::{DbExecutor, UpdateKarma},
    models::{Post, User},
};
use actix::{fut, prelude::*};
use capnp::{
    self,
    message::Builder,
    serialize_packed,
};
use uuid::Uuid;

use protocol_capnp::{response, update, Vote as P_Vote};

// Chat Server sends message of this type to sessions
#[derive(Message)]
pub struct ServerMessage(pub Vec<u8>, pub Option<String>);

// Message from client that will be broadcasted
#[derive(Message)]
pub struct ClientMessage {
    /// Id of client session
    pub id: String,
    /// Peer message
    pub msg: Post,
}

// New chat session is created
#[derive(Message)]
#[rtype(String)]
pub struct Connect {
    pub addr: Recipient<ServerMessage>,
}

/// Session is disconnected
#[derive(Message)]
pub struct Disconnect {
    pub id: String,
}

pub struct ChatServer {
    session_ids: Vec<String>,
    session_addrs: Vec<Recipient<ServerMessage>>,
    db: Addr<DbExecutor>,
}

impl ChatServer {
    pub fn new(addr: Addr<DbExecutor>) -> Self {
        ChatServer {
            session_ids: Vec::new(),
            session_addrs: Vec::new(),
            db: addr,
        }
    }

    fn send_message(&self, data: Vec<u8>, skip: Option<String>) {
        for addr in &self.session_addrs {
            println!("Server message sent to session!");
            let _ = addr.do_send(ServerMessage(data.clone(), skip.clone()));
        }
    }

    fn send_updates(&self, (invalid, users): (Vec<Post>, Vec<User>)) {
        let mut b = Builder::new_default();
        let mut data = Vec::new();
        {
            let update = b.init_root::<response::Builder>().init_update();

            let mut invalid_posts = update.init_invalid(invalid.len() as u32);

            for (i, post) in invalid.iter().enumerate() {
                invalid_posts.set(i as u32, post.id);
            }
        }

        if let Ok(()) = serialize_packed::write_message(&mut data, &b) {
            self.send_message(data.clone(), None);
        }

        data.clear();

        {
            let update = b.init_root::<response::Builder>().init_update();

            let mut users_to_update = update.init_users(users.len() as u32);

            for (i, usr) in users.iter().enumerate() {
                let mut u = users_to_update.reborrow().get(i as u32);
                u.set_id(usr.id);
                u.set_username(&usr.username);
                u.set_karma(usr.karma);
                u.set_streak(usr.streak);
            }
        }

        if let Ok(()) = serialize_packed::write_message(&mut data, &b) {
            self.send_message(data, None);
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for ChatServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        use std::time::Duration;

        ctx.run_interval(Duration::from_secs(600), |act, ctx| {
            let query_task = act
                .db
                .send(UpdateKarma)
                .into_actor(act)
                .timeout(Duration::from_secs(480), MailboxError::Timeout)
                .then(|res, actor, _contx| match res {
                    Ok(res) => {
                        if let Ok(res) = res {
                            actor.send_updates(res);
                        }
                        fut::ok(())
                    }
                    _ => fut::err(()),
                });
            ctx.spawn(query_task);
        });
    }
}

// Handler for Server Messages
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) -> Self::Result {
        let mut b = Builder::new_default();
        let mut data = Vec::new();
        {
            let update = b.init_root::<response::Builder>().init_update();
            let p = msg.msg;

            let mut post = update.init_new_post();
            post.set_id(p.id);
            post.set_content(&p.content);
            post.set_valid(p.valid);
            post.set_user_id(p.user_id);
            post.set_vote(P_Vote::None);
        }

        let _ = serialize_packed::write_message(&mut data, &b);

        self.send_message(data, Some(msg.id));
    }
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer {
    type Result = String;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        // register session with random id
        // check to see if this session addr already exists
        let idx = self.session_addrs.iter().position(|x| *x == msg.addr);
        if let Some(idx) = idx {
            // send existing id back
            self.session_ids[idx].clone()
        } else {
            let id = Uuid::new_v4().to_string();
            self.session_addrs.push(msg.addr);
            self.session_ids.push(id.clone());
            assert!(self.session_addrs.len() == self.session_ids.len());
            // send new id back
            id
        }
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        let idx = self.session_ids.iter().position(|x| *x == msg.id);

        if let Some(i) = idx {
            // Order does not matter
            // Can swap the id and addr of the disconnecting
            // session with the last element and pop the last
            // element from the vectors
            self.session_addrs.swap_remove(i);
            self.session_ids.swap_remove(i);
        }

        assert!(self.session_addrs.len() == self.session_ids.len());
    }
}
