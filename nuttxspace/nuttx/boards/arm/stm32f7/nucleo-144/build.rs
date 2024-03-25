use std::path::PathBuf;


fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rerun-if-changed=src/nucleo-144.h");
    
    let paths = [
        "../../../../include/",
        "../../../../arch/arm/src/stm32f7/",
        "../../../../arch/arm/src/stm32f7/hardware/",
    ].map(|dir| PathBuf::from(dir).canonicalize().unwrap());

    let include_args = paths.map(|path| format!("-I{}", path.to_str().unwrap()));

    
    bindgen::Builder::default()
        .header("include/wrapper.h")
        .clang_args(include_args)
        .use_core()                                 // use ::core instead of ::std
        .ctypes_prefix("cty")                       // Use cty::* for the C types
        .layout_tests(false)                        // Don't generate #[test]'s because #![no_std]
        .fit_macro_constants(true)                  // Reduce the size of the constant to the smallest integer size, (e.g. u32 -> u8)
        .raw_line("#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/bindings/generated.rs")
        .expect("Couldn't write bindings!")  
}