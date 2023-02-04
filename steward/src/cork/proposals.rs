use std::time::Duration;

use abscissa_core::{
    tracing::{debug, error, info, log::warn},
    Application,
};
use gravity_bridge::gravity_proto::{
    cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::GetLatestBlockRequest,
    gravity::DelegateKeysByOrchestratorRequest,
};
use somm_proto::{
    cork::ScheduledCorkProposal, cosmos_sdk_proto::cosmos::gov::v1beta1::QueryProposalRequest,
};
use steward_proto::steward::{governance_call::Call, GovernanceCall};
use tokio::task::JoinHandle;
use tonic::{transport::Channel, Code};

use crate::{
    cellars::{aave_v2_stablecoin, cellar_v1, cellar_v2},
    config::DELEGATE_ADDRESS,
    cork::schedule_cork,
    error::{Error, ErrorKind},
    prelude::APP,
};

use super::{client::CorkQueryClient, id_hash};

type GovQueryClient = gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::gov::v1beta1::query_client::QueryClient<Channel>;
type GravityQueryClient =
    gravity_bridge::gravity_proto::gravity::query_client::QueryClient<Channel>;
type TendermintQueryClient = gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::service_client::ServiceClient<Channel>;

const RETRY_SLEEP: u64 = 5;

pub async fn start_scheduled_cork_proposal_polling_thread() -> JoinHandle<()> {
    debug!("starting cork proposal polling thread");
    let config = APP.config();
    let mut state = ProposalThreadState::default();
    let query_period = Duration::from_secs(config.cork.proposal_poll_period);

    // get validator address that corresponds to delegate for cork scheduling confirmation
    let mut client = GravityQueryClient::connect(config.cosmos.grpc.clone())
        .await
        .expect("failed to connect gravity query client at startup");
    let request = DelegateKeysByOrchestratorRequest {
        orchestrator_address: DELEGATE_ADDRESS.to_string(),
    };
    state.validator_address = client
        .delegate_keys_by_orchestrator(request)
        .await
        .expect("failed to get validator address at startup.")
        .into_inner()
        .validator_address;

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(query_period).await;

            debug!("polling for new approved scheduled cork proposals");
            state
                .update_last_observed_height(config.cosmos.grpc.clone())
                .await;
            if let Err(err) = poll_approved_cork_proposals(&mut state).await {
                error!(
                    "failed to process proposal {}: {}",
                    state.last_processed_proposal_id + 1,
                    err
                );
            }
        }
    })
}

#[derive(Debug, Default)]
struct ProposalThreadState {
    last_observed_height: u64,
    last_processed_proposal_id: u64,
    validator_address: String,
}

