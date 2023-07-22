# BYONDAPI-rs - The primary safe rust bindings for BYONDAPI

This crate implements a rusty safe API for using BYONDAPI.

## Linux Warnings

Normally, when finding the BYOND functions the library needs fails, usually due to being loaded by a process that
does not have them already loaded into the global namespace, the crate will write to `byondapi-rs-log.txt` informing
you what went wrong.

Currently, this is unable to be implemented for Linux, so no log will be produced other than the panic!() leaking into
BYOND, which is undefined behavior.

## Testing

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