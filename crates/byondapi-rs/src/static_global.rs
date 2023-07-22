#[cfg(target_os = "windows")]
pub fn init_lib() -> byondapi_sys::ByondApi {
    // Load files from disk if in testing mode
    if std::env::var("BYONDAPI_TEST").is_ok() || cfg!(test) {
        let library = unsafe {
            let result = libloading::os::windows::Library::new("byondcore.dll");

            match result {
                Ok(lib) => lib,
                Err(e) => {
                    let message = format!("Unable to find byondcore.dll: {:#?}", e);
                    crate::error::crash_logging::log_to_file(&message);
                    panic!("{}", message)
                }
            }
        };
        byondapi_sys::init_from_library(library)
    } else {
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
        byondapi_sys::init_from_library(library)
    }
}

#[cfg(target_os = "linux")]
pub fn init_lib() -> byondapi_sys::ByondApi {
    // Load files from disk if in testing mode
    if std::env::var("BYONDAPI_TEST").is_ok() || cfg!(test) {
        let library = unsafe {
            // Load libext.so into the global namespace first
            let libext = {
                let result = libloading::os::unix::Library::open(
                    Some("./libext.so"),
                    libloading::os::unix::RTLD_NOW | libloading::os::unix::RTLD_GLOBAL,
                );

                match result {
                    Ok(lib) => lib,
                    Err(e) => {
                        let message = format!("Unable to find libext.so: {:#?}", e);
                        crate::error::crash_logging::log_to_file(&message);
                        panic!("{}", message)
                    }
                }
            };

            // Leak it intentionally.
            std::mem::forget(libext);

            // Then load libbyond.so
            let result = libloading::os::unix::Library::open(
                Some("./libbyond.so"),
                libloading::os::unix::RTLD_NOW | libloading::os::unix::RTLD_GLOBAL,
            );

            match result {
                Ok(lib) => lib,
                Err(e) => {
                    let message = format!("Unable to find libbyond.so: {:#?}", e);
                    crate::error::crash_logging::log_to_file(&message);
                    panic!("{}", message)
                }
            }
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
