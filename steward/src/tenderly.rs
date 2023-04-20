use abscissa_core::tracing::log::error;
use abscissa_core::Application;
use reqwest::Response;
use serde::Deserialize;
use serde::Serialize;
use tonic::{Code, Status};

use crate::{config::StewardConfig, prelude::APP};

const TENDERLY_BASE_URL: &str = "https://api.tenderly.co/api/v1";

pub fn validate_tenderly_config(config: &StewardConfig) {
    if config.simulate.tenderly_access_key.is_empty() {
        panic!("Tenderly access key is not set");
    }
    if config.simulate.tenderly_username.is_empty() {
        panic!("Tenderly user is not set");
    }
    if config.simulate.tenderly_project_name.is_empty() {
        panic!("Tenderly project name is not set");
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimulateRequest {
    pub network_id: String,
    pub from: String,
    pub to: String,
    pub input: String,
    pub save: bool,
    pub save_if_fails: bool,
    pub simulation_type: String,
}

pub async fn simulate(cellar_id: String, encoded_call: String) -> Result<Response, Status> {
    let config = APP.config();
    let body = serde_json::to_string(&SimulateRequest {
        network_id: config.simulate.network_id.clone(),
        from: config.simulate.gravity_address.clone(),
        to: cellar_id,
        input: encoded_call,
        save: true,
        save_if_fails: true,
        simulation_type: "quick".to_string(),
    })
    .unwrap();
    let client = reqwest::Client::new();
    let user = &config.simulate.tenderly_username;
    let project = &config.simulate.tenderly_project_name;
    let url = format!("{TENDERLY_BASE_URL}/account/{user}/project/{project}/simulate");
    let request = match client
        .post(url)
        .header("X-Access-Key", &config.simulate.tenderly_access_key)
        .header("Content-Type", "application/JSON")
        .body(body)
        .build()
    {
        Ok(r) => r,
        Err(e) => {
            panic!("failed to build tenderly request: {}", e);
        }
    };

    client.execute(request).await.map_err(|e| {
        error!("simulate error: {}", e);
        Status::new(Code::Unknown, format!("simulate error: {e}"))
    })
}
