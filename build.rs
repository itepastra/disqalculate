extern crate pkg_config;

fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/disqalc.cc")
        .compile("disqalc");

    let lcalc = pkg_config::Config::new()
        .atleast_version("5.0.0")
        .probe("libqalculate")
        .unwrap();

    for lib in lcalc.link_files {
        println!(
            "cargo:rustc-link-lib={}",
            lib.to_str().expect("path should be a valid string")
        );
    }
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/disqalc.cc");
    println!("cargo:rerun-if-changed=include/disqalc.h");
}
