//! Steward
//!
//! Application based on the [Abscissa] framework.
//!
//! [Abscissa]: https://github.com/iqlusioninc/abscissa

// Tip: Deny warnings with `RUSTFLAGS="-D warnings"` environment variable in CI

#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, trivial_casts)]
#![allow(clippy::uninlined_format_args)]

pub mod application;
pub mod cellars;
pub mod commands;
pub mod config;
pub mod cork;
pub mod error;
pub mod gas;
pub mod prelude;
pub mod server;
pub mod simulate;
pub mod somm_send;
pub mod tenderly;
pub mod utils;
