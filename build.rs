use std::{
    env,
    path::PathBuf,
    process::{Command, Stdio},
};

fn main() {
    let outdir = PathBuf::from(".out").canonicalize().expect("cannot canonicalize path");

    std::fs::create_dir(&outdir).ok();

    println!("cargo:rustc-link-search={}", outdir.to_str().unwrap());

    println!("cargo:rustc-link-lib=vm");
    println!("cargo:rustc-link-lib=vm_zig");

    println!("cargo:rerun-if-changed=c/");
    println!("cargo:rerun-if-changed=zig/");

    if !Command::new("clang")
        .arg("-O3")
        .arg("-flto=full")
        // .arg("-ffat-lto-objects")
        .arg("-static")
        .arg("-DDO_RESTRICT")
        .arg("-g")
        .arg("-c")
        .arg("-o")
        .arg(outdir.join("vm.o"))
        .arg("./c/vm.c")
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("could not spawn `clang`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not compile object file");
    }

    // Run `ar` to generate the `libvm.a` file from the `vm.o` file.
    // Unwrap if it is not possible to spawn the process.
    if !Command::new("ar")
        .arg("crus")
        .arg(outdir.join("libvm.a"))
        .arg(outdir.join("vm.o"))
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("could not spawn `ar`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not emit library file");
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("c/vm.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings.write_to_file(out_path).expect("Couldn't write bindings!");

    if !Command::new("zig")
        .arg("build-lib")
        .arg("./zig/src/vm_zig.zig")
        .arg("-fsingle-threaded")
        .arg("-static")
        .arg("-fPIC")
        .arg("-Ofast")
        .arg(format!("-femit-bin={}", outdir.join("libvm_zig.a").to_str().unwrap()))
        .arg("-flto")
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("could not spawn `zig`")
        .status
        .success()
    {
        panic!("could not call `zig`");
    }

    //
    if !Command::new("ls")
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .unwrap()
        .status
        .success()
    {
        panic!("???");
    }
}
