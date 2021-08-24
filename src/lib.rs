//! CellarRebalancer
//!
//! Application based on the [Abscissa] framework.
//!
//! [Abscissa]: https://github.com/iqlusioninc/abscissa

// Tip: Deny warnings with `RUSTFLAGS="-D warnings"` environment variable in CI

#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, trivial_casts)]

pub mod application;
pub mod cellar_wrapper;
pub mod collector;
pub mod commands;
pub mod config;
pub mod cosmos_somm;
pub mod erc20;
pub mod error;
pub mod gas;
pub mod position_manager;
pub mod prelude;
pub mod time_range;
pub mod uniswap_pool;
pub mod uniswap_router;
