use deep_space::coin::Coin;
use deep_space::error::CosmosGrpcError;
use deep_space::Contact;
use deep_space::Fee;
use deep_space::Msg;
use deep_space::PrivateKey as CosmosPrivateKey;
use gravity_bridge::gravity_proto::cosmos_sdk_proto::cosmos::{
    base::abci::v1beta1::TxResponse, tx::v1beta1::BroadcastMode,
};
use somm_proto::cork::Cork;
use somm_proto::cork::MsgScheduleCorkRequest;
use std::{result::Result, time::Duration};

pub const TIMEOUT: Duration = Duration::from_secs(60);
pub const MEMO: &str = "Sent using Somm Orchestrator";

pub async fn schedule_cork(
    contact: &Contact,
    cork: Cork,
    delegate_address: String,
    delegate_key: &CosmosPrivateKey,
    fee: Coin,
    gas_limit_per_msg: u64,
    block_height: u64,
) -> Result<TxResponse, CosmosGrpcError> {
    let msg = MsgScheduleCorkRequest {
        cork: Some(cork),
        signer: delegate_address,
        block_height,
    };
    let msg = Msg::new("/cork.v2.MsgScheduleCorkRequest", msg);
    __send_messages(contact, delegate_key, fee, gas_limit_per_msg, vec![msg]).await
}

async fn __send_messages(
    contact: &Contact,
    cosmos_key: &CosmosPrivateKey,
    fee: Coin,
    gas_limit_per_msg: u64,
    messages: Vec<Msg>,
) -> Result<TxResponse, CosmosGrpcError> {
    let cosmos_address = cosmos_key.to_address(&contact.get_prefix())?;
    let gas_limit = gas_limit_per_msg * (messages.len() as u64);

    // block gas limit check
    if gas_limit > 20_000_000u64 {
        return Err(CosmosGrpcError::BadInput(
            "total gas limit too high. (gas limit) * (number of messages) exceeded 20M".to_string(),
        ));
    }

    let fee = Fee {
        amount: vec![fee],
        gas_limit,
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
