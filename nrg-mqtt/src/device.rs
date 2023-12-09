use serde::{Deserialize, Serialize};

use crate::{
    client::MqttClient,
    topic::{Input, Output, Topic},
};

pub struct Device {
    client: MqttClient,
    name: String,
}

impl Device {
    pub fn new(client: MqttClient, name: impl Into<String>) -> Self {
        Self {
            client,
            name: name.into(),
        }
    }
    pub fn output<T: Serialize>(&self, topic: Topic<T>) -> Output<T> {
        Output::new(topic)
    }
    pub fn input<T: Deserialize<'static>>(&self, topic: Topic<T>) -> Input<T> {
        Input::new(topic)
    }
}
