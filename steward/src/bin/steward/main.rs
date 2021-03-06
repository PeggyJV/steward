//! Main entry point for ContractMonitor

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use steward::application::APP;

/// Boot ContractMonitor
fn main() {
    abscissa_core::boot(&APP);
}
