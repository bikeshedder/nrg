use std::sync::Arc;

use derive_builder::Builder;
use serde::Serialize;

use crate::{discovery::Discovery, state::State};

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
    // This field is marked as optional in the docs but since
    // the field is required for the auto discovery to work it
    // is marked as required.
    pub object_id: String,
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
    pub fn builder() -> NumberBuilder {
        NumberBuilder::default()
    }
}

impl Discovery for Number {
    const COMPONENT: &'static str = "number";
    fn object_id(&self) -> &str {
        &self.object_id
    }
}

impl State for Number {
    fn topic(&self) -> &str {
        self.state_topic.as_ref().unwrap() // FIXME
    }
}
