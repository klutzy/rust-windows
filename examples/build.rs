use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").ok().expect("can't find out_dir");

    Command::new("windres").args(&["src/hello.rc",  "-o"])
                       .arg(&format!("{}/hello.rc.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libhello_rc.a", "hello.rc.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=hello_rc");
}