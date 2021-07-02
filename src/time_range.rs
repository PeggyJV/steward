//! Time independent bollinger ranges
use crate::error::Error;
/// This is a Rust type for the JSON data from time independent bollinger ranges.
use abscissa_core::error::BoxError;
use ethers::prelude::*;

use crate::{collector, config, prelude::*};
use chrono::DateTime;
use iqhttp::{HttpsClient, Result};
use serde::{Deserialize, Serialize};
use tower::{util::ServiceExt, Service};

// Struct TimeRange for time independent bollinger ranges
#[derive(Serialize, Deserialize)]
pub struct TimeRange {
    time: DateTime<chrono::Utc>,
    previous_update: DateTime<chrono::Utc>,
    pair_id: U256,
    tick_weights: Vec<TickWeights>,
}

/// Implement TimeRange for time independent bollinger ranges
impl TimeRange {
    // Fetch timerange from JSON file "tickdata"
    pub async fn fetch(host: impl Into<String>) -> Result<TimeRange> {
        let client = HttpsClient::new(host);
        client.get_json("/tickdata", &Default::default()).await
    }
}

impl Default for TimeRange {
    fn default() -> Self {
        TimeRange {
            time: chrono::Utc::now(),
            previous_update: chrono::Utc::now(),
            pair_id: U256::zero(),
            tick_weights: Vec::new(),
        }
    }
}

/// Implementation for TimeRange field format
impl std::fmt::Debug for TimeRange {
    // Implement TimeRange field format for time, previous_update, pair_id and tick_weight
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fields = f.debug_struct("TimeRange");
        fields
            .field("time", &self.time)
            .field("previous_update", &self.previous_update)
            .field("pair_id", &self.pair_id);
        for (i, tick) in self.tick_weights.iter().enumerate() {
            fields.field(&format!("tick_weight #:{}", i), tick);
        }
        fields.finish()
    }
}

/// Struct TickWeights for time independent bollinger ranges
#[derive(Serialize, Deserialize, Debug)]
pub struct TickWeights {
    upper_bound: i32,
    lower_bound: i32,
    weight: f32,
}

// Implement TimeRange for time independent bollinger ranges
impl TimeRange {
    // Instantiate TimeRange for time independent bollinger ranges with fn new.
    pub fn new(
        time: DateTime<chrono::Utc>,
        previous_update: DateTime<chrono::Utc>,
        pair_id: U256,
        tick_weights: TickWeights,
    ) -> Self {
        TimeRange {
            time,
            previous_update,
            pair_id,
            tick_weights: Vec::new(),
        }
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
