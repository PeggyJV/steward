use std::time::Duration;

use abscissa_core::{
    tracing::{debug, error, info, log::warn},
    Application,
};
use futures::executor::block_on;
use gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::base::{
    query::v1beta1::PageRequest,
    tendermint::v1beta1::GetLatestBlockRequest,
};
use somm_proto::{cork::ScheduledCorkProposal, cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse};
use steward_proto::steward::{GovernanceCall, governance_call::Call};
use tokio::task::JoinHandle;

use crate::{
    cellars::{aave_v2_stablecoin, cellar_v1},
    error::Error,
    prelude::APP,
    config::StewardConfig, cork::schedule_cork,
};

type GovQueryClient = gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::gov::v1beta1::query_client::QueryClient<tonic::transport::channel::Channel>;
type TendermintQueryClient = gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::service_client::ServiceClient<tonic::transport::channel::Channel>;

pub async fn start_scheduled_cork_proposal_polling_thread() -> JoinHandle<()> {
    debug!("starting cork proposal polling thread");
    let config = APP.config();
    let mut state = ProposalThreadState::default();

    // let query_period = Duration::from_secs(config.cork.proposal_poll_period);
    // Collin: This is only for testing
    let query_period = Duration::from_secs(10);

    tokio::spawn(async move {
        let mut fail_count = 0;
        loop {
            tokio::time::sleep(query_period).await;

            debug!("polling for new approved scheduled cork proposals");
            state.update_latest_observed_height(config.cosmos.grpc.clone()).await;
            if let Err(err) = poll_approved_cork_proposals(&mut state).await {
                fail_count += 1;
                error!(
                    "failed to poll approved scheduled cork proposals {} time(s): {}",
                    fail_count, err
                );
            } else {
                fail_count = 0;
            }
        }
    })
}

#[derive(Debug, Default)]
struct ProposalThreadState {
    proposal_query_offset: u64,
    last_observed_height: u64,
    retry_data: Vec<(u64, String, Vec<u8>, u64, u8)>,
}

impl ProposalThreadState {
    async fn update_latest_observed_height(&mut self, endpoint: String) {
        match TendermintQueryClient::connect(endpoint).await {
            Ok(mut client) => {
                match client.get_latest_block(GetLatestBlockRequest {}).await {
                    Ok(r) => {
                        let r = r.into_inner();
                        if r.block.is_some() {
                            let block = r.block.unwrap();
                            if block.header.is_some() {
                                self.last_observed_height = block.header.unwrap().height as u64;
                            }
                        }
                    }
                    Err(err) => warn!("failed to query latest block height. this could eventually result in degraded scheduled cork proposal query performance: {}", err),
                };
            }
            Err(err) => warn!("failed to query latest block height. this could eventually result in degraded scheduled cork proposal query performance: {}", err),
        };
    }
}

async fn poll_approved_cork_proposals(state: &mut ProposalThreadState) -> Result<(), Error> {
    let config = APP.config();
    let mut gov_client = GovQueryClient::connect(config.cosmos.grpc.clone()).await?;
    let mut pagination = PageRequest {
        limit: 20,
        offset: state.proposal_query_offset,
        ..Default::default()
    };
    let mut processed_total: u64 = 0;

    loop {
        debug!("querying proposals");
        let proposals = gov_client
            .proposals(tonic::Request::new(gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::gov::v1beta1::QueryProposalsRequest {
                proposal_status: gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::gov::v1beta1::ProposalStatus::Passed.into(),
                pagination: Some(pagination.clone()),
                ..Default::default()
            }))
            .await?
            .into_inner()
            .proposals;

        if proposals.is_empty() {
            debug!("no proposals found.");
            state.proposal_query_offset = pagination.offset;
            break;
        }
        processed_total += proposals.len() as u64;
        debug!("found {} proposals", processed_total);
        for proposal in proposals {
            let content = match proposal.content {
                Some(c) => c,
                None => {
                    debug!(
                        "ignoring proposal of ID {} because of empty content",
                        proposal.proposal_id
                    );
                    continue;
                }
            };
            if content.type_url == "/cork.v2.ScheduledCorkProposal" {
                let cork_proposal: ScheduledCorkProposal =
                    match prost::Message::decode(content.value.as_slice()) {
                        Ok(c) => c,
                        Err(err) => {
                            error!(
                                "failed to decode ScheduledCorkProposal {}: {}",
                                proposal.proposal_id, err
                            );
                            continue;
                        }
                    };
                if cork_proposal.block_height <= state.last_observed_height {
                    continue;
                }

                let json = cork_proposal.contract_call_proto_json;
                let cellar_id = cork_proposal.target_contract_address;
                let block_height = cork_proposal.block_height;
                let encoded_call: Vec<u8>;
                debug!("JSON: {}", json);
                let governance_call = match serde_json::from_str::<GovernanceCall>(&json) {
                    Ok(c) => c,
                    Err(err) => {
                        error!("failed to decode GovernanceCall JSON {}: {}", json, err);
                        continue;
                    }
                };
                if governance_call.call.is_none() {
                    error!("governance call is empty: {:?}", governance_call);
                    continue;
                }
                match governance_call.call.unwrap() {
                    Call::AaveV2Stablecoin(data) => {
                        if data.function.is_none() {
                            error!(
                                // if this happens there is a problem with proposal validation in sommelier
                                "cork proposal {} call data contains no function data: {:?}",
                                proposal.proposal_id, data,
                            );
                            continue;
                        }
                        let function = data.function.unwrap();
                        encoded_call = aave_v2_stablecoin::get_encoded_governance_call(
                            function,
                            &cellar_id,
                            proposal.proposal_id,
                        )?;
                    },
                    Call::CellarV1(data) => {
                        if data.function.is_none() {
                            error!(
                                "cork proposal {} call data contains no function data: {:?}",
                                proposal.proposal_id, data
                            );
                            continue;
                        }
                        let function = data.function.unwrap();
                        encoded_call = cellar_v1::get_encoded_governance_call(
                            function,
                            &cellar_id,
                            proposal.proposal_id,
                        )?;
                    }
                }
                debug!("proposal {} encoded call: {:?}", proposal.proposal_id, encoded_call);

                if let Err(err) = schedule_cork(&cellar_id, encoded_call.clone(), block_height).await {
                    error!(
                        "failed to schedule cork for proposal {}, will retry shortly. reason: {}",
                        proposal.proposal_id, err
                    );
                    // don't add an item twice to retry data
                    for d in state.retry_data.iter() {
                        if d.0 == proposal.proposal_id {
                            continue;
                        }
                    }
                    state.retry_data.push((proposal.proposal_id, cellar_id.clone(), encoded_call, block_height, 0));
                }
                info!(
                    "submitted cork for cellar {} triggered by proposal {}",
                    cellar_id, proposal.proposal_id
                );
            }
        }
        pagination.offset += processed_total;
    }

    retry_failed_schedulings(
        state,
        &config,
        |cellar_id: String, encoded_call: Vec<u8>, block_height: u64| {
            // async closures with arguments are unstable, so we have to block
            block_on(schedule_cork(&cellar_id, encoded_call, block_height))
        }).await;

    Ok(())
}

