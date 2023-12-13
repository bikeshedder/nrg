use rumqttc::AsyncClient;
use serde::Serialize;

pub trait State {
    fn topic(&self) -> &str;
}

pub async fn publish_state<T, E>(
    client: &AsyncClient,
    entity: &E,
    payload: T,
) -> Result<(), rumqttc::ClientError>
where
    T: Serialize,
    E: State,
{
    let json = serde_json::to_string(&payload).unwrap();
    client
        .publish(entity.topic(), rumqttc::QoS::AtLeastOnce, true, json)
        .await
}
