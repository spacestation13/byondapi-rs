use std::path::{Path, PathBuf};

#[cfg(feature = "515.1609")]
const DOWNLOAD_URL: &str = "https://files.catbox.moe/o69jxs.zip"; // TODO: Update to byond.com

fn main() {
    let tmp_dir = tempfile::Builder::new()
        .prefix("byondapi-sys")
        .tempdir_in(std::env::var("OUT_DIR").unwrap())
        .expect("Unable to create temporary directory");

    let (lib_dir, wrapper) = download_url(DOWNLOAD_URL, tmp_dir.path());

    println!("cargo:rustc-link-search={}", lib_dir.to_string_lossy());

    let bindings = bindgen::Builder::default()
        .header(wrapper.to_string_lossy())
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(Path::new(&std::env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=byondapi");
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
