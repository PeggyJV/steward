use std::time::Duration;

use abscissa_core::tracing::{debug, error, info, warn};
use abscissa_core::Application;
use prost_types::Any;
use somm_proto::cork::ScheduledCorkProposal;
use somm_proto::cosmos_sdk_proto::cosmos::gov::v1beta1::Proposal;

use crate::{
    cellars::{aave_v2_stablecoin, cellar_v1, cellar_v2, cellar_v2_2, cellar_v2_5},
    config::DELEGATE_ADDRESS,
    cork::schedule_cork,
    error::{Error, ErrorKind},
    prelude::APP,
    proto::{governance_call::Call, GovernanceCall},
};

use super::{client::CorkQueryClient, id_hash};

type GovQueryClient = gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::gov::v1beta1::query_client::QueryClient<Channel>;
type GravityQueryClient =
    gravity_bridge::gravity_proto::gravity::query_client::QueryClient<Channel>;
type TendermintQueryClient = gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::service_client::ServiceClient<Channel>;

const RETRY_SLEEP: u64 = 5;

pub async fn handle_scheduled_cork_proposal(
    state: &mut ProposalThreadState,
    proposal: Proposal,
    proposal_id: u64,
    content: Any,
) {
    info!("processing scheduled cork proposal of ID {}", proposal_id);
    let cork_proposal: ScheduledCorkProposal =
        match prost::Message::decode(content.value.as_slice()) {
            Ok(c) => c,
            Err(err) => {
                error!(
                    "failed to decode ScheduledCorkProposal {}: {}",
                    proposal_id, err
                );
                return;
            }
            Call::CellarV25(data) => {
                if data.function.is_none() {
                    warn!(
                        "scheduled cork proposal {} call data contains no function data and will be ignored: {:?}",
                        proposal.proposal_id, data,
                    );
                    state.increment_proposal_id();
                    continue;
                }
                let function = data.function.unwrap();
                match cellar_v2_5::get_encoded_governance_call(
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

    if cork_proposal.block_height <= state.last_observed_height {
        info!(
            "proposal {} block height {} is in the past. skipping!",
            proposal_id, cork_proposal.block_height
        );

        return;
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
            return;
        }
    };
    if governance_call.call.is_none() {
        warn!(
            "governance call for proposal {} is empty and will be ignored: {:?}",
            proposal_id, governance_call
        );
    }
    let encoded_call: Vec<u8> = match governance_call.call.unwrap() {
        Call::AaveV2Stablecoin(data) => {
            if data.function.is_none() {
                warn!(
                    "scheduled cork proposal {} call data contains no function data and will be ignored: {:?}",
                    proposal.proposal_id, data,
                );
            }
            let function = data.function.unwrap();
            match aave_v2_stablecoin::get_encoded_governance_call(
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
                    return;
                }
            }
        }
        Call::CellarV1(data) => {
            if data.function.is_none() {
                warn!(
                    "scheduled cork proposal {} call data contains no function data and will be ignored: {:?}",
                    proposal.proposal_id, data,
                );
            }
            let function = data.function.unwrap();
            match cellar_v1::get_encoded_governance_call(function, &cellar_id, proposal.proposal_id)
            {
                Ok(d) => d,
                // this is likely a bug in steward
                Err(err) => {
                    error!(
                        "failed to get encoded governance call data for proposal {}: {}",
                        proposal.proposal_id, err
                    );
                    return;
                }
            }
        }
        Call::CellarV2(data) => {
            if data.function.is_none() {
                warn!(
                    "scheduled cork proposal {} call data contains no function data and will be ignored: {:?}",
                    proposal.proposal_id, data,
                );
            }
            let function = data.function.unwrap();
            match cellar_v2::get_encoded_governance_call(function, &cellar_id, proposal.proposal_id)
            {
                Ok(d) => d,
                // this is likely a bug in steward
                Err(err) => {
                    error!(
                        "failed to get encoded governance call data for proposal {}: {}",
                        proposal.proposal_id, err
                    );
                    return;
                }
            }
        }
        Call::CellarV22(data) => {
            if data.function.is_none() {
                warn!(
                    "scheduled cork proposal {} call data contains no function data and will be ignored: {:?}",
                    proposal.proposal_id, data,
                );
            }
            let function = data.function.unwrap();
            match cellar_v2_2::get_encoded_governance_call(
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
                    return;
                }
            }
        }
    };

    debug!(
        "proposal {} encoded call: {:?}",
        proposal.proposal_id, encoded_call
    );

    // retry scheduling call on errors up to configured amount of retries
    let config = APP.config();
    let mut attempts = 0;
    loop {
        if let Err(schedule_err) =
            schedule_cork(&cellar_id, encoded_call.clone(), block_height).await
        {
            match confirm_scheduling(state, &cellar_id, &encoded_call, block_height).await {
                Ok(confirmed) => {
                    if confirmed {
                        info!(
                            "call for cellar {} scheduled for block {} by proposal {}",
                            cellar_id, block_height, proposal.proposal_id
                        );
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
            break;
        }
    }
}
