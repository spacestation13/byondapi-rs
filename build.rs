use std::path::{Path, PathBuf};

fn main() {
    bindgen();
    if std::env::var("DOCS_RS").is_err() {
        link();
    }
}

fn get_header() -> PathBuf {
    #[cfg(feature = "515-1609")]
    return Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("headers")
        .join("515-1609")
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

    let builder = bindgen::Builder::default()
        .header(wrapper.to_string_lossy())
        // Also make headers included by main header dependencies of the build
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

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

fn get_download_link() -> &'static str {
    #[cfg(feature = "515-1609")]
    #[cfg(target_os = "windows")]
    return "https://files.catbox.moe/o69jxs.zip"; // TODO: Update to byond.com

    #[cfg(feature = "515-1609")]
    #[cfg(target_os = "linux")]
    return "http://www.byond.com/download/build/515/515.1609_byond_linux.zip";
}

fn link() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not defined"));
    let base_path = download_url(get_download_link(), &out_dir);

    #[cfg(target_os = "windows")]
    {
        println!(
            "cargo:rustc-link-search={}",
            find_lib(&base_path).to_string_lossy()
        );
        println!("cargo:rustc-link-lib=byondapi");
    }

    #[cfg(target_os = "linux")]
    {
        let bin_dir = find_bin(&base_path);
        println!("cargo:rustc-link-search={}", bin_dir.to_string_lossy());
        println!("cargo:rustc-link-lib=byond");
    }
}

fn download_url(url: &str, path: &Path) -> PathBuf {
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

    extracted
}

#[cfg(target_os = "windows")]
fn find_lib(base_path: &Path) -> PathBuf {
    let api_file = walkdir::WalkDir::new(base_path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|f| f.file_name().to_string_lossy() == "byondapi.lib")
        .expect("Cannot find byondapi.lib");

    let lib_dir = api_file.path().parent().unwrap().to_owned();

    lib_dir
}

#[cfg(target_os = "linux")]
fn find_bin(base_path: &Path) -> PathBuf {
    walkdir::WalkDir::new(base_path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|f| f.file_name().to_string_lossy() == "bin")
        .expect("Cannot find bin")
        .into_path()
}
