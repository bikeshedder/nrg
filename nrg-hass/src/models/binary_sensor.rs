use std::{num::NonZeroU32, sync::Arc};

use crate::{discovery::Discovery, state::State};

use super::{
    availability::{Availability, AvailabilityMode},
    device::Device,
    device_class::DeviceClass,
    entity_category::EntityCategory,
    qos::Qos,
    state_class::StateClass,
    unit::UnitOfMeasurement,
};
use derive_builder::Builder;
use serde::Serialize;

/// https://www.home-assistant.io/integrations/binary_sensor.mqtt/
#[derive(Clone, Debug, Default, Serialize, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct BinarySensor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availabilty: Option<Vec<Availability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_mode: Option<AvailabilityMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_topic: Option<String>,
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
    pub expire_after: Option<NonZeroU32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_update: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,
    pub name: String,
    // This field is marked as optional in the docs but since
    // the field is required for the auto discovery to work it
    // is marked as required.
    pub object_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_delay: Option<NonZeroU32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_off: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_class: Option<StateClass>,
    pub state_topic: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl BinarySensor {
    pub fn builder() -> BinarySensorBuilder {
        BinarySensorBuilder::default()
    }
}

impl Discovery for BinarySensor {
    const COMPONENT: &'static str = "binary_sensor";
    fn object_id(&self) -> &str {
        &self.object_id
    }
}

impl State for BinarySensor {
    fn topic(&self) -> &str {
        &self.state_topic
    }
}
