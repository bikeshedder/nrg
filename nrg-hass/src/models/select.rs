use std::sync::Arc;

use derive_builder::Builder;
use serde::Serialize;

use crate::{discovery::Discovery, state::State};

use super::{
    availability::{Availability, AvailabilityMode},
    device::Device,
    qos::Qos,
};

/// https://www.home-assistant.io/integrations/select.mqtt/
#[derive(Clone, Debug, Default, Serialize, Builder)]
#[builder(default, setter(into, strip_option))]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Arc<Device>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,
    pub options: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,
    // This field is marked as optional in the docs but since we
    // use this to construct the state topics it is required by
    // this implementation.
    pub unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Select {
    pub fn builder() -> SelectBuilder {
        SelectBuilder::default()
    }
}

// FIXME it would be nice if this was a proper enum instead
#[derive(Default, Serialize)]
pub struct OptionsItem {
    pub name: String,
    pub id: String,
    pub value: String,
}

impl Discovery for Select {
    const COMPONENT: &'static str = "select";
    fn unique_id(&self) -> &str {
        &self.unique_id
    }
}

// FIXME replace this by proper enum
impl State for Select {
    fn topic(&self) -> &str {
        self.state_topic.as_ref().unwrap() // FIXME
    }
}
