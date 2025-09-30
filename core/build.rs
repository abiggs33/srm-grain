extern crate cbindgen;

use std::env;

fn main() {
    cbindgen::Builder::new()
        .with_crate(env::var("CARGO_MANIFEST_DIR").expect("unable to locate crate"))
        .with_language(cbindgen::Language::C)
        .with_no_includes()
        .with_sys_include("stddef.h")
        .generate()
        .expect("unable to generate C bindings")
        .write_to_file("bindings.h");
}
