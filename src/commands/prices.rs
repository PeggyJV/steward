use crate::prelude::*;

use crate::application::APP;
use abscissa_core::time::naive::serde::ts_milliseconds::deserialize;
use ethers::prelude::*;
use std::ops::Deref;
use std::time::Duration;

use crate::config::ContractMonitorConfig;
use abscissa_core::{config, Command, FrameworkError, Options, Runnable};
use serde::de::{self, Deserializer, Error, Visitor};
use serde::Deserialize;
use std::str::FromStr;
use reqwest::Client;
use std::time::{SystemTime, UNIX_EPOCH};


use graphql_client::*;

#[derive(Debug)]
pub struct BigInt(num_bigint::BigInt);

impl<'de> Deserialize<'de> for BigInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BigIntVisitor;

        impl<'de> Visitor<'de> for BigIntVisitor {
            type Value = BigInt;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("A value that can turned into a big int")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match num_bigint::BigInt::from_str(v) {
                    Ok(x) => Ok(BigInt { 0: x }),
                    Err(e) => Err(E::custom(format!("Invalid big int {}", e))),
                }
            }
        }

        deserializer.deserialize_string(BigIntVisitor)
    }
}

// Enable `Deref` coercion.
impl Deref for BigInt {
    type Target = num_bigint::BigInt;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct BigDecimal(bigdecimal::BigDecimal);

impl<'de> Deserialize<'de> for BigDecimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BigDecimalVisitor;

        impl<'de> Visitor<'de> for BigDecimalVisitor {
            type Value = BigDecimal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("A value that can turned into a big int")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match bigdecimal::BigDecimal::from_str(v) {
                    Ok(x) => Ok(BigDecimal { 0: x }),
                    Err(e) => Err(E::custom(format!("Invalid big int {}", e))),
                }
            }
        }

        deserializer.deserialize_string(BigDecimalVisitor)
    }
}

// Enable `Deref` coercion.
impl Deref for BigDecimal {
    type Target = bigdecimal::BigDecimal;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql_models/schemas/uniswap-v3.graphql",
    query_path = "graphql_models/queries/get-pool-daily-data.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
struct GetPoolDailyData;

///
/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>
#[derive(Command, Debug, Options)]
pub struct PricesCmd {}

impl Runnable for PricesCmd {
    fn run(&self) {
        abscissa_tokio::run(&APP, async {
            let client = Client::new();

            let q = GetPoolDailyData::build_query(get_pool_daily_data::Variables {
                pool: "0x4e68ccd3e89f51c3074ca5072bbac773960dfa36".to_string(),
                start_date: 1622775126 - 5 * 86400,
                end_date: 1622775126,
                order_by: "date".to_string(),
                order_direction: "asc".to_string(),
            });

            let res =client.post("http://localhost:8000/subgraphs/name/sommelier/uniswap-v3-history")
            .json(&q)
            .send().await.unwrap();

            res.error_for_status_ref().unwrap();

            let response_body: Response<get_pool_daily_data::ResponseData> = res.json().await.unwrap();
            info!("{:?}", response_body);
        


        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
