use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[repr(u8)]
pub enum Qos {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce,
}
