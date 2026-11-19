use bindgen::callbacks::ParseCallbacks;
use bindgen::Abi;
use std::path::{Path, PathBuf};

fn main() {
    generate_all();
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

fn generate_all() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not defined"));
    let header_path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("headers");
    let header_path = header_path.join("byondapi.h");

    let target = out_dir.join("byondapi.h");
    std::fs::copy(header_path, target).expect("Failed to copy to out_dir");
    let wrapper = copy_wrapper(&out_dir);

    let builder = bindgen::Builder::default()
        .header(wrapper.to_string_lossy())
        .dynamic_library_name("ByondApi")
        .dynamic_link_require_all(true)
        .override_abi(Abi::CUnwind, "Byond.*")
        // Clang mangling can fuck up symbol lookup on windows, we are also on c so it dont matter
        .trust_clang_mangling(false)
        // Also make headers included by main header dependencies of the build
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .parse_callbacks(Box::new(DoxygenCallbacks));

    builder
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[derive(Debug)]
struct DoxygenCallbacks;

impl ParseCallbacks for DoxygenCallbacks {
    fn process_comment(&self, comment: &str) -> Option<String> {
        Some(doxygen_rs::transform(comment))
    }
}
