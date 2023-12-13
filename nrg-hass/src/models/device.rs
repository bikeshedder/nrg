use derive_builder::Builder;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct Device {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<(String, String)>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hw_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_area: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sw_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_device: Option<String>,
}

impl Device {
    pub fn builder() -> DeviceBuilder {
        DeviceBuilder::default()
    }
}
