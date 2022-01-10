use bytes::BytesMut;
use deep_space::address::Address;
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
use somm_proto::somm;
use somm_proto::somm::query_client::QueryClient as AllocationQueryClient;
use somm_proto::somm::AllocationPrecommit;
use std::{result::Result, time::Duration};
use tonic::transport::Channel;

use crate::error::Error;
use crate::error::ErrorKind;

pub const TIMEOUT: Duration = Duration::from_secs(60);
pub const MEMO: &str = "Sent using Somm Orchestrator";

pub async fn send_precommit(
    contact: &Contact,
    delegate_cosmos_address: Address,
    cosmos_key: CosmosPrivateKey,
    fee: Coin,
    allocation_precommit: Vec<somm::AllocationPrecommit>,
) -> Result<TxResponse, CosmosGrpcError> {
    let msg = somm::MsgAllocationPrecommit {
        precommit: allocation_precommit,
        signer: delegate_cosmos_address.to_bech32("somm").unwrap(),
    };

    let msg = Msg::new("/allocation.v1.MsgAllocationPrecommit", msg);
    __send_messages(contact, cosmos_key, fee, vec![msg]).await
}

pub async fn send_allocation(
    contact: &Contact,
    delegate_cosmos_address: Address,
    cosmos_key: CosmosPrivateKey,
    fee: Coin,
    allocation_commit: Vec<somm::Allocation>,
) -> Result<TxResponse, CosmosGrpcError> {
    let msg = somm::MsgAllocationCommit {
        signer: delegate_cosmos_address.to_bech32("somm").unwrap(),
        commit: allocation_commit,
    };

    let msg = Msg::new("/allocation.v1.MsgAllocationCommit", msg);
    __send_messages(contact, cosmos_key, fee, vec![msg]).await
}

async fn __send_messages(
    contact: &Contact,
    cosmos_key: CosmosPrivateKey,
    fee: Coin,
    messages: Vec<Msg>,
) -> Result<TxResponse, CosmosGrpcError> {
    let cosmos_address = cosmos_key.to_address(&contact.get_prefix()).unwrap();

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
    allocation: &somm::Allocation,
    val_address: String,
) -> Result<AllocationPrecommit, Error> {
    let mut hasher = sha2::Sha256::new();
    if let Some(cellar) = &allocation.clone().vote.unwrap().cellar {
        let mut buf = BytesMut::new();
        cellar.encode(&mut buf).unwrap();
        let cellar_data = hex::encode(&buf).to_string();
        let msg = format!("{}:{}:{}", allocation.salt, cellar_data, val_address);
        hasher.update(msg.as_bytes());

        return Ok(AllocationPrecommit {
            hash: hasher.finalize().to_vec(),
            cellar_id: cellar.id.clone(),
        });
    }
    return Err(ErrorKind::AllocationError.context("No cellar".to_string()).into());
}

pub async fn query_allocation_precommits(
    client: &mut AllocationQueryClient<Channel>,
) -> Result<Vec<AllocationPrecommit>, CosmosGrpcError> {
    let response = client
        .query_allocation_precommits(somm::QueryAllocationPrecommitsRequest {})
        .await?;
    let precommits = response.into_inner().precommits;
    Ok(precommits)
}

pub async fn query_commit_period(
    client: &mut AllocationQueryClient<Channel>,
) -> Result<somm::QueryCommitPeriodResponse, CosmosGrpcError> {
    let response = client
        .query_commit_period(somm::QueryCommitPeriodRequest {})
        .await?;

    let query_commit_response = response.into_inner();
    Ok(query_commit_response)
}
