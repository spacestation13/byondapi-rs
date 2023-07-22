# BYONDAPI-rs - The primary safe rust bindings for BYONDAPI

This crate implements a rusty safe API for using BYONDAPI.

# Testing

In order to successfully run cargo test, you must have the following files from the most recent BYOND version
in your library search path. For Windows, I recommend copying

For Windows, I recommend copying the following files to the crate root, `crates/byondapi-rs/`:
 - `byond/bin/byondcore.dll`
 - `byond/bin/byondext.dll`
 - `byond/bin/byondwin.dll`

For Linux, ensure these are in your `LD_LIBRARY_PATH`:
 - `byond/bin/libbyond.so`
 - `byond/bin/libext.so`

Failure to do this will result in an error when trying to run tests.