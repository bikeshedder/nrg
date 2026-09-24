use rumqttc::AsyncClient;
use serde::Serialize;

use crate::config::HomeAssistantConfig;

#[derive(Serialize)]
struct DiscoveryPayload<'a, T: Discovery> {
    platform: &'static str,
    #[serde(flatten)]
    discovery: &'a T,
}

pub trait Discovery: Serialize {
    const COMPONENT: &'static str;
    fn unique_id(&self) -> &str;
    fn topic(&self, discovery_prefix: &str, node_id: &str) -> String {
        let component = Self::COMPONENT;
        let unique_id = self.unique_id();
        // https://www.home-assistant.io/integrations/mqtt#discovery-messages
        format!("{discovery_prefix}/{component}/{node_id}/{unique_id}/config")
    }
}

pub async fn announce<T: Discovery>(
    client: &AsyncClient,
    cfg: &HomeAssistantConfig,
    node_id: &str,
    discovery: &T,
) -> Result<(), rumqttc::ClientError> {
    let topic = discovery.topic(&cfg.discovery_prefix, node_id);
    let json = serde_json::to_string(&DiscoveryPayload {
        platform: T::COMPONENT,
        discovery,
    })
    .unwrap();
    client
        .publish(topic, rumqttc::QoS::AtLeastOnce, true, json)
        .await
}
