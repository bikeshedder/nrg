use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum UnitOfMeasurement {
    #[serde(rename = "°C")]
    TempCelsius,
    #[serde(rename = "mW")]
    MilliWatt,
    #[serde(rename = "W")]
    Watt,
    #[serde(rename = "Wh")]
    WattHours,
}
