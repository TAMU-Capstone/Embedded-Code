use std::error::Error;

use std::path::PathBuf;

use std::io::{Write, stderr};
use std::process::Command;

use std::collections::HashMap;

use regex::Regex;
use topo_sort::TopoSort;

/**
Topologically Sorts the given header file based on each "#define" statement's dependancies
 */
fn toposort_macros(lines: String) -> Result<String, Box<dyn Error>> {
    let mut macros: HashMap<String, String> = HashMap::new();
    let mut deps = TopoSort::<String>::new();

    let macro_re = Regex::new(r"#\s*define\s*(\S*)\s*(.*)")?;
    let quote_re = Regex::new(r#"[\"\']+.*[\"\']"#)?;
    let parse_re = Regex::new(r"\w+")?;

    for line in lines.trim_end().split('\n') {
        let [name, value] = macro_re.captures(line).and_then(|c| c.extract().1.into()).unwrap();
        macros.insert(name.to_string(), value.to_string());
        deps.insert(
            name.to_string(),
            parse_re
                .find_iter(&quote_re.replace(value, "0"))
                .map(|m| m.as_str().to_string())
                .filter(|dep| dep.parse::<i64>().is_err()),
        );
    }

    macros
        .keys()
        .filter(|c| !(c.contains("(") || c.contains(")")))
        .for_each(|conf| println!("cargo:rustc-cfg={}", conf));

    let sorted: Vec<String> = deps
        .try_into_vec_nodes()?
        .iter()
        .map(|name| format!("#define {} {}\n", name, macros[name]))
        .collect();

    Ok(sorted.concat())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Constants that are used to determine `#ifdef` and `#if defined`
    let prepocessor_constants =
        PathBuf::from("../../../../include/nuttx/config.h").canonicalize()?;

    let include_paths = [
        "src/",
        "../../../../sched/",
        "../../../../include/",
        "../../../../arch/arm/src/common/",
        "../../../../arch/arm/src/stm32f7/",
        "../../../../arch/arm/src/stm32f7/hardware/",
    ]
    .map(|path| {
        format!(
            "-I{}",
            PathBuf::from(path)
                .canonicalize()
                .unwrap()
                .to_str()
                .unwrap()
        )
    });

    let preprocess = Command::new("clang")
        .arg("-E")
        .arg("-dM")
        .arg("-include")
        .arg(prepocessor_constants)
        .arg("include/wrapper.h")
        .arg("-o")
        .arg("-") // write to stdout so we can capture in a variable
        .args(&include_paths)
        .output()
        .expect("Failed to execute Clang command");

    stderr().write_all(&preprocess.stderr)?; // Write the stderr of the command to stderr
    assert!(preprocess.status.success()); // Panic if the command didnt succeed

    let macros = String::from_utf8(preprocess.stdout)?;
    let sorted_macros: String = toposort_macros(macros)?;

    std::fs::write("sorted_macros.h", sorted_macros)?;

    bindgen::Builder::default()
        .clang_arg("-H")                    // Print the names of header files during compilation
        .clang_args(include_paths)          // Search in these directories for the headers
        .header("include/wrapper.h")
        .header("sorted_macros.h")
        .use_core()                         // use ::core instead of ::std
        .ctypes_prefix("cty")               // Use cty::* for the C types
        .layout_tests(false)                // Don't generate #[test]'s because #![no_std]
        .fit_macro_constants(true)          // Reduce the size of the constant to the smallest integer size, (e.g. u32 -> u8)
        .raw_line(
            "#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]",
        )
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/bindings/generated.rs")
        .expect("Couldn't write bindings!");

    std::fs::remove_file("sorted_macros.h")?;

    Ok(())
}
