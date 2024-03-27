use std::process::Command;
use std::path::PathBuf;


fn main() {
    // Constants that are used to determine `#ifdef` and `#if defined`
    let prepocessor_constants = PathBuf::from("../../../../include/nuttx/config.h")
        .canonicalize()
        .unwrap();
        
    let include_paths = [
        "src/",
        "../../../../sched/",
        "../../../../include/",
        "../../../../arch/arm/src/common/",
        "../../../../arch/arm/src/stm32f7/",
        "../../../../arch/arm/src/stm32f7/hardware/",
    ].map(|path| format!("-I{}", PathBuf::from(path)
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap())
    );


    let wrapper = "include/wrapper.h";
    let header = "include/preprocessed_wrapper.h";


    Command::new("clang")
        .arg("-E")
        .arg("-dM")
        .arg("-include")
        .arg(prepocessor_constants)
        .arg(wrapper)
        .arg("-o")
        .arg(header)
        .args(&include_paths)
        .output()
        .expect("Failed to execute Clang command");


    bindgen::Builder::default()
        .clang_arg("-H")
        .clang_args(include_paths)
        .header(wrapper)
        .header(header)
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