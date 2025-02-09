use std::time::Duration;

use nrg_hass::config::HomeAssistantConfig;
use nrg_mqtt::config::MqttConfig;
use serde::Deserialize;
use serde_with::serde_as;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mqtt: MqttConfig,
    pub hass: HomeAssistantConfig,
    pub sensors: Vec<SensorConfig>,
    pub w1: W1Config,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct SensorConfig {
    pub name: String,
    #[serde_as(as = "serde_with::hex::Hex")]
    pub serial: [u8; 6],
}

#[derive(Debug, Deserialize)]
pub struct W1Config {
    pub interval: Duration,
    #[serde(default = "default_w1_master")]
    pub master: String,
}

fn default_w1_master() -> String {
    "w1_bus_master1".into()
}
