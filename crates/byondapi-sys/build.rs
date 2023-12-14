use bindgen::callbacks::ParseCallbacks;
use std::path::{Path, PathBuf};

fn main() {
    generate_all();
}

fn get_version(x: &str) -> (u32, u32) {
    let vec: Vec<_> = x.split('-').take(2).collect();
    (vec[0].parse().unwrap(), vec[1].parse().unwrap())
}

fn get_headers() -> Vec<(PathBuf, (u32, u32))> {
    let base_path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("headers");

    base_path
        .read_dir()
        .expect("headers folder fucked up")
        .filter_map(|f| {
            if let Ok(file) = f {
                Some(file.file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .map(|f| (base_path.join(&f).join("byondapi.h"), get_version(&f)))
        .collect()
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

    get_headers()
        .into_iter()
        .for_each(|(path, (major, minor))| {
            let target = out_dir.join("byondapi.h");
            std::fs::copy(path, target).expect("Failed to copy to out_dir");
            let wrapper = copy_wrapper(&out_dir);

            let builder = bindgen::Builder::default()
                .header(wrapper.to_string_lossy())
                .dynamic_library_name("ByondApi")
                .dynamic_link_require_all(true)
                // Also make headers included by main header dependencies of the build
                .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
                .parse_callbacks(Box::new(DoxygenCallbacks));

            builder
                .generate()
                .expect("Unable to generate bindings")
                .write_to_file(out_dir.join(format!("bindings_{major}_{minor}.rs")))
                .expect("Couldn't write bindings!");
        });
}

#[derive(Debug)]
struct DoxygenCallbacks;

impl ParseCallbacks for DoxygenCallbacks {
    fn process_comment(&self, comment: &str) -> Option<String> {
        Some(doxygen_rs::transform(comment))
    }
}
