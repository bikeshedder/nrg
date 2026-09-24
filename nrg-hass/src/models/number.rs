use std::sync::Arc;

use derive_builder::Builder;
use serde::Serialize;

use crate::{
    discovery::Discovery,
    models::{
        availability::{Availability, AvailabilityMode},
        entity_category::EntityCategory,
        qos::Qos,
    },
    state::State,
};

use super::{device::Device, device_class::DeviceClass, unit::UnitOfMeasurement};

#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum NumberMode {
    #[default]
    Auto,
    Box,
    Slider,
}

/// https://www.home-assistant.io/integrations/number.mqtt/
#[derive(Clone, Debug, Default, Serialize, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct Number {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availabilty: Option<Vec<Availability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_mode: Option<AvailabilityMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,
    pub command_topic: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Arc<Device>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class: Option<DeviceClass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<NumberMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_reset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    pub unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measurement: Option<UnitOfMeasurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Number {
    pub fn builder() -> NumberBuilder {
        NumberBuilder::default()
    }
}

impl Discovery for Number {
    const COMPONENT: &'static str = "number";
    fn unique_id(&self) -> &str {
        &self.unique_id
    }
}

impl State for Number {
    fn topic(&self) -> &str {
        self.state_topic.as_ref().unwrap() // FIXME
    }
}
