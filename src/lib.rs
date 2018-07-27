extern crate capnp;

#[cfg(feature = "frontend")]
#[macro_use]
extern crate yew;

#[cfg(feature="default")]
extern crate actix;

#[cfg(feature="default")]
extern crate actix_web;

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