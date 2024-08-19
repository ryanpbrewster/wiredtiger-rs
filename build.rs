use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=vendor");

    Command::new("tar").arg("xf").arg("vendor/3.1.0.tar.gz")
      .output()
      .expect("untar vendored code");
}
