extern crate capnpc;

fn main() {
    ::capnpc::CompilerCommand::new()
        .file("schema/rust.capnp")
        .file("schema/preference.capnp")
        .run()
        .expect("compiling schema");
}
