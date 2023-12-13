use rumqttc::AsyncClient;
use serde::Serialize;

use crate::config::HomeAssistantConfig;

pub trait Discovery: Serialize {
    const COMPONENT: &'static str;
    fn object_id(&self) -> &str;
    fn topic(&self, discovery_prefix: &str, node_id: &str) -> String {
        let component = Self::COMPONENT;
        let object_id = self.object_id();
        // https://www.home-assistant.io/integrations/mqtt#discovery-messages
        format!("{discovery_prefix}/{component}/{node_id}/{object_id}/config")
    }
}

pub async fn announce(
    client: &AsyncClient,
    cfg: &HomeAssistantConfig,
    node_id: &str,
    discovery: &impl Discovery,
) -> Result<(), rumqttc::ClientError> {
    let topic = discovery.topic(&cfg.discovery_prefix, node_id);
    let json = serde_json::to_string(&discovery).unwrap();
    client
        .publish(topic, rumqttc::QoS::AtLeastOnce, true, json)
        .await
}
