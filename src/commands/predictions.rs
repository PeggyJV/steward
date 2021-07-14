use crate::prelude::*;
use abscissa_core::{config, Command, FrameworkError, Options, Runnable};
use mongodb::{
    bson::{bson, doc},
    options::ClientOptions,
    options::FindOptions,
    Client,
};

use crate::time_range::{MongoData, MongoTickWeights};

use futures::TryStreamExt;

#[derive(Command, Debug, Options)]
pub struct PredictionsCmd {
    /// To whom are we saying hello?
    #[options(free)]
    recipient: Vec<String>,
}

impl Runnable for PredictionsCmd {
    fn run(&self) {
        let config = APP.config();

        abscissa_tokio::run(&APP, async {
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
