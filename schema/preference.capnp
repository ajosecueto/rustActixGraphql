@0x9eb32e19f86ee179;

using Rust = import "rust.capnp";
$Rust.parentModule("events::schema");

struct Locale {
    locale @0 :Text;
    description @1 :Text;
    videoUrl @2 :Text;
}

struct Preference {
    locale @0 :List(Locale);
}
