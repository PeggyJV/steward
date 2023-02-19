//! Steward
//!
//! Application based on the [Abscissa] framework.
//!
//! [Abscissa]: https://github.com/iqlusioninc/abscissa

// Tip: Deny warnings with `RUSTFLAGS="-D warnings"` environment variable in CI

#![forbid(unsafe_code)]
#![warn(trivial_casts)]
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
pub mod somm_send;
pub mod utils;

// Generated ABI definitions. This has to be manually updated when new contracts are added.
pub mod abi {
    pub mod aave_v2_stablecoin {
        include!("gen/abi/aave_v2_stablecoin.rs");
    }

    pub mod cellar_v1 {
        include!("gen/abi/cellar_v1.rs");
    }

    pub mod cellar_v2 {
        include!("gen/abi/cellar_v2.rs");
    }

    pub mod adaptors {
        pub mod aave_a_token_adaptor {
            include!("gen/abi/aave_a_token.rs");
        }

        pub mod aave_debt_token_adaptor {
            include!("gen/abi/aave_debt_token.rs");
        }

        pub mod compound_c_token_adaptor {
            include!("gen/abi/compound_c_token.rs");
        }

        pub mod uniswap_v3_adaptor {
            include!("gen/abi/uniswap_v3.rs");
        }

        pub mod vesting_simple_adaptor {
            include!("gen/abi/vesting_simple.rs");
        }
    }
}
