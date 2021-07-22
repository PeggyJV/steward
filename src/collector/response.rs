use crate::time_range::TimeRange;
use chrono::DateTime;

pub enum Response {
    AccummulateResponse,
    RebalanceNotNeeded,
    RebalanceResponse(RebalanceResponse),
}

pub struct RebalanceResponse {
    pub time_since_last_rebalance: DateTime<chrono::Utc>,
    pub good_gas_price: bool,
    pub rebalance_target: TimeRange,
}
