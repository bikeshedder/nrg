use std::sync::Arc;

use rumqttc::AsyncClient;
use serde::Serialize;

use crate::{Device, DeviceClass, UnitOfMeasurement};

#[derive(Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum NumberMode {
    #[default]
    Auto,
    Box,
    Slider,
}

/// https://www.home-assistant.io/integrations/number.mqtt/
#[derive(Default, Serialize)]
pub struct Number {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Arc<Device>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class: Option<DeviceClass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<NumberMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measurement: Option<UnitOfMeasurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Number {
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
                format!("{hass_discovery_prefix}/number/{device_prefix}/{name}/config"),
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
