use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=vendor");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR env var");

    let output = Command::new("tar")
        .arg("--extract")
        .arg("--file=vendor/3.1.0.tar.gz")
        .arg(format!("--directory={out_dir}"))
        .arg("--strip-components=2")
        .output()
        .expect("untar vendored code");
    if !output.status.success() {
        panic!("failed to untar: {:?}", output);
    }
}
