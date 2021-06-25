/// This is a Rust type for the JSON data from time independent bollinger ranges.
use ethers::{
    prelude::*};
use abscissa_core::error::{BoxError};

use crate::error::Error;

use crate::{collector, config, prelude::*};
use tower::{util::ServiceExt, Service};
use chrono::DateTime;


/// Struct TimeRange for time independent bollinger ranges
pub struct TimeRange {
    time: DateTime<chrono::Utc>,// i don't know what data types to use for time
    previous_update: DateTime<chrono::Utc>, // i don't know what data types to use for time
    pair_id: U256,
    tick_weights: Vec<TickWeights>,
    data_url:String,
}

impl std::fmt::Debug for TimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fields =f.debug_struct("TimeRange");
        fields
        .field("time", &self.time)
        .field("previous_update", &self.previous_update)
        .field("pair_id",&self.pair_id);
        for (i,tick) in self.tick_weights.iter().enumerate(){
            fields.field(&format!("tick_weight #:{}",i),tick);
        }
        fields.field("data_url",&self.data_url).finish()


    }
}


/// Struct TickWeights for time independent bollinger ranges
#[derive(Debug)]
pub struct TickWeights {
    upper_bound: i32,
    lower_bound: i32,
    weight: f32,
}

// Implement TimeRange for time independent bollinger ranges
impl TimeRange{
    // Instantiate TimeRange for time independent bollinger ranges with fn new.
    pub fn new(time: DateTime<chrono::Utc>, previous_update: DateTime<chrono::Utc>, pair_id: U256, tick_weights: TickWeights) -> Self {
        TimeRange{time, previous_update, pair_id,tick_weights: Vec::new(), data_url:"".to_owned() }  
    }

    pub async fn poll<S>(&self, mut collector: S)
    where
        S: Service<collector::Request, Response = collector::Response, Error = BoxError>
            + Send
            + Clone
            + 'static,
    {
        todo!()
    }
}