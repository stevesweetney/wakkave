extern crate capnp;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate failure;

#[cfg(feature = "frontend")]
#[macro_use]
extern crate yew;

#[cfg(feature="default")]
extern crate actix;

#[cfg(feature="default")]
extern crate actix_web;

#[cfg(feature="default")]
extern crate jsonwebtoken;

#[cfg(feature="default")]
extern crate uuid;

#[cfg(feature="default")]
extern crate time;

#[cfg(feature="default")]
extern crate serde;

#[cfg(feature="default")]
#[macro_use]
extern crate serde_derive;

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