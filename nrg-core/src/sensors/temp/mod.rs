use time::PrimitiveDateTime;

use crate::units::Temp;

pub mod ds18b20;

pub trait TempSensor {
    fn measurement(&self) -> Option<(PrimitiveDateTime, Temp)>;
}
