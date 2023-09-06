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

    unsafe { byondapi_sys::ByondApi::init_from_library(library) }
        .expect("Failed to initialize library.")
}

#[cfg(target_os = "linux")]
pub fn init_lib() -> byondapi_sys::ByondApi {
    let library = libloading::os::unix::Library::this();
    match unsafe { byondapi_sys::ByondApi::init_from_library(library) } {
        Err(e) => {
            let message = format!(
                "byondcore.dll is not loaded into the process as expected: {:#?}",
                e
            );
            crate::error::crash_logging::log_to_file(&message);
            panic!("{}", message)
        }
        Ok(res) => res,
    }
}

#[inline(always)]
pub fn byond() -> &'static byondapi_sys::ByondApi {
    BYOND.get_or_init(init_lib)
}

static BYOND: std::sync::OnceLock<byondapi_sys::ByondApi> = std::sync::OnceLock::new();
