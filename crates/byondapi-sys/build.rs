use std::path::{Path, PathBuf};

fn main() {
    bindgen();
}

fn get_header() -> PathBuf {
    #[cfg(feature = "515-1609")]
    return Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("headers")
        .join("515-1609")
        .join("byondapi.h");
    #[cfg(feature = "515-1610")]
    return Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("headers")
        .join("515-1610")
        .join("byondapi.h");
}

fn copy_wrapper(lib_dir: &Path) -> PathBuf {
    let wrapper_path = lib_dir.join("wrapper.hpp");

    std::fs::copy(
        Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("src")
            .join("wrapper.hpp"),
        &wrapper_path,
    )
    .expect("Failed to copy wrapper.hpp to byondapi");

    wrapper_path
}

fn bindgen() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not defined"));

    let vendored_header = get_header();
    std::fs::copy(&vendored_header, out_dir.join("byondapi.h"))
        .expect("Failed to copy header to OUT_DIR");
    let wrapper = copy_wrapper(&out_dir);

    // Make byondapi-c interface header a dependency of the build
    println!(
        "cargo:rerun-if-changed={}",
        vendored_header.to_string_lossy()
    );

    let mut builder = bindgen::Builder::default()
        .header(wrapper.to_string_lossy())
        .dynamic_library_name("ByondApi")
        .dynamic_link_require_all(true)
        // Also make headers included by main header dependencies of the build
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    // Disable copy on refcounted types
    builder = builder.no_copy("CByondValue").no_copy("CByondValueList");

    // TODO: Enable C++ conversion when bindgen supports CUnwind correctly
    // let rust_version = rustc_version::version().unwrap();

    // if rust_version.major > 1 || (rust_version.major == 1 && rust_version.minor > 72) {
    // builder = builder
    //     // Tweaks
    //     .override_abi(bindgen::Abi::CUnwind, "Byond_GetVar")
    //         .override_abi(bindgen::Abi::CUnwind, "Byond_SetVar")
    //         .override_abi(bindgen::Abi::CUnwind, "Byond_GetVarByStrId")
    //         .override_abi(bindgen::Abi::CUnwind, "Byond_SetVarByStrId")
    //         .override_abi(bindgen::Abi::CUnwind, "Byond_CreateList")
    //         .override_abi(bindgen::Abi::CUnwind, "Byond_GetList")
    //         .override_abi(bindgen::Abi::CUnwind, "Byond_SetList")
    //         .override_abi(bindgen::Abi::CUnwind, "Byond_ReadPointer")
    //         .override_abi(bindgen::Abi::CUnwind, "Byond_WritePointer")
    //         .override_abi(bindgen::Abi::CUnwind, "Byond_CallProc")
    //         .override_abi(bindgen::Abi::CUnwind, "Byond_CallProcByStrId");
    // }

    builder
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
