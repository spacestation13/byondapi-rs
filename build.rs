use std::path::{Path, PathBuf};

#[cfg(feature = "515.1609")]
const DOWNLOAD_URL: &str = "https://files.catbox.moe/o69jxs.zip"; // TODO: Update to byond.com

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not defined"));

    let (lib_dir, wrapper) = download_url(DOWNLOAD_URL, &out_dir);

    // Make byondapi-c interface header a dependency of the build
    println!("cargo:rerun-if-changed={}", wrapper.to_string_lossy());
    println!("cargo:rustc-link-search={}", lib_dir.to_string_lossy());
    println!("cargo:rustc-link-lib=byondapi");

    bindgen::Builder::default()
        .header(wrapper.to_string_lossy())
        // Also make headers included by main header dependencies of the build
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn download_url(url: &str, path: &Path) -> (PathBuf, PathBuf) {
    let response =
        reqwest::blocking::get(url).unwrap_or_else(|_| panic!("Unable to fetch {}", url));

    let mut content = std::io::Cursor::new(
        response
            .bytes()
            .unwrap_or_else(|_| panic!("Unable to get bytes of {}", url)),
    );

    let mut zip = zip::ZipArchive::new(&mut content).expect("Invalid zip archive");

    let extracted = path.join("byond");

    zip.extract(&extracted).expect("Failed to unzip archive");

    let api_file = walkdir::WalkDir::new(extracted)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|f| f.file_name().to_string_lossy() == "byondapi.h")
        .expect("Cannot find byondapi.h");

    let lib_dir = api_file.path().parent().unwrap().to_owned();

    std::fs::copy(
        Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("src")
            .join("wrapper.hpp"),
        lib_dir.join("wrapper.hpp"),
    )
    .expect("Failed to copy wrapper.hpp to byondapi");

    let wrapper = lib_dir.join("wrapper.hpp").to_owned();

    (lib_dir, wrapper)
}
