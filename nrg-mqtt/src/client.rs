use std::{ops::Deref, sync::Arc};

use rumqttc::{AsyncClient, EventLoop, MqttOptions, Packet, QoS};
use tokio::sync::Mutex;

use crate::{config::MqttConfig, topic::Pattern};

struct Subscription {
    pattern: Pattern,
    cb: Box<SubscriptionCallback>,
}

type SubscriptionCallback = dyn Fn(&str, &[u8]) + Sync + Send + 'static;

type Subscriptions = Arc<Mutex<Vec<Subscription>>>;

pub struct MqttClient {
    client: AsyncClient,
    subscriptions: Subscriptions,
}

impl MqttClient {
    pub fn new(config: &MqttConfig) -> Self {
        let mut options = MqttOptions::new(
            config.id.clone(),
            config.addr.ip().to_string(),
            config.addr.port(),
        );
        if let Some(cred) = &config.credentials {
            options.set_credentials(cred.username.clone(), cred.password.clone());
        }
        options.set_keep_alive(config.keepalive);
        let (client, eventloop) = rumqttc::AsyncClient::new(options, config.capacity);

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
        notify: impl Fn(&str, &[u8]) + Sync + Send + 'static,
    ) -> Result<(), rumqttc::ClientError> {
        let topic = topic.into();
        let pattern = Pattern::parse(&topic).unwrap(); // XXX
        self.subscriptions.lock().await.push(Subscription {
            pattern,
            cb: Box::new(notify),
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
        let rumqttc::Event::Incoming(Packet::Publish(publish)) = &notification else {
            continue;
        };
        for subscription in subscriptions.lock().await.iter() {
            if subscription.pattern.matches(&publish.topic) {
                (*subscription.cb)(&publish.topic, &publish.payload);
            }
        }
        // XXX
        // info!("Publish packed without a subscription: {}", publish.topic);
    }
}
