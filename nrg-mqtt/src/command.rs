use std::sync::Arc;

use anyhow::Result;
use rumqttc::{AsyncClient, Publish};
use serde::de::DeserializeOwned;
use tokio::sync::{mpsc, Mutex};
use tracing::error;

use crate::client::{MqttClient, Sender, SubscribeError};

pub struct Commands<T> {
    client: Arc<MqttClient>,
    tx: mpsc::UnboundedSender<T>,
    rx: Mutex<mpsc::UnboundedReceiver<T>>,
}

pub struct Command<T> {
    decoder: Box<dyn Decoder<T>>,
    tx: mpsc::UnboundedSender<T>,
}

impl<T: Send> Sender for Command<T> {
    fn send(&self, publish: &Publish) {
        match self.decoder.decode(&publish.payload) {
            Ok(payload) => {
                let _ = self.tx.send(payload);
            }
            Err(e) => {
                error!(
                    "Unable to decode payload for topic {}: {}",
                    publish.topic, e
                )
            }
        }
    }
}

impl<T: Send + 'static> Commands<T> {
    pub fn new(client: Arc<MqttClient>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            client,
            tx,
            rx: Mutex::new(rx),
        }
    }
    pub async fn cmd(
        &self,
        topic: &str,
        decoder: impl Decoder<T> + 'static,
    ) -> Result<(), SubscribeError> {
        self.client
            .sub(
                topic,
                Command {
                    decoder: Box::new(decoder),
                    tx: self.tx.clone(),
                },
            )
            .await
    }
    pub async fn next(&self) -> Option<T> {
        self.rx.lock().await.recv().await
    }
    pub fn client(&self) -> &AsyncClient {
        &self.client
    }
}

pub trait Decoder<T>: Send {
    fn decode(&self, data: &[u8]) -> Result<T>;
}

pub struct JsonDecoder<P, T>(pub fn(P) -> T);

impl<P, T> Decoder<T> for JsonDecoder<P, T>
where
    P: DeserializeOwned,
{
    fn decode(&self, data: &[u8]) -> Result<T> {
        Ok(self.0(serde_json::from_slice(data)?))
    }
}
