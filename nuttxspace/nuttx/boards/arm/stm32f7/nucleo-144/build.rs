use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rerun-if-changed=src/nucleo-144.h");

    let headers_dir = PathBuf::from("../nuttx");
    let headers_dir_canonical = canonicalize(headers_dir).unwrap();
    let include_path = headers_dir_canonical.to_str().unwrap();



    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .header("src/nucleo-144.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("nucleo-144.h.rs"))
        .expect("Couldn't write bindings!");
}