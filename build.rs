use std::{
    env,
    path::PathBuf,
    process::{Command, Stdio},
};

fn main() {
    let c_libdir_path = PathBuf::from("c").canonicalize().expect("cannot canonicalize path");

    let zig_libdir_path = PathBuf::from("zig").canonicalize().expect("cannot canonicalize path");

    // This is the path to the `c` headers file.
    let headers_path = c_libdir_path.join("vm.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // This is the path to the intermediate object file for our library.
    let obj_path = c_libdir_path.join("vm.o");
    // This is the path to the static library file.
    let lib_path = c_libdir_path.join("libvm.a");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", c_libdir_path.to_str().unwrap());
    // no way to change the output path of zig build-lib, the file outputs to the current working directory
    println!("cargo:rustc-link-search=.");

    // Tell cargo to tell rustc to link our `vm` library. Cargo will
    // automatically know it must look for a `libvm.a` file.
    println!("cargo:rustc-link-lib=vm");
    println!("cargo:rustc-link-lib=vm_zig");

    println!("cargo:rerun-if-changed=c/");

    // Run `clang` to compile the `vm.c` file into a `vm.o` object file.
    // Unwrap if it is not possible to spawn the process.

    if !Command::new("clang")
        .arg("-O3")
        .arg("-flto=full")
        // .arg("-ffat-lto-objects")
        .arg("-static")
        .arg("-DDO_RESTRICT")
        .arg("-g")
        .arg("-c")
        .arg("-o")
        .arg(&obj_path)
        .arg(c_libdir_path.join("vm.c"))
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
        .arg(lib_path)
        .arg(obj_path)
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
        .header(headers_path_str)
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
