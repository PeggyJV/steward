mod add_subscriber;
mod remove_subscriber;
mod subscribe;
mod unsubscribe;

use abscissa_core::{clap::Parser, Command, Runnable};

/// Commands for managing stewards identity and cellar subscriptions in the pubsub module
#[derive(Command, Debug, Parser, Runnable)]
pub enum PubsubCmd {
    AddSubscriber(add_subscriber::AddSubscriberCmd),
    RemoveSubscriber(remove_subscriber::RemoveSubscriberCmd),
    Subscribe(subscribe::SubscribeCmd),
    Unsubscribe(unsubscribe::UnsubscribeCmd),
}
