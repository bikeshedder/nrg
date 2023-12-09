use time::PrimitiveDateTime;

use crate::units::Temp;

use super::TempSensor;

pub struct DS18B20 {}

impl TempSensor for DS18B20 {
    fn measurement(&self) -> Option<(PrimitiveDateTime, Temp)> {
        todo!()
    }
}
