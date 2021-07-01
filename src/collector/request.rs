use ethers::prelude::*;
use crate::time_range::TimeRange;

pub enum Request {
    /// Handle an incoming message from an agent.
    Gas(GasPollEvent),

    /// Get the network state for a given network.
    TickData(TickDataPollEvent),

    /// Report information obtained from an external poller.
    ContractState(ContractStatePollEvent),
}

pub struct GasPollEvent {
    pub current_gas_price:U256,
}
pub struct TickDataPollEvent {
    pub current_tick_data:TimeRange,
}
pub struct ContractStatePollEvent {}
