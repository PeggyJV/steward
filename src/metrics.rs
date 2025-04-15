use axum::{response::IntoResponse, routing::get, Router};
use prometheus::{register_counter, Counter};
use std::net::SocketAddr;
use tokio::task::JoinHandle;

lazy_static::lazy_static! {
    // Proposal metrics
    pub static ref PROPOSALS_PROCESSED: Counter = register_counter!(
        "steward_proposals_processed_total",
        "Total number of proposals processed"
    ).unwrap();

    pub static ref PROPOSAL_PROCESSING_ERRORS: Counter = register_counter!(
        "steward_proposal_processing_errors_total",
        "Total number of proposal processing errors"
    ).unwrap();

    // Trust state metrics
    pub static ref TRUST_STATE_LOAD_SUCCESS: Counter = register_counter!(
        "steward_trust_state_load_success_total",
        "Total number of successful trust state loads"
    ).unwrap();

    pub static ref TRUST_STATE_LOAD_ERRORS: Counter = register_counter!(
        "steward_trust_state_load_errors_total",
        "Total number of trust state load errors"
    ).unwrap();

    // GRPC metrics
    pub static ref GRPC_CONNECTION_ERRORS: Counter = register_counter!(
        "steward_grpc_connection_errors_total",
        "Total number of GRPC connection errors"
    ).unwrap();

    pub static ref GRPC_REQUESTS: Counter = register_counter!(
        "steward_grpc_requests_total",
        "Total number of GRPC requests made"
    ).unwrap();

    // Retry metrics
    pub static ref SCHEDULING_RETRIES: Counter = register_counter!(
        "steward_scheduling_retries_total",
        "Total number of scheduling retries"
    ).unwrap();

    pub static ref SCHEDULING_FAILURES: Counter = register_counter!(
        "steward_scheduling_failures_total",
        "Total number of scheduling failures"
    ).unwrap();

    // Cache metrics
    pub static ref CACHE_REFRESH_SUCCESS: Counter = register_counter!(
        "steward_cache_refresh_success_total",
        "Total number of successful cache refreshes"
    ).unwrap();

    pub static ref CACHE_REFRESH_ERRORS: Counter = register_counter!(
        "steward_cache_refresh_errors_total",
        "Total number of cache refresh errors"
    ).unwrap();
}

async fn metrics_handler() -> impl IntoResponse {
    prometheus::TextEncoder::new()
        .encode_to_string(&prometheus::gather())
        .unwrap()
}

pub async fn start_metrics_server(addr: SocketAddr) -> JoinHandle<()> {
    let app = Router::new().route("/metrics", get(metrics_handler));

    tokio::spawn(async move {
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    })
}
