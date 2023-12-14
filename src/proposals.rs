use std::time::Duration;

use abscissa_core::{
    tracing::{debug, error, info, log::warn},
    Application,
};
use gravity_bridge::gravity_proto::{
    cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::GetLatestBlockRequest,
    gravity::DelegateKeysByOrchestratorRequest,
};
use somm_proto::cosmos_sdk_proto::cosmos::gov::v1beta1::QueryProposalRequest;
use tokio::{sync::mpsc::Sender, task::JoinHandle};
use tonic::{transport::Channel, Code};

use crate::{
    config::get_delegate_address,
    cork::{client::CorkQueryClient, id_hash, proposals::handle_scheduled_cork_proposal},
    error::{Error, ErrorKind},
    prelude::APP,
    pubsub::cache::refresh_publisher_trust_state_cache,
};

type GovQueryClient = gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::gov::v1beta1::query_client::QueryClient<Channel>;
type GravityQueryClient =
    gravity_bridge::gravity_proto::gravity::query_client::QueryClient<Channel>;
type TendermintQueryClient = gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::service_client::ServiceClient<Channel>;

// Proposal types
const ADD_PUBLISHER_PROPOSAL: &str = "/pubsub.v1.AddPublisherProposal";
const REMOVE_PUBLISHER_PROPOSAL: &str = "/pubsub.v1.RemovePublisherProposal";
const ADD_DEFAULT_SUBSCRIPTION_PROPOSAL: &str = "/pubsub.v1.AddDefaultSubscriptionProposal";
const REMOVE_DEFAULT_SUBSCRIPTION_PROPOSAL: &str = "/pubsub.v1.RemoveDefaultSubscriptionProposal";
const SCHEDULED_CORK_PROPOSAL: &str = "/cork.v2.ScheduledCorkProposal";

pub async fn start_approved_proposal_polling_thread(
    publisher_cache_tx: Sender<()>,
) -> JoinHandle<()> {
    debug!("starting cork proposal polling thread");
    let config = APP.config();
    let mut state = ProposalThreadState::default();
    let query_period = Duration::from_secs(config.cork.proposal_poll_period);

    // get validator address that corresponds to delegate for cork scheduling confirmation
    let mut client = GravityQueryClient::connect(config.cosmos.grpc.clone())
        .await
        .expect("failed to connect gravity query client at startup");
    let request = DelegateKeysByOrchestratorRequest {
        orchestrator_address: get_delegate_address().to_string(),
    };
    state.validator_address = client
        .delegate_keys_by_orchestrator(request)
        .await
        .expect("failed to get validator address at startup.")
        .into_inner()
        .validator_address;

    tokio::spawn(async move {
        loop {
            debug!("polling for new approved scheduled cork proposals");
            state
                .update_last_observed_height(config.cosmos.grpc.clone())
                .await;
            if let Err(err) = poll_approved_proposals(&mut state, publisher_cache_tx.clone()).await
            {
                error!(
                    "failed to process proposal {}: {}",
                    state.last_processed_proposal_id + 1,
                    err
                );
            }

            tokio::time::sleep(query_period).await;
        }
    })
}

#[derive(Debug, Default)]
pub struct ProposalThreadState {
    pub last_observed_height: u64,
    pub last_processed_proposal_id: u64,
    pub validator_address: String,
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

async fn poll_approved_proposals(
    state: &mut ProposalThreadState,
    publisher_cache_tx: Sender<()>,
) -> Result<(), Error> {
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
        let content = match proposal.clone().content {
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

        match content.type_url.as_str() {
            SCHEDULED_CORK_PROPOSAL => {
                handle_scheduled_cork_proposal(state, proposal, proposal_id, content).await;
            }
            ADD_PUBLISHER_PROPOSAL
            | REMOVE_PUBLISHER_PROPOSAL
            | ADD_DEFAULT_SUBSCRIPTION_PROPOSAL
            | REMOVE_DEFAULT_SUBSCRIPTION_PROPOSAL => {
                debug!("proposal {} not a ScheduledCorkProposal", proposal_id);
                match refresh_publisher_trust_state_cache().await {
                    Ok(cache_changed) => {
                        if cache_changed {
                            // signal that the server should be restarted
                            debug!(
                                "the publisher trust state cache has changed. signaling server restart"
                            );
                            publisher_cache_tx.send(()).await.unwrap();
                        } else {
                            error!(
                                "a proposal passed but it is not reflected in the refreshed cache"
                            );
                        }
                    }
                    Err(err) => error!("failed to refresh cache after proposal approval: {err}"),
                }
            }
            // no-op
            _ => info!(
                "observed {} proposal approval with ID {proposal_id}",
                content.type_url
            ),
        };

        state.increment_proposal_id();
        continue;
    }

    Ok(())
}

pub async fn confirm_scheduling(
    state: &ProposalThreadState,
    cellar_id: &str,
    encoded_call: &[u8],
    block_height: u64,
) -> Result<bool, Error> {
    let id = id_hash(block_height, cellar_id, &encoded_call);
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

pub async fn log_schedule_failure(
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
