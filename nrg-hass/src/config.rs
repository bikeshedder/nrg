use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HomeAssistantConfig {
    #[serde(default = "default_discovery_prefix")]
    pub discovery_prefix: String,
    pub object_id: String,
    pub name: String,
}

pub fn default_discovery_prefix() -> String {
    "homeassistant".into()
}
