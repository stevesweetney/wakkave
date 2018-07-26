extern crate capnpc;

use ::std::path::PathBuf;

const CAPNP_FILE: &str = "protocol.capnp";

fn main() {
    ::capnpc::CompilerCommand::new()
        .file(PathBuf::from("src").join(CAPNP_FILE))
        .run()
        .expect("compiling schema");
}