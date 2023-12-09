use std::{net::SocketAddr, time::Duration};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MqttConfig {
    pub addr: SocketAddr,
    pub id: String,
    #[serde(flatten)]
    pub credentials: Option<Credentials>,
    pub keepalive: Duration,
    pub capacity: usize,
}

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}
