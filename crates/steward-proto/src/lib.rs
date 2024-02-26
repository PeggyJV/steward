//! Proto bindings for [Steward](https://github.com/peggyjv/steward), the sidecar application which hosts the Strategist API and
//! handles many other key processes for the Sommelier blockchain.
//!
//! The proto definitions can be found [here](https://github.com/PeggyJV/steward/tree/main/proto/steward/v4)

/// Generated proto definitions
pub mod proto {
    include!("gen/steward.v4.rs");
}
