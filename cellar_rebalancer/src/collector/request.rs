use crate::time_range::TimeRange;
use ethers::prelude::*;

pub enum Request {
    /// Report information obtained from an external poller.
    ContractState(ContractStatePollEvent),
    /// Handle an incoming message from an agent.
    Gas(GasPollEvent),
    /// Request a rebalance event from the collector.
    ReblanceRequest,
    /// Get the recommendded or a given network.
    TickData(TickDataPollEvent),
}

pub struct GasPollEvent {
    pub current_gas_price: U256,
}
pub struct TickDataPollEvent {
    pub current_tick_data: TimeRange,
}
pub struct ContractStatePollEvent {}
