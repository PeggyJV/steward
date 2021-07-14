//! Time independent bollinger ranges
use crate::error::Error;
/// This is a Rust type for the JSON data from time independent bollinger ranges.
use abscissa_core::error::BoxError;
use ethers::prelude::*;
use futures::TryStreamExt;

use crate::{collector, config, prelude::*};
use chrono::DateTime;
use iqhttp::{HttpsClient, Result};
use mongodb::{
    bson::{bson, doc},
    options::ClientOptions,
    options::FindOptions,
    Client,
};
use serde::{Deserialize, Serialize};
use tower::{util::ServiceExt, Service};

// Struct TimeRange for time independent bollinger ranges
#[derive(Serialize, Deserialize)]
pub struct TimeRange {
    pub time: DateTime<chrono::Utc>,
    pub previous_update: DateTime<chrono::Utc>,
    pub pair_id: U256,
    pub tick_weights: Vec<TickWeight>,
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
pub struct TickWeight {
    pub upper_bound: i32,
    pub lower_bound: i32,
    pub weight: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MongoData {
    pub _id: mongodb::bson::Bson,
    pub created_timestap: mongodb::bson::Bson,
    pub pair_id: ethers::prelude::U256,
    pub symbol: String,
    pub tick_weights: Vec<MongoTickWeights>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MongoTickWeights {
    lower: mongodb::bson::Bson,
    upper: mongodb::bson::Bson,
    weight: mongodb::bson::Bson,
}

// Implement TimeRange for time independent bollinger ranges
impl TimeRange {
    // Instantiate TimeRange for time independent bollinger ranges with fn new.
    pub fn new(
        time: DateTime<chrono::Utc>,
        previous_update: DateTime<chrono::Utc>,
        pair_id: U256,
        tick_weights: TickWeight,
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
        let client = Client::with_uri_str("mongodb://10.32.0.224:27017/")
            .await
            .unwrap();

        let db = client.database("predictions");

        // Get a handle to a collection in the database.
        let collection = db.collection::<MongoData>(
            "tick_range_predictions
        ",
        );

        let find_options = FindOptions::builder()
            .sort(doc! { "created_timestamp": -1 })
            .build();

        let mut sorted_predictions = collection.find(None, find_options).await.unwrap();

        if let Some(_latest_prediction) = sorted_predictions.try_next().await.unwrap() {
            todo!()
        }
    }
}
