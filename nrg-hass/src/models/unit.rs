use serde::Serialize;

// https://github.com/home-assistant/core/blob/master/homeassistant/const.py#L384
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
    #[serde(rename = "mA")]
    MilliAmpere,
    #[serde(rename = "A")]
    Ampere,
}
