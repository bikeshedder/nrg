use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HomeAssistantConfig {
    pub discovery_prefix: String,
    pub object_id: String,
    pub name: String,
}
