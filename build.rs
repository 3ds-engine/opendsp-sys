use std::env;
use std::process::Command;
use std::path::PathBuf;

fn main() {
    let devkitpro = env::var("DEVKITPRO")
        .unwrap_or_else(|_| "/opt/devkitpro".to_string());

    // Compile OpenDSP using its Makefile
    let status = Command::new("make")
        .args(&["-C", "vendor/OpenDSP/library"])
        .status()
        .expect("Fallo al intentar ejecutar 'make' en vendor/OpenDSP/library");

    if !status.success() {
        panic!("OpenDSP makefile failed");
    }

    // Point the static library to cargo
    println!("cargo:rustc-link-search=native=vendor/OpenDSP/library/lib");
    println!("cargo:rustc-link-lib=static=opendsp");

    let libctru_include = format!("{}/libctru/include", devkitpro);
    let portlibs_include = format!("{}/portlibs/3ds/include", devkitpro);
    let devkitarm_include = format!("{}/devkitARM/arm-none-eabi/include", devkitpro);

    // Generate bindgen bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-Ivendor/OpenDSP/library/include")
        .clang_arg(format!("-I{}", portlibs_include))
        .clang_arg(format!("-I{}", libctru_include))
        .clang_arg(format!("-I{}", devkitarm_include))
        .clang_arg("--target=armv6k-none-eabi")
        // Filters to only bind openDSP's functions, not the entirety of libctru
        .allowlist_function("opendsp_.*")
        .allowlist_function("opendsp.*")
        .allowlist_type("opendsp_.*") 
        .allowlist_type("opendsp.*")      
        .allowlist_var("OPENDSP_.*")      
        .allowlist_var("OPENDSP.*")      
        .allowlist_type("OpenDSP.*")
        //--
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Could not generate bindings for OpenDSP");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings");
}