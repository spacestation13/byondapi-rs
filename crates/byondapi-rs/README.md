# BYONDAPI-rs - Safe rust bindings for BYONDAPI

This crate implements a rusty safe API for using BYONDAPI.

# WARNING

This library automatically initializes on the first function call, using lazy_static!. This initialization can fail
in one circumstance: where the symbols needed by the BYONDAPI are not found in the current executable.

The only sane way to handle this is to panic, which will inevitably unwind across the FFI barrier, which is **undefined
behavior**.

There's two ways to fix this and I've chosen the second:
1. Make the library API substantially worse by forcing every function to take an argument to a library struct.
2. Wait for bindgen to [stabilize the C-unwind abi](https://github.com/rust-lang/rust-bindgen/issues/2581)

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