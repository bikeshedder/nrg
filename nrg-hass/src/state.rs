use rumqttc::AsyncClient;
use serde::Serialize;

pub trait State {
    fn topic(&self) -> &str;
}

// FIXME this code should actually return a `Result<(), PublishError>` and one
// of the variants should be `MissingStateTopic`. The current implementation
// simply panics if the `state_topic` of the given configuration field is None.
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
