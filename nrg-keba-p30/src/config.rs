use std::{net::SocketAddr, time::Duration};

use nrg_hass::config::HomeAssistantConfig;
use nrg_mqtt::config::MqttConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub modbus: ModbusConfig,
    pub mqtt: MqttConfig,
    #[serde(rename = "home-assistant")]
    pub hass: HomeAssistantConfig,
}

#[derive(Debug, Deserialize)]
pub struct ModbusConfig {
    pub addr: SocketAddr,
    pub slave: u8,
    pub retry_delay: Duration,
    pub poll_delay: Duration,
}
