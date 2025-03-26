use abscissa_core::{tracing::debug, Application};
use cosmos_sdk_proto_althea::cosmos::base::abci::v1beta1::TxResponse;
use cosmos_sdk_proto_althea::cosmos::tx::v1beta1::BroadcastMode;
use deep_space::coin::Coin;
use deep_space::error::CosmosGrpcError;
use deep_space::Contact;
use deep_space::Fee;
use deep_space::Msg;
use deep_space::PrivateKey;
use somm_proto::axelar_cork::AxelarCork;
use somm_proto::axelar_cork::MsgScheduleAxelarCorkRequest;
use somm_proto::cork::Cork;
use somm_proto::cork::MsgScheduleCorkRequest;
use somm_proto::pubsub::Subscriber;
use somm_proto::pubsub::{
    MsgAddSubscriberIntentRequest, MsgAddSubscriberRequest, MsgRemoveSubscriberIntentRequest,
    MsgRemoveSubscriberRequest, SubscriberIntent,
};
use std::{result::Result, time::Duration};

use crate::config::get_delegate_address;
use crate::config::get_delegate_key;
use crate::prelude::APP;

const MESSAGE_TIMEOUT: Duration = Duration::from_secs(10);
const CHAIN_PREFIX: &str = "somm";
pub const TIMEOUT: Duration = Duration::from_secs(60);
pub const MEMO: &str = "Sent using Somm Orchestrator";
pub const MAX_GAS_PER_BLOCK: u64 = 60_000_000;

pub async fn add_subscriber(subscriber: Subscriber) -> Result<TxResponse, CosmosGrpcError> {
    let msg = MsgAddSubscriberRequest {
        subscriber: Some(subscriber),
        signer: get_delegate_address().to_string(),
    };
    let msg = Msg::new("/pubsub.v1.MsgAddSubscriberRequest", msg);

    send_messages(vec![msg]).await
}

pub async fn remove_subscriber() -> Result<TxResponse, CosmosGrpcError> {
    let signer = get_delegate_address().to_string();
    let msg = MsgRemoveSubscriberRequest {
        subscriber_address: signer.clone(),
        signer,
    };
    let msg = Msg::new("/pubsub.v1.MsgAddSubscriberRequest", msg);

    send_messages(vec![msg]).await
}

pub async fn subscribe(
    cellar_id: String,
    publisher_domain: String,
) -> Result<TxResponse, CosmosGrpcError> {
    let signer = get_delegate_address().to_string();
    let subscriber_intent = SubscriberIntent {
        subscription_id: cellar_id,
        subscriber_address: signer.clone(),
        publisher_domain,
    };
    let msg = MsgAddSubscriberIntentRequest {
        subscriber_intent: Some(subscriber_intent),
        signer,
    };
    let msg = Msg::new("/pubsub.v1.MsgAddSubscriberIntentRequest", msg);

    send_messages(vec![msg]).await
}

pub async fn unsubscribe(cellar_id: String) -> Result<TxResponse, CosmosGrpcError> {
    let signer = get_delegate_address().to_string();
    let msg = MsgRemoveSubscriberIntentRequest {
        subscription_id: cellar_id,
        subscriber_address: signer.clone(),
        signer,
    };
    let msg = Msg::new("/pubsub.v1.MsgRemoveSubscriberIntentRequest", msg);

    send_messages(vec![msg]).await
}

pub async fn schedule_cork(cork: Cork, block_height: u64) -> Result<TxResponse, CosmosGrpcError> {
    let msg = MsgScheduleCorkRequest {
        cork: Some(cork),
        signer: get_delegate_address().to_string(),
        block_height,
    };
    let msg = Msg::new("/cork.v2.MsgScheduleCorkRequest", msg);

    debug!("sending cork msg: {msg:?}");
    send_messages(vec![msg]).await
}

pub async fn schedule_axelar_cork(
    cork: AxelarCork,
    block_height: u64,
) -> Result<TxResponse, CosmosGrpcError> {
    let msg = MsgScheduleAxelarCorkRequest {
        chain_id: cork.chain_id,
        cork: Some(cork),
        signer: get_delegate_address().to_string(),
        block_height,
    };
    let msg = Msg::new("/axelarcork.v1.MsgScheduleAxelarCorkRequest", msg);

    debug!("sending axelarcork msg: {msg:?}");
    send_messages(vec![msg]).await
}

async fn send_messages(messages: Vec<Msg>) -> Result<TxResponse, CosmosGrpcError> {
    let contact = get_cosmos_client()?;
    let msg_bytes = get_signed_messages(messages, &contact).await?;
    let response = contact
        .send_transaction(msg_bytes, BroadcastMode::Sync)
        .await?;

    debug!("transaction response: {response:?}");
    debug!("waiting for tx to be included");

    contact.wait_for_tx(response, TIMEOUT).await.map(Into::into)
}

fn get_cosmos_client() -> Result<Contact, CosmosGrpcError> {
    let config = APP.config();
    Contact::new(&config.cosmos.grpc, MESSAGE_TIMEOUT, CHAIN_PREFIX)
}

async fn get_signed_messages(
    messages: Vec<Msg>,
    contact: &Contact,
) -> Result<Vec<u8>, CosmosGrpcError> {
    let delegate_address = get_delegate_address();
    let fee = get_fee(&messages);
    let args = contact
        .get_message_args(*delegate_address, fee, None)
        .await?;
    let msg_bytes = get_delegate_key().sign_std_msg(&messages, args, MEMO)?;

    Ok(msg_bytes)
}

fn get_fee(messages: &[Msg]) -> Fee {
    let config = APP.config();
    let cosmos_gas_price = config.cosmos.gas_price.as_tuple();
    let fee = Coin {
        amount: (cosmos_gas_price.0 as u64).into(),
        denom: cosmos_gas_price.1,
    };

    Fee {
        amount: vec![fee],
        gas_limit: 500_000u64 * (messages.len() as u64),
        granter: None,
        payer: None,
    }
}
