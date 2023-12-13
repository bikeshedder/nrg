use derive_builder::Builder;
use serde::Serialize;

/// https://www.home-assistant.io/integrations/sensor.mqtt/#availability
#[derive(Clone, Debug, Default, Serialize, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct Availability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,
    pub topic: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

/// https://www.home-assistant.io/integrations/sensor.mqtt/#availability_mode
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityMode {
    All,
    Any,
    Latest,
}
