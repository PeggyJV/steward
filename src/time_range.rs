/// This is a Rust type for the JSON data from time independent bollinger ranges.
use ethers::{
    prelude::*};

/// Struct TimeRange for time independent bollinger ranges
pub struct TimeRange<T> {
    time: T,// i don't know what data types to use for time
    previous_update: T, // i don't know what data types to use for time
    pair_id: U256,
    tick_weights: TickWeights,
}

/// Struct TickWeights for time independent bollinger ranges
pub struct TickWeights {
    upper_bound: i32,
    lower_bound: i32,
    weight: f32,
}

// Implement TimeRange for time independent bollinger ranges
impl<T> TimeRange<T>{
    // Instantiate TimeRange for time independent bollinger ranges with fn new.
    pub fn new(time: T, previous_update: T, pair_id: U256, tick_weights: TickWeights) -> Self {
        TimeRange{time, previous_update, pair_id, tick_weights}  
    }
}