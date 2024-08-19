use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=vendor");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR env var");

    std::env::set_current_dir(out_dir).expect("chdir to OUT_DIR");

    let output = Command::new("./autogen.sh")
        .output()
        .expect("autogen wiredtiger build");
    if !output.status.success() {
        panic!("failed to autogen: {:?}", output);
    }

    let output = Command::new("./configure")
        .output()
        .expect("configure wiredtiger build");
    if !output.status.success() {
        panic!("failed to configure wiredtiger: {:?}", output);
    }

    let output = Command::new("make")
        .output()
        .expect("make wiredtiger");
    if !output.status.success() {
        panic!("failed to make wiredtiger: {:?}", output);
    }
}
