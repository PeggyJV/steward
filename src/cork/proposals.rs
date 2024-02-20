use std::time::Duration;

use abscissa_core::tracing::{debug, error, info};
use abscissa_core::Application;
use prost_types::Any;
use somm_proto::axelar_cork::AxelarScheduledCorkProposal;
use somm_proto::cork::ScheduledCorkProposal;
use somm_proto::cosmos_sdk_proto::cosmos::gov::v1beta1::Proposal;

use crate::cork::ETHEREUM_CHAIN_ID;
use crate::error::{Error, ErrorKind};
use crate::{
    cellars::{aave_v2_stablecoin, cellar_v1, cellar_v2, cellar_v2_2, cellar_v2_5},
    cork::{schedule_axelar_cork, schedule_cork},
    prelude::APP,
    proposals::{confirm_scheduling, log_schedule_failure, ProposalThreadState},
    proto::{governance_call::Call, GovernanceCall},
};

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

    let encoded_call: Vec<u8> = match get_encoded_governance_call(json, &cellar_id, proposal_id) {
        Ok(d) => d,
        Err(err) => {
            error!(
                "failed to get encoded governance call data for proposal {}: {}",
                proposal_id, err
            );
            return;
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
        let schedule_result = schedule_cork(&cellar_id, encoded_call.clone(), block_height).await;

        if schedule_result.is_ok() {
            debug!("schedule response: {:?}", schedule_result);
            info!(
                "governance call for cellar {} scheduled for block {} by proposal {}",
                cellar_id, block_height, proposal.proposal_id
            );

            return;
        };

        let schedule_err = schedule_result.unwrap_err();
        let confirm_err = match confirm_scheduling(
            state,
            ETHEREUM_CHAIN_ID,
            &cellar_id,
            &encoded_call,
            block_height,
        )
        .await
        {
            Ok(confirmed) => {
                if confirmed {
                    info!(
                        "governance call for chain {} cellar {} scheduled for block {} by proposal {}",
                        ETHEREUM_CHAIN_ID, cellar_id, block_height, proposal.proposal_id
                    );

                    return;
                }

                None
            }
            Err(confirm_err) => Some(confirm_err),
        };

        log_schedule_failure(proposal.proposal_id, schedule_err, confirm_err).await;

        info!(
            "scheduling attempts {}/{}",
            attempts, config.cork.max_scheduling_retries
        );

        attempts += 1;
        if attempts > config.cork.max_scheduling_retries {
            return;
        }

        tokio::time::sleep(Duration::from_secs(RETRY_SLEEP)).await;
    }
}

