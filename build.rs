use std::{
    env,
    ffi::OsStr,
    path::PathBuf,
    process::{Command, Stdio},
};

const OUT_DIR: &str = ".out";

fn main() {
    std::fs::create_dir(OUT_DIR).ok();

    let outdir = PathBuf::from(OUT_DIR).canonicalize().expect("cannot canonicalize path");

    println!("cargo:rustc-link-search={}", outdir.to_str().unwrap());

    println!("cargo:rustc-link-lib=vm");
    println!("cargo:rustc-link-lib=vm_zig");

    println!("cargo:rerun-if-changed=c/");
    println!("cargo:rerun-if-changed=zig/");

    run(
        "clang",
        &[
            &"-O3",
            // &"-fsanitize=fuzzer",
            // &"-flto=full",
            &"-static",
            &"-fPIC",
            &"-fsanitize=address",
            &"-static-libasan",
            &"-Wl,-fsanitize=address",
            // &"-Wl,-Bsymbolic",
            &"-DDO_RESTRICT",
            &"-g",
            &"-c",
            &"-o",
            &outdir.join("vm.o"),
            &"./c/vm.c",
        ],
    );

    run("ar", &[&"crs", &outdir.join("libvm.a"), &outdir.join("vm.o")]);
    // run("llvm-ranlib", &[&outdir.join("libvm.a")]);

    let bindings = bindgen::Builder::default()
        .header("c/vm.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings.write_to_file(out_path).expect("couldn't write bindings");

    run(
        "zig",
        &[
            &"build-lib",
            &"./zig/src/vm_zig.zig",
            &"-fsingle-threaded",
            &"-static",
            &"-fPIC",
            &"-Ofast",
            &format!("-femit-bin={}", outdir.join("libvm_zig.a").to_str().unwrap()),
            // &"-flto",
        ],
    );
}

#[track_caller]
fn run(cmd: impl AsRef<OsStr>, args: &[&dyn AsRef<OsStr>]) {
    if !Command::new(cmd)
        .args(args)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("could not spawn command")
        .status
        .success()
    {
        panic!("error running command");
    }
}
