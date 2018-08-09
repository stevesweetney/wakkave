extern crate capnp;
extern crate serde;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate failure;

#[cfg(feature = "frontend")]
#[macro_use]
extern crate yew;

#[cfg(feature = "frontend")]
#[macro_use]
extern crate stdweb;

#[cfg(feature="default")]
extern crate actix;

#[cfg(feature="default")]
extern crate actix_web;

#[cfg(feature="default")]
#[macro_use]
extern crate diesel;

#[cfg(feature="default")]
extern crate dotenv;

#[cfg(feature="default")]
extern crate r2d2;

#[cfg(feature="default")]
extern crate jsonwebtoken;

#[cfg(feature="default")]
extern crate uuid;

#[cfg(feature="default")]
extern crate time;

#[cfg(feature="default")]
extern crate futures;

#[cfg(feature="default")]
pub mod backend;

#[cfg(feature = "frontend")]
pub mod frontend;

pub mod protocol_capnp {
    #![allow(dead_code)]
    #![allow(missing_docs)]
    #![allow(unknown_lints)]
    #![allow(clippy)]
    include!(concat!(env!("OUT_DIR"), "/src/protocol_capnp.rs"));
}