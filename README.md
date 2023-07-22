# BYONDAPI-rs - BYONDAPI for Rust

This is a monorepo of all of the relevant crates forming the Rust wrapper for [BYONDApi](https://www.byond.com/docs/ref/#/{{appendix}}/Byondapi)

You'll find each subproject in the `crates/` directory.
 - [`crates/byondapi-rs`](crates/byondapi-rs) is the main "user facing" API with nice safe rust bindings.
 - [`crates/byondapi-sys`](crates/byondapi-sys) is the raw [bindgen](https://github.com/rust-lang/rust-bindgen) generated bindings for the BYONDAPI C header
 - [`crates/byondapi-rs-test`](crates/byondapi-rs-test) is an example/testing repo that uses the other two crates to ensure it actually works in BYOND.