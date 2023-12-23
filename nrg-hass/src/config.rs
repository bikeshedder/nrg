use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HomeAssistantConfig {
    pub discovery_prefix: String,
    pub object_id: String,
    pub name: String,
}