// Manages retrying any failed calls in the thread state using the passed in function. Passing in the scheduling
// function as an argument is a little gross but makes testing much simpler since we don't have to worry about
// rewriting things to allow real mocking or initializing abscissa's application state.
async fn retry_failed_schedulings(state: &mut ProposalThreadState, config: &StewardConfig, schedule_function: fn(String, Vec<u8>, u64) -> Result<TxResponse, Error>) {
    if state.retry_data.len() == 0 {
        return ()
    }

    // Retry the calls that failed during scheduling in case of transient network failure
    info!("retrying {} failed schedulings", state.retry_data.len());

    // Sort in ascending order by proposal ID
    state.retry_data.sort();
    let mut remove_indeces = Vec::<usize>::new();
    for (i, mut d) in state.retry_data.iter_mut().enumerate() {
        if let Err(err) = schedule_function(d.1.clone(), d.2.clone(), d.3) {
            // increment retry count
            d.4 += 1;

            error!(
                "failed to schedule cork for proposal {}. retried {} time(s). reason: {}",
                d.0, d.4, err
            );

            if d.4 as u64 >= config.cork.max_scheduling_retries {
                warn!("proposal {} failed to schedule, will not be retried again.", d.0);
                remove_indeces.push(i);
            }
        } else {
            info!(
                "cork scheduled for cellar {}, triggered by proposal {}, will execute at block {}",
                d.1, d.0, d.3
            );
            remove_indeces.push(i);
        }
    }

    // Remove scheduled corks that have reached the retry threshold or were scheduled successfully.
    // We sort the indeces in descending order to avoid changing the indeces of other items during removal.
    remove_indeces.sort_by(|a, b| b.cmp(a));
    for i in remove_indeces {
        state.retry_data.remove(i);
    }
}

#[cfg(test)]
mod tests {
    use assay::assert_eq;

    use crate::error::ErrorKind;

    use super::*;

    // Tests state management of retry-able cork data.
    // Collin: If we want to, there is probably a way to initialize abscissa's application state and do
    // this in an integration test, which would allow us to avoid passing a scheduling function as an arg.
    // I spent some time figuring that out and this just seemed faster.
    #[test]
    fn test_retry_failed_schedulings() {
        use crate::config::CorkConfig;

        let mut state = ProposalThreadState::default();
        state.retry_data.push((3, "0x3333333333333333333333333333333333333333".to_string(), Vec::<u8>::default(), 3000, 0));
        state.retry_data.push((1, "0x1111111111111111111111111111111111111111".to_string(), Vec::<u8>::default(), 1000, 1));
        state.retry_data.push((2, "0x2222222222222222222222222222222222222222".to_string(), Vec::<u8>::default(), 2000, 0));

        let test_config = StewardConfig {
            cork: CorkConfig {
                max_scheduling_retries: 2,
                ..Default::default()
            },
            ..Default::default()
        };

        // all fail
        let erroring_function = |_: String, _: Vec<u8>, _: u64| -> Result<TxResponse, crate::error::Error> {
            Err(ErrorKind::GovernanceCall.context("error").into())
        };
        block_on(retry_failed_schedulings(&mut state, &test_config, erroring_function));

        // prop 1 removed after second retry
        assert_eq!(state.retry_data.len(), 2);

        // prop 2 is first in vec, and incremented
        assert_eq!(state.retry_data[0].0, 2);
        assert_eq!(state.retry_data[0].4, 1);

        // prop 3 is second and incremented
        assert_eq!(state.retry_data[1].0, 3);
        assert_eq!(state.retry_data[1].4, 1);

        // remaining complete and are removed from state
        let passing_function = |_: String, _: Vec<u8>, _: u64| -> Result<TxResponse, crate::error::Error> {
            Ok(TxResponse::default())
        };
        block_on(retry_failed_schedulings(&mut state, &test_config, passing_function));

        assert_eq!(state.retry_data.len(), 0);
    }
}
