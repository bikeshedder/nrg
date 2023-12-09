use std::sync::Arc;

pub mod waveshare_modbus_rtu_relay_16ch;

pub trait Actuator {
    fn channels(&self) -> &[Arc<dyn ActuatorChannel>];
}

pub trait ActuatorChannel {
    fn name(&self) -> &str;
    fn open(&self);
    fn close(&self);
    fn is_open(&self) -> bool;
}
