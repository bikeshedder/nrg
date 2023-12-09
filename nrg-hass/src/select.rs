use std::sync::Arc;

use rumqttc::AsyncClient;
use serde::Serialize;

use crate::{Availability, AvailabilityMode, Device, Qos};

/// https://www.home-assistant.io/integrations/select.mqtt/
#[derive(Default, Serialize)]
pub struct Select {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availabilty: Option<Vec<Availability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_mode: Option<AvailabilityMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,
    pub command_topic: String,
    // TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Arc<Device>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,
    pub options: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Select {
    pub async fn register(
        &self,
        client: &AsyncClient,
        hass_discovery_prefix: &str,
        device_prefix: &str,
        name: &str,
    ) -> Result<(), rumqttc::ClientError> {
        let json = serde_json::to_string(&self).unwrap();
        client
            .publish(
                format!("{hass_discovery_prefix}/select/{device_prefix}/{name}/config"),
                rumqttc::QoS::AtLeastOnce,
                true,
                json,
            )
            .await
    }
    pub async fn value(
        &self,
        client: &AsyncClient,
        payload: &str,
    ) -> Result<(), rumqttc::ClientError> {
        // FIXME panicking when there is no state_topic set is a bad idea
        client
            .publish(
                self.state_topic.as_ref().expect("state_topic is not set"),
                rumqttc::QoS::AtLeastOnce,
                true,
                payload,
            )
            .await
    }
}

#[derive(Default, Serialize)]
pub struct OptionsItem {
    pub name: String,
    pub id: String,
    pub value: String,
}
