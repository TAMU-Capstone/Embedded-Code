use std::env;
use std::path::PathBuf;
use std::fs::canonicalize;

fn nucleo_h (include_path: &str) {
    bindgen::Builder::default()
        .header("src/nucleo-144.h")
        .clang_arg(format!("-I{include_path}"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/nucleo-144.rs")
        .expect("Couldn't write bindings!");
}

fn board_h (include_path: &str) {
    bindgen::Builder::default()
        .header("include/board.h")
        .clang_arg(format!("-I{include_path}"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/board.rs")
        .expect("Couldn't write bindings!");
}

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rerun-if-changed=src/nucleo-144.h");

    
    let headers_dir = PathBuf::from("../../../../include/");
    let headers_dir_canonical = canonicalize(headers_dir).unwrap();
    let include_path = headers_dir_canonical.to_str().unwrap();

    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    nucleo_h(include_path);
    board_h(include_path);
    
}