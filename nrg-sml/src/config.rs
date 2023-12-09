use nrg_hass::config::HomeAssistantConfig;
use nrg_mqtt::config::MqttConfig;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub serial: SerialConfig,
    pub mqtt: MqttConfig,
    #[serde(rename = "home-assistant")]
    pub hass: HomeAssistantConfig,
}

#[derive(Debug, Deserialize)]
pub struct SerialConfig {
    pub device: String,
    #[serde(default = "default_baud")]
    pub baud: u32,
}

fn default_baud() -> u32 {
    9600
}
