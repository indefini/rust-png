extern crate gcc;

use std::default::Default;
use std::os;
use std::ffi::AsOsStr;

fn main() {
    let mut cfg = gcc::Config::new();

    let mut src_dir = os::getenv("CARGO_MANIFEST_DIR").unwrap();
    src_dir.push_str("png-sys/libpng-1.6.16");
    cfg.include(src_dir.as_os_str());

    let dep_dir = os::getenv("DEP_PNG_ROOT").unwrap();
    cfg.include(dep_dir.as_os_str());

    gcc::compile_library("libpngshim.a",
                         &["src/shim.c"]);
}
