use std::{ops::Deref, sync::Arc};

use dashmap::DashMap;
use rumqttc::{AsyncClient, EventLoop, MqttOptions, Packet, QoS};
use tracing::info;

use crate::config::MqttConfig;

type Subscription = dyn Fn(&[u8]) + Sync + Send + 'static;

type Subscriptions = DashMap<String, Box<Subscription>>;

pub struct MqttClient {
    client: AsyncClient,
    subscriptions: Arc<Subscriptions>,
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

        let subscriptions = Arc::new(Subscriptions::new());

        tokio::spawn(run_eventloop(eventloop, subscriptions.clone()));

        Self {
            client,
            subscriptions,
        }
    }
    pub async fn subscribe(
        &self,
        topic: impl Into<String>,
        notify: impl Fn(&[u8]) + Sync + Send + 'static,
    ) -> Result<(), rumqttc::ClientError> {
        let topic = topic.into();
        self.subscriptions.insert(topic.clone(), Box::new(notify));
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

async fn run_eventloop(mut eventloop: EventLoop, subscriptions: Arc<Subscriptions>) {
    while let Ok(notification) = eventloop.poll().await {
        let rumqttc::Event::Incoming(Packet::Publish(publish)) = &notification else {
            continue;
        };
        let Some(subscription) = subscriptions.get(&publish.topic) else {
            info!("Publish packed without a subscription: {}", publish.topic);
            continue;
        };
        subscription(&publish.payload);
    }
}
