use std::{ops::Deref, sync::Arc};

use rumqttc::{AsyncClient, EventLoop, Packet, QoS};
use tokio::sync::Mutex;

use crate::{config::MqttConfig, topic::Pattern};

struct Subscription {
    pattern: Pattern,
    sender: Box<dyn Sender>,
}

pub trait Sender: Send {
    fn send(&self, topic: &str, data: &[u8]);
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
    pub async fn subscribe(
        &self,
        topic: impl Into<String>,
        sender: impl Sender + 'static,
    ) -> Result<(), rumqttc::ClientError> {
        let topic = topic.into();
        let pattern = Pattern::parse(&topic).unwrap(); // XXX
        self.subscriptions.lock().await.push(Subscription {
            pattern,
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
                subscription.sender.send(&publish.topic, &publish.payload);
            }
        }
        // XXX
        // info!("Publish packed without a subscription: {}", publish.topic);
    }
}
