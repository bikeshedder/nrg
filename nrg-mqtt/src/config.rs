use std::{net::SocketAddr, time::Duration};

use rumqttc::{AsyncClient, EventLoop, MqttOptions};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MqttConfig {
    pub addr: SocketAddr,
    pub id: String,
    #[serde(flatten)]
    pub credentials: Option<Credentials>,
    pub keepalive: Duration,
    pub capacity: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl MqttConfig {
    pub fn client(&self) -> (AsyncClient, EventLoop) {
        let mut options = MqttOptions::new(
            self.id.clone(),
            self.addr.ip().to_string(),
            self.addr.port(),
        );
        if let Some(cred) = &self.credentials {
            options.set_credentials(cred.username.clone(), cred.password.clone());
        }
        options.set_keep_alive(self.keepalive);
        rumqttc::AsyncClient::new(options, self.capacity)
    }
}
