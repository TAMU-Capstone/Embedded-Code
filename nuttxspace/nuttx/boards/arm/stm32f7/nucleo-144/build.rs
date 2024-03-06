use std::path::PathBuf;
use std::fs::canonicalize;


fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rerun-if-changed=src/nucleo-144.h");
    
    let headers_dir = PathBuf::from("../../../../include/");
    let headers_dir_canonical = canonicalize(headers_dir).unwrap();
    let include_path = headers_dir_canonical.to_str().unwrap();

    bindgen::Builder::default()
        .header("include/wrapper.h")
        .clang_arg(format!("-I{include_path}"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/include/bindings.rs")
        .expect("Couldn't write bindings!");    
}