use std::{ops::Deref, sync::Arc};

use rumqttc::{AsyncClient, ClientError, EventLoop, Packet, Publish, QoS};
use thiserror::Error;
use tokio::sync::Mutex;

use crate::{
    config::MqttConfig,
    topic::{Pattern, PatternError},
};

struct Subscription {
    pattern: Pattern,
    sender: Box<dyn Sender>,
}

pub trait Sender: Send {
    fn send(&self, publish: &Publish);
}

pub struct CallbackSubscriber<C: Send + Clone> {
    pub context: C,
    pub callback: Box<dyn Fn(C, &Publish) + Send>,
}

impl<C: Send + Clone> CallbackSubscriber<C> {
    pub fn new(context: C, callback: impl Fn(C, &Publish) + Send + 'static) -> Self {
        Self {
            context,
            callback: Box::new(callback),
        }
    }
}

impl<C: Send + Clone> Sender for CallbackSubscriber<C> {
    fn send(&self, publish: &Publish) {
        (self.callback)(self.context.clone(), publish);
    }
}

type Subscriptions = Arc<Mutex<Vec<Subscription>>>;

pub struct MqttClient {
    client: AsyncClient,
    subscriptions: Subscriptions,
}

impl MqttClient {
    pub fn new(config: &MqttConfig) -> Self {
        let (client, eventloop) = config.client();

        let subscriptions = Arc::new(Mutex::new(Vec::new()));
        tokio::spawn(run_eventloop(eventloop, subscriptions.clone()));

        Self {
            client,
            subscriptions,
        }
    }
    pub async fn sub(
        &self,
        topic: &str,
        sender: impl Sender + 'static,
    ) -> Result<(), SubscribeError> {
        self.subscriptions.lock().await.push(Subscription {
            pattern: Pattern::parse(topic)?,
            sender: Box::new(sender),
        });
        self.client.subscribe(topic, QoS::AtLeastOnce).await?;
        Ok(())
    }
}

impl Deref for MqttClient {
    type Target = rumqttc::AsyncClient;
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

async fn run_eventloop(mut eventloop: EventLoop, subscriptions: Subscriptions) {
    while let Ok(notification) = eventloop.poll().await {
        let rumqttc::Event::Incoming(Packet::Publish(publish)) = notification else {
            continue;
        };
        for subscription in subscriptions.lock().await.iter() {
            if subscription.pattern.matches(&publish.topic) {
                subscription.sender.send(&publish);
            }
        }
        // XXX
        // info!("Publish packed without a subscription: {}", publish.topic);
    }
}

#[derive(Debug, Error)]
pub enum SubscribeError {
    #[error("Client error")]
    Client(#[from] ClientError),
    #[error("Parse topic error")]
    Topic(#[from] PatternError),
}
