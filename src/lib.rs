extern crate capnp;

pub mod protocol_capnp {
    #![allow(dead_code)]
    #![allow(missing_docs)]
    #![allow(unknown_lints)]
    #![allow(clippy)]
    include!(concat!(env!("OUT_DIR"), "/src/protocol_capnp.rs"));
}
