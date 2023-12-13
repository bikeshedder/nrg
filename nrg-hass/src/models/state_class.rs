use serde::Serialize;

/// https://developers.home-assistant.io/docs/core/entity/sensor/#available-state-classes
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StateClass {
    Measurement,
    Total,
    TotalIncreasing,
}
