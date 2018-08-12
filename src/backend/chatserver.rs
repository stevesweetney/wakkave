//! ChatServer is an actor. It maintains a list of connected client sessions.
//! Also manages available rooms. Peers can send messages to other peers
//! the same room through ChatServer.

use actix::prelude::*;
use uuid::Uuid;

// Chat Server sends messags of this type to sessions
#[derive(Message)]
pub struct ServerMessage(pub Vec<u8>);

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

#[derive(Default)]
pub struct ChatServer {
    session_ids: Vec<String>,
    session_addrs: Vec<Recipient<ServerMessage>> 
}

impl ChatServer {
    fn send_message(&self, data: Vec<u8>) {
        for addr in &self.session_addrs {
            let _ = addr.do_send(ServerMessage(data.clone()));
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for ChatServer {
    type Context = Context<Self>;
}

// Handler for Server Messages
impl Handler<ServerMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, _: &mut Context<Self>) -> Self::Result {
        self.send_message(msg.0);
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
            self.session_addrs.remove(i);
            self.session_ids.remove(i);
        }

        assert!(self.session_addrs.len() == self.session_ids.len());
    }
}