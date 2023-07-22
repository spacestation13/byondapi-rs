#[macro_use]
extern crate lazy_static;

mod static_global;

#[macro_use]
pub mod error;
pub use error::Error;

pub mod list;
pub mod prelude;
pub mod typecheck_trait;
pub mod value;
