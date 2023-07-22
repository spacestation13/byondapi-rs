#[cfg(target_os = "windows")]
pub fn init_lib() -> byondapi_sys::ByondApi {
    // Load files from disk if in testing mode
    if std::env::var("BYONDAPI_TEST").is_ok() || cfg!(test) {
        let library = unsafe {
            libloading::os::windows::Library::new("byondcore.dll")
                .expect("unable to find byondcore.dll")
        };
        byondapi_sys::init_from_library(library)
    } else {
        let library = libloading::os::windows::Library::open_already_loaded("byondcore.dll")
            .expect("byondcore.dll is not already loaded");
        byondapi_sys::init_from_library(library)
    }
}

#[cfg(target_os = "linux")]
pub fn init_lib() -> byondapi_sys::ByondApi {
    // Load files from disk if in testing mode
    if std::env::var("BYONDAPI_TEST").is_ok() || cfg!(test) {
        let library = unsafe {
            // Load libext.so first
            let libext = libloading::os::unix::Library::open(
                Some("./libext.so"),
                libloading::os::unix::RTLD_NOW | libloading::os::unix::RTLD_GLOBAL,
            )
            .expect("Failed to load libext.so");

            std::mem::forget(libext);

            // Then load libbyond.so
            libloading::os::unix::Library::open(
                Some("./libbyond.so"),
                libloading::os::unix::RTLD_NOW | libloading::os::unix::RTLD_GLOBAL,
            )
            .expect("Failed to load libbyond.so")
        };
        byondapi_sys::init_from_library(library)
    } else {
        let library = libloading::os::unix::Library::this();
        byondapi_sys::init_from_library(library)
    }
}

lazy_static! {
    pub static ref BYOND: byondapi_sys::ByondApi = init_lib();
}