pub(crate) async fn handle_axelar_scheduled_cork_proposal(
    state: &mut ProposalThreadState,
    proposal: Proposal,
    proposal_id: u64,
    content: Any,
) {
    info!(
        "processing scheduled axelar cork proposal of ID {}",
        proposal_id
    );
    let cork_proposal: AxelarScheduledCorkProposal =
        match prost::Message::decode(content.value.as_slice()) {
            Ok(c) => c,
            Err(err) => {
                error!(
                    "failed to decode AxelarScheduledCorkProposal {}: {}",
                    proposal_id, err
                );
                return;
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
    let chain_id = cork_proposal.chain_id;
    let block_height = cork_proposal.block_height;
    let deadline = cork_proposal.deadline;
    debug!(
        "proposal {} contract call proto JSON: {}",
        proposal_id, json
    );

    let encoded_call: Vec<u8> = match get_encoded_governance_call(json, &cellar_id, proposal_id) {
        Ok(d) => d,
        Err(err) => {
            error!(
                "failed to get encoded governance call data for proposal {}: {}",
                proposal_id, err
            );
            return;
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
        let schedule_result = schedule_axelar_cork(
            chain_id,
            &cellar_id,
            encoded_call.clone(),
            block_height,
            deadline,
        )
        .await;

        if schedule_result.is_ok() {
            debug!("schedule response: {:?}", schedule_result);
            info!(
                "governance call for chain ID {} cellar {} scheduled for block {} by proposal {}",
                chain_id, cellar_id, block_height, proposal.proposal_id
            );

            return;
        };

        let schedule_err = schedule_result.unwrap_err();
        let confirm_err = match confirm_scheduling(
            state,
            chain_id,
            &cellar_id,
            &encoded_call,
            block_height,
        )
        .await
        {
            Ok(confirmed) => {
                if confirmed {
                    info!(
                        "governance call for chain {} cellar {} scheduled for block {} by proposal {}",
                        chain_id, cellar_id, block_height, proposal.proposal_id
                    );

                    return;
                }

                None
            }
            Err(confirm_err) => Some(confirm_err),
        };

        log_schedule_failure(proposal.proposal_id, schedule_err, confirm_err).await;

        info!(
            "scheduling attempts {}/{}",
            attempts, config.cork.max_scheduling_retries
        );

        attempts += 1;
        if attempts > config.cork.max_scheduling_retries {
            return;
        }

        tokio::time::sleep(Duration::from_secs(RETRY_SLEEP)).await;
    }
}

fn get_encoded_governance_call(
    json: String,
    cellar_id: &str,
    proposal_id: u64,
) -> Result<Vec<u8>, Error> {
    let governance_call = match serde_json::from_str::<GovernanceCall>(&json) {
        Ok(c) => c,
        Err(err) => {
            return Err(ErrorKind::CallDecodeError
                .context(format!(
                    "failed to decode GovernanceCall JSON {}: {}",
                    json, err
                ))
                .into());
        }
    };
    if governance_call.call.is_none() {
        return Err(ErrorKind::GovernanceCall
            .context(format!(
                "governance call for proposal {} is empty and will be ignored: {:?}",
                proposal_id, governance_call
            ))
            .into());
    }
    match governance_call.call.unwrap() {
        Call::AaveV2Stablecoin(data) => {
            if data.function.is_none() {
                return Err(ErrorKind::GovernanceCall
                    .context(format!(
                        "proposal {} call data contains no function data and will be ignored: {:?}",
                        proposal_id, data,
                    ))
                    .into());
            }

            let function = data.function.unwrap();
            aave_v2_stablecoin::get_encoded_governance_call(function, cellar_id, proposal_id)
        }
        Call::CellarV1(data) => {
            if data.function.is_none() {
                return Err(ErrorKind::GovernanceCall
                    .context(format!(
                        "proposal {} call data contains no function data and will be ignored: {:?}",
                        proposal_id, data,
                    ))
                    .into());
            }

            let function = data.function.unwrap();
            cellar_v1::get_encoded_governance_call(function, cellar_id, proposal_id)
        }
        Call::CellarV2(data) => {
            if data.function.is_none() {
                return Err(ErrorKind::GovernanceCall
                    .context(format!(
                        "proposal {} call data contains no function data and will be ignored: {:?}",
                        proposal_id, data,
                    ))
                    .into());
            }

            let function = data.function.unwrap();
            cellar_v2::get_encoded_governance_call(function, cellar_id, proposal_id)
        }
        Call::CellarV22(data) => {
            if data.call_type.is_none() {
                return Err(ErrorKind::GovernanceCall
                    .context(format!(
                        "proposal {} call data contains no function data and will be ignored: {:?}",
                        proposal_id, data,
                    ))
                    .into());
            }

            let call = data.call_type.unwrap();
            cellar_v2_2::get_encoded_governance_call(call, cellar_id, proposal_id)
        }
        Call::CellarV25(data) => {
            if data.call_type.is_none() {
                return Err(ErrorKind::GovernanceCall
                    .context(format!(
                        "proposal {} call data contains no function data and will be ignored: {:?}",
                        proposal_id, data,
                    ))
                    .into());
            }

            let call = data.call_type.unwrap();
            cellar_v2_5::get_encoded_governance_call(call, cellar_id, proposal_id)
        }
    }
}
