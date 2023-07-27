#[cfg(target_os = "windows")]
pub fn init_lib() -> byondapi_sys::ByondApi {
    let library = {
        let result = libloading::os::windows::Library::open_already_loaded("byondcore.dll");

        match result {
            Ok(lib) => lib,
            Err(e) => {
                let message = format!(
                    "byondcore.dll is not loaded into the process as expected: {:#?}",
                    e
                );
                crate::error::crash_logging::log_to_file(&message);
                panic!("{}", message)
            }
        }
    };

    unsafe { byondapi_sys::init_from_library(library) }.expect("Failed to initialize library.")
}

#[cfg(target_os = "linux")]
pub fn init_lib() -> byondapi_sys::ByondApi {
    let library = libloading::os::unix::Library::this();
    unsafe { byondapi_sys::init_from_library(library) }.unwrap_or(|| {
        let message = format!(
            "byondcore.dll is not loaded into the process as expected: {:#?}",
            e
        );
        crate::error::crash_logging::log_to_file(&message);
        panic!("{}", message)
    })
}

lazy_static! {
    pub static ref BYOND: byondapi_sys::ByondApi = init_lib();
}
