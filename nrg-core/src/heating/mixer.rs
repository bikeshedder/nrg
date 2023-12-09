use std::{sync::Arc, time::Duration};

use crate::{actuators::Actuator, sensors::temp::TempSensor, units::Temp};

pub struct Mixer {
    /// Temperature sensor for the water supply
    pub supply_sensor: Arc<dyn TempSensor>,
    /// Temperature sensor attached to the flow part
    pub flow_sensor: Arc<dyn TempSensor>,
    /// Temperature sensor attached to the return part
    pub return_sensor: Arc<dyn TempSensor>,
    /// Target temperature this mixer tries to reach
    pub target_temperature: Temp,
    /// Actuator for controlling the pump
    pub pump_actuator: Arc<dyn Actuator>,
    /// Actuator for opening the three way valve
    pub open_actuator: Arc<dyn Actuator>,
    /// Actuator for closing the three way valve
    pub close_actuator: Arc<dyn Actuator>,
    /// Time the three way valve takes to open and close
    pub travel_time: Duration,
    /// Target position for the valve
    pub target_position: f32,
    /// Current position of the valve
    pub current_position: f32,
}
