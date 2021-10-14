use crate::prelude::*;
use abscissa_core::{Command, Clap, Runnable};
use mongodb::{bson::doc, options::FindOptions, Client};

use crate::time_range::MongoData;

use futures::TryStreamExt;

/// Display lastest prediction in the application
#[derive(Command, Debug, Clap)]
pub struct PredictionsCmd {
    /// To whom are we saying hello?
    recipient: Vec<String>,
}

impl Runnable for PredictionsCmd {
    fn run(&self) {
        abscissa_tokio::run(&APP, async {
            // Get a handle to the deployment.
            let client = Client::with_uri_str("mongodb://localhost:27017/?directconnection=true")
                .await
                .unwrap();

            let db = client.database("predictions");

            // Get a handle to a collection in the database.
            let collection = db.collection::<MongoData>("tick_range_predictions");

            let find_options = FindOptions::builder()
                .sort(doc! { "created_timestamp": -1 })
                .build();

            let mut sorted_predictions = collection.find(None, find_options).await.unwrap();

            if let Some(latest_prediction) = sorted_predictions.try_next().await.unwrap() {
                info!("lastet prediction:{:?}", latest_prediction)
            }
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}