#![feature(os, path)]

extern crate gcc;

use std::default::Default;
use std::os;
use std::path::PathBuf;
use std::env;
use std::path::Path;

fn main() {
    let mut cfg: gcc::Config = gcc::Config::new();

    cfg.file("src/shim.c");

    let cmd = env::var("CARGO_MANIFEST_DIR").ok().expect("cargo dir");
    let src_dir = Path::new(&cmd).join("png-sys/libpng-1.6.16");

    cfg.include(&src_dir);

    let od = env::var("DEP_PNG_ROOT").ok().expect("png root");
    let dep_dir = Path::new(&od);
    cfg.include(&dep_dir);

    cfg.compile("libpngshim.a")
}
