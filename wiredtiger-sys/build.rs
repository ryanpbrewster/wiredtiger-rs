use std::{path::PathBuf, process::Command};

fn main() {
    println!("cargo::rerun-if-changed=vendor");
    let subdir: &str = "wiredtiger-3.1.0";

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR env var"));

    eprintln!("out_dir={out_dir:?}");

    let output = Command::new("cp")
        .arg("-R")
        .arg(format!("vendor/{subdir}"))
        .arg(&out_dir)
        .output()
        .expect("autogen wiredtiger build");
    if !output.status.success() {
        panic!("failed to copy sources: {:?}", output);
    } else {
        eprintln!("cp: {output:?}");
    }

    std::env::set_current_dir(out_dir.join(subdir)).expect("chdir to OUT_DIR");

    eprintln!("starting configure...");
    let output = Command::new("./configure")
        .output()
        .expect("configure wiredtiger build");
    if !output.status.success() {
        panic!("failed to configure wiredtiger: {:?}", output);
    }
    eprintln!("done with configure");

    eprintln!("starting make...");
    let output = Command::new("make")
        .arg("-j4") // TODO(rpb): what is a reasonable value here?
        .output()
        .expect("make wiredtiger");
    if !output.status.success() {
        panic!("failed to make wiredtiger: {:?}", output);
    }
    eprintln!("done with make");

    println!(
        "cargo:rustc-link-search={build_dir}/.libs",
        build_dir = out_dir.join(subdir).display(),
    );
    println!("cargo:rustc-link-lib=static=wiredtiger");

    eprintln!("starting bindgen...");
    bindgen::Builder::default()
        .header("wiredtiger.h")
        .generate()
        .expect("generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("write bindings to file");
    eprintln!("done with bindgen");
}
