use std::path::PathBuf;
use std::fs::canonicalize;


fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rerun-if-changed=src/nucleo-144.h");
    
    let headers_dir1 = PathBuf::from("../../../../include/");
    let headers_dir_canonical1 = canonicalize(headers_dir1).unwrap();
    let include_path1 = headers_dir_canonical1.to_str().unwrap();

    let headers_dir2 = PathBuf::from("../../../../arch/arm/src/stm32f7/");
    let headers_dir_canonical2 = canonicalize(headers_dir2).unwrap();
    let include_path2 = headers_dir_canonical2.to_str().unwrap();

    let headers_dir3 = PathBuf::from("../../../../arch/arm/src/stm32f7/hardware/");
    let headers_dir_canonical3 = canonicalize(headers_dir3).unwrap();
    let include_path3 = headers_dir_canonical3.to_str().unwrap();
    
    bindgen::Builder::default()
        .header("include/wrapper.h")
        .clang_arg(format!("-I{include_path1}"))
        .clang_arg(format!("-I{include_path2}"))
        .clang_arg(format!("-I{include_path3}"))
        .use_core()                                 // use ::core:: instead of ::std::
        .ctypes_prefix("cty")                       // Use cty::* for the C types
        .layout_tests(false)                        // Don't generate #[test]'s because #![no_std]
        .fit_macro_constants(true)                  // Reduce the size of the constant to the smallest integer size, (e.g. u32 -> u8)
        .raw_line("#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/include/bindings.rs")
        .expect("Couldn't write bindings!");    
}