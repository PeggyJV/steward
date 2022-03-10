use crate::error::{Error, ErrorKind};
use bytes::BytesMut;
use deep_space::coin::Coin;
use deep_space::error::CosmosGrpcError;
use deep_space::private_key::PrivateKey as CosmosPrivateKey;
use deep_space::Contact;
use deep_space::Fee;
use deep_space::Msg;
use gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::{
    base::abci::v1beta1::TxResponse, tx::v1beta1::BroadcastMode,
};
use prost::Message;
use sha2::Digest;
use somm_proto::allocation::{
    query_client::QueryClient as AllocationQueryClient, Allocation, AllocationPrecommit,
    MsgAllocationCommit, MsgAllocationPrecommit, QueryAllocationPrecommitsRequest,
    QueryCommitPeriodRequest, QueryCommitPeriodResponse,
};
use somm_proto::cork::Cork;
use somm_proto::cork::MsgSubmitCorkRequest;
use std::{result::Result, time::Duration};
use tonic::transport::Channel;

pub const TIMEOUT: Duration = Duration::from_secs(60);
pub const MEMO: &str = "Sent using Somm Orchestrator";

pub async fn send_precommit(
    contact: &Contact,
    delegate_address: String,
    delegate_key: CosmosPrivateKey,
    fee: Coin,
    allocation_precommit: Vec<AllocationPrecommit>,
) -> Result<TxResponse, CosmosGrpcError> {
    let msg = MsgAllocationPrecommit {
        precommit: allocation_precommit,
        signer: delegate_address,
    };

    let msg = Msg::new("/allocation.v1.MsgAllocationPrecommit", msg);
    __send_messages(contact, delegate_key, fee, vec![msg]).await
}

pub async fn send_allocation(
    contact: &Contact,
    delegate_address: String,
    delegate_key: CosmosPrivateKey,
    fee: Coin,
    allocation_commit: Vec<Allocation>,
) -> Result<TxResponse, CosmosGrpcError> {
    let msg = MsgAllocationCommit {
        commit: allocation_commit,
        signer: delegate_address,
    };

    let msg = Msg::new("/allocation.v1.MsgAllocationCommit", msg);
    __send_messages(contact, delegate_key, fee, vec![msg]).await
}

pub async fn send_cork(
    contact: &Contact,
    cork: Cork,
    delegate_address: String,
    delegate_key: CosmosPrivateKey,
    fee: Coin,
) -> Result<TxResponse, CosmosGrpcError> {
    let msg = MsgSubmitCorkRequest {
        cork: Some(cork),
        signer: delegate_address,
    };
    let msg = Msg::new("/cork.v1.MsgSubmitCorkRequest", msg);
    __send_messages(contact, delegate_key, fee, vec![msg]).await
}

async fn __send_messages(
    contact: &Contact,
    cosmos_key: CosmosPrivateKey,
    fee: Coin,
    messages: Vec<Msg>,
) -> Result<TxResponse, CosmosGrpcError> {
    let cosmos_address = cosmos_key.to_address(&contact.get_prefix())?;

    let fee = Fee {
        amount: vec![fee],
        gas_limit: 500_000_000u64 * (messages.len() as u64),
        granter: None,
        payer: None,
    };

    let args = contact.get_message_args(cosmos_address, fee).await?;

    let msg_bytes = cosmos_key.sign_std_msg(&messages, args, MEMO)?;

    let response = contact
        .send_transaction(msg_bytes, BroadcastMode::Sync)
        .await?;

    contact.wait_for_tx(response, TIMEOUT).await
}

pub async fn data_hash(
    allocation: &Allocation,
    val_address: String,
) -> Result<AllocationPrecommit, Error> {
    let mut hasher = sha2::Sha256::new();
    if let Some(vote) = &allocation.clone().vote {
        let mut buf = BytesMut::new();
        vote.encode(&mut buf)?;
        let vote_data = hex::encode(&buf);
        let msg = format!("{}:{}:{}", allocation.salt, vote_data, val_address);
        hasher.update(msg.as_bytes());

        if vote.cellar.is_none() {
            return Err(ErrorKind::AllocationError
                .context("vote has empty cellar field")
                .into());
        }
        let cellar_id = vote.cellar.clone().unwrap().id;

        return Ok(AllocationPrecommit {
            hash: hasher.finalize().to_vec(),
            cellar_id,
        });
    }
    Err(ErrorKind::AllocationError
        .context("no vote".to_string())
        .into())
}

pub async fn query_allocation_precommits(
    client: &mut AllocationQueryClient<Channel>,
) -> Result<Vec<AllocationPrecommit>, CosmosGrpcError> {
    let response = client
        .query_allocation_precommits(QueryAllocationPrecommitsRequest {})
        .await?;
    let precommits = response.into_inner().precommits;
    Ok(precommits)
}

pub async fn query_commit_period(
    client: &mut AllocationQueryClient<Channel>,
) -> Result<QueryCommitPeriodResponse, CosmosGrpcError> {
    let response = client
        .query_commit_period(QueryCommitPeriodRequest {})
        .await?;

    let query_commit_response = response.into_inner();
    Ok(query_commit_response)
}
