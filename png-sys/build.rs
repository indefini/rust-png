#![feature(collections, io, os, path)]

use std::process::Command;
use std::path::Path;
use std::env;
use std::os;
use std::io::ErrorKind;

fn main() {
    let target = os::getenv("TARGET").unwrap();
    let is_android = target.find_str("android").is_some();

    if is_android {
        let cc = format!("{}-gcc", target);
        let ar = format!("{}-ar", target);
        os::setenv("CC", cc);
        os::setenv("AR", ar);
    }

    let cmd = env::var("CARGO_MANIFEST_DIR").ok().expect("cargo dir");
    let cfg = Path::new(&cmd).join("libpng-1.6.16/configure");

    let od = env::var("OUT_DIR").ok().expect("out dir");
    let dst = Path::new(&od);

    os::setenv("CFLAGS", "-fPIC -O3");

    let mut cmd = Command::new(cfg);
    cmd.arg("--with-libpng-prefix=RUST_");
    if is_android {
        cmd.arg("--host=arm-linux-gnueabi");
    }
    cmd.current_dir(&dst);
    run(&mut cmd);

    let mut cmd = Command::new("make");
    cmd.arg("-j4");
    cmd.current_dir(&dst);
    run(&mut cmd);

    println!("cargo:root={}", dst.display());
    println!("cargo:rustc-flags=-l png16:static -L {}/.libs", dst.display());
}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            fail(&format!("failed to execute command: {}", e));
        }
        Err(e) => fail(&format!("failed to execute command: {}", e)),
    };
    if !status.success() {
        fail(&format!("command did not execute successfully, got: {}", status));
    }
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}