impl ProposalThreadState {
    async fn update_last_observed_height(&mut self, endpoint: String) {
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

    fn increment_proposal_id(&mut self) {
        self.last_processed_proposal_id += 1;
    }
}

async fn poll_approved_cork_proposals(state: &mut ProposalThreadState) -> Result<(), Error> {
    let config = APP.config();
    let mut gov_client = GovQueryClient::connect(config.cosmos.grpc.clone()).await?;

    loop {
        // Proposal IDs start at 1, so this should be ok even for the first query after startup.
        let proposal_id = state.last_processed_proposal_id + 1;
        debug!("querying proposal {}", proposal_id);
        let proposal = match gov_client
            .proposal(tonic::Request::new(QueryProposalRequest { proposal_id }))
            .await
        {
            Ok(r) => r.into_inner().proposal,
            Err(err) => {
                if err.code() == Code::NotFound {
                    // look ahead in case id was skipped
                    let mut missing_proposals = 1;
                    let mut found_proposal = false;

                    for i in 1..=10 {
                        if let Err(err) = gov_client
                            .proposal(tonic::Request::new(QueryProposalRequest {
                                proposal_id: proposal_id + i,
                            }))
                            .await
                        {
                            if err.code() == Code::NotFound {
                                missing_proposals += 1;
                                continue;
                            } else {
                                return Err(proposal_processing_error(format!(
                                    "error querying proposal {}: {}",
                                    proposal_id, err
                                )));
                            }
                        }

                        found_proposal = true;
                        break;
                    }

                    if found_proposal {
                        state.last_processed_proposal_id += missing_proposals;
                    } else {
                        info!(
                            "no new proposals. last processed proposal ID: {}",
                            state.last_processed_proposal_id
                        );
                    }

                    break;
                } else {
                    return Err(proposal_processing_error(format!(
                        "error querying proposal {}: {}",
                        proposal_id, err
                    )));
                }
            }
        };

        // Unsure if this can ever happen but needs to be handled.
        if proposal.is_none() {
            error!(
                "proposal {} was None even though the query status code indicates it was found.",
                proposal_id
            );
            state.increment_proposal_id();
            continue;
        }

        let proposal = proposal.unwrap();
        let content = match proposal.content {
            Some(c) => c,
            None => {
                warn!(
                    "ignoring proposal of ID {} because of empty content",
                    proposal.proposal_id
                );
                state.increment_proposal_id();
                continue;
            }
        };
        if content.type_url != "/cork.v2.ScheduledCorkProposal" {
            debug!("proposal {} not a ScheduledCorkProposal", proposal_id);
            state.increment_proposal_id();
            continue;
        }

        info!("processing scheduled cork proposal of ID {}", proposal_id);
        let cork_proposal: ScheduledCorkProposal =
            match prost::Message::decode(content.value.as_slice()) {
                Ok(c) => c,
                Err(err) => {
                    error!(
                        "failed to decode ScheduledCorkProposal {}: {}",
                        proposal_id, err
                    );
                    state.increment_proposal_id();
                    continue;
                }
            };
        if cork_proposal.block_height <= state.last_observed_height {
            info!(
                "proposal {} block height {} has already passed.",
                proposal_id, cork_proposal.block_height
            );
            state.increment_proposal_id();
            continue;
        }

        let json = cork_proposal.contract_call_proto_json;
        let cellar_id = cork_proposal.target_contract_address;
        let block_height = cork_proposal.block_height;
        debug!(
            "proposal {} contract call proto JSON: {}",
            proposal_id, json
        );
        let governance_call = match serde_json::from_str::<GovernanceCall>(&json) {
            Ok(c) => c,
            Err(err) => {
                error!("failed to decode GovernanceCall JSON {}: {}", json, err);
                state.increment_proposal_id();
                continue;
            }
        };
        if governance_call.call.is_none() {
            warn!(
                "governance call for proposal {} is empty and will be ignored: {:?}",
                proposal_id, governance_call
            );
            state.increment_proposal_id();
            continue;
        }
        let encoded_call: Vec<u8> = match governance_call.call.unwrap() {
            Call::AaveV2Stablecoin(data) => {
                if data.function.is_none() {
                    warn!(
                        "scheduled cork proposal {} call data contains no function data and will be ignored: {:?}",
                        proposal.proposal_id, data,
                    );
                    state.increment_proposal_id();
                    continue;
                }
                let function = data.function.unwrap();
                aave_v2_stablecoin::get_encoded_governance_call(
                    function,
                    &cellar_id,
                    proposal.proposal_id,
                )?
            }
            Call::CellarV1(data) => {
                if data.function.is_none() {
                    warn!(
                        "scheduled cork proposal {} call data contains no function data and will be ignored: {:?}",
                        proposal.proposal_id, data,
                    );
                    state.increment_proposal_id();
                    continue;
                }
                let function = data.function.unwrap();
                match cellar_v1::get_encoded_governance_call(
                    function,
                    &cellar_id,
                    proposal.proposal_id,
                ) {
                    Ok(d) => d,
                    // this is likely a bug in steward
                    Err(err) => {
                        error!(
                            "failed to get encoded governance call data for proposal {}: {}",
                            proposal.proposal_id, err
                        );
                        state.increment_proposal_id();
                        continue;
                    }
                }
            }
            Call::CellarV2(data) => {
                if data.function.is_none() {
                    warn!(
                        "scheduled cork proposal {} call data contains no function data and will be ignored: {:?}",
                        proposal.proposal_id, data,
                    );
                    state.increment_proposal_id();
                    continue;
                }
                let function = data.function.unwrap();
                match cellar_v2::get_encoded_governance_call(
                    function,
                    &cellar_id,
                    proposal.proposal_id,
                ) {
                    Ok(d) => d,
                    // this is likely a bug in steward
                    Err(err) => {
                        error!(
                            "failed to get encoded governance call data for proposal {}: {}",
                            proposal.proposal_id, err
                        );
                        state.increment_proposal_id();
                        continue;
                    }
                }
            }
        };

        debug!(
            "proposal {} encoded call: {:?}",
            proposal.proposal_id, encoded_call
        );

        // retry scheduling call on errors up to configured amount of retries
        let mut attempts = 0;
        loop {
            if let Err(schedule_err) =
                schedule_cork(&cellar_id, encoded_call.clone(), block_height).await
            {
                match confirm_sheduling(state, &cellar_id, encoded_call.clone(), block_height).await
                {
                    Ok(confirmed) => {
                        if confirmed {
                            info!(
                                "call for cellar {} scheduled for block {} by proposal {}",
                                cellar_id, block_height, proposal.proposal_id
                            );
                            state.increment_proposal_id();
                            break;
                        } else {
                            log_schedule_failure(
                                proposal.proposal_id,
                                attempts,
                                config.cork.max_scheduling_retries,
                                schedule_err,
                                None,
                            )
                            .await;

                            attempts += 1;
                            if attempts > config.cork.max_scheduling_retries {
                                state.increment_proposal_id();
                                break;
                            }

                            tokio::time::sleep(Duration::from_secs(RETRY_SLEEP)).await;
                        }
                    }
                    Err(confirm_err) => {
                        log_schedule_failure(
                            proposal.proposal_id,
                            attempts,
                            config.cork.max_scheduling_retries,
                            schedule_err,
                            Some(confirm_err),
                        )
                        .await;

                        attempts += 1;
                        if attempts > config.cork.max_scheduling_retries {
                            state.increment_proposal_id();
                            break;
                        }

                        tokio::time::sleep(Duration::from_secs(RETRY_SLEEP)).await;
                    }
                };
            } else {
                info!(
                    "call for cellar {} scheduled for block {} by proposal {}",
                    cellar_id, block_height, proposal.proposal_id
                );
                state.increment_proposal_id();
                break;
            }
        }
    }

    Ok(())
}

async fn confirm_sheduling(
    state: &ProposalThreadState,
    cellar_id: &str,
    encoded_call: Vec<u8>,
    block_height: u64,
) -> Result<bool, Error> {
    let id = id_hash(block_height, cellar_id, encoded_call);
    let mut client = CorkQueryClient::new().await?;

    Ok(client
        .get_scheduled_corks_by_id(&id)
        .await?
        .into_inner()
        .corks
        .iter()
        .any(|c| c.validator == state.validator_address))
}

fn proposal_processing_error(message: String) -> Error {
    ErrorKind::ProposalProcessingError.context(message).into()
}

async fn log_schedule_failure(
    proposal_id: u64,
    attempts: u64,
    max_retries: u64,
    schedule_err: Error,
    confirm_err: Option<Error>,
) {
    error!(
        "failed to schedule cork for proposal {}. attempt {}/{}. reason: {}",
        proposal_id, attempts, max_retries, schedule_err
    );
    if confirm_err.is_some() {
        warn!(
            "failed to confirm if cork was scheduled in spite of error: {}",
            confirm_err.unwrap()
        );
    }
}
