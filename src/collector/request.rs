
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Request {
        /// Handle an incoming message from an agent.
        Gas(GasPollEvent),
    
        /// Get the network state for a given network.
        TickData(TickDataPollEvent),
    
        /// Report information obtained from an external poller.
        ContractState(ContractStatePollEvent),
    }



#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GasPollEvent {

}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TickDataPollEvent {

}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContractStatePollEvent {

}