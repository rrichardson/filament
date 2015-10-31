use std::env;
use std::path::{PathBuf};
use std::process::{Command, Stdio};

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    let res = cmd.stdout(Stdio::inherit())
                 .stderr(Stdio::inherit())
                 .status()
                 .unwrap()
                 .success();
    assert!(res);
}

fn main() {
    let root = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());

    let mut nghttp_root = root.clone();
    nghttp_root.push("nghttp2");

    let mut out_dir = nghttp_root.clone();
    for d in vec!["lib", ".libs"] {
        out_dir.push(d);
    }

    let mut libfile = out_dir.clone();
    libfile.push("libnghttp2.a");

    if !libfile.exists() {
        env::set_current_dir(&nghttp_root).unwrap();

        run(&mut Command::new("autoreconf").arg("-i"));
        run(&mut Command::new("automake"));
        run(&mut Command::new("autoconf"));
        run(&mut Command::new("./configure"));
        run(&mut Command::new("make").arg("-j4"));

        env::set_current_dir(&root).unwrap();
    }

    println!("cargo:rustc-link-search=native={}", out_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=nghttp2");
}
