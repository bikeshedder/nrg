use serde::Serialize;

/// https://developers.home-assistant.io/docs/core/entity/#generic-properties
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityCategory {
    Config,
    Diagnostic,
}
