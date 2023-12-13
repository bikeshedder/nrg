use std::sync::Arc;

use nrg_hass::{
    config::HomeAssistantConfig,
    models::{
        device_class::DeviceClass,
        number::{Number, NumberMode},
        select::Select,
        sensor::Sensor,
        state_class::StateClass,
        unit::UnitOfMeasurement,
    },
};

use crate::registers::{ACTIVE_POWER, CHARGING_STATE, TOTAL_ENERGY};

pub struct Hass {
    pub charging_state: Sensor,
    pub active_power: Sensor,
    pub total_energy: Sensor,
    pub mode: Select,
    pub charging_current: Number,
}

impl Hass {
    pub fn new(cfg: &HomeAssistantConfig) -> Self {
        let device = Arc::new(
            nrg_hass::models::device::Device::builder()
                .configuration_url("http://192.168.178.40/")
                .identifiers(vec![cfg.object_id.clone()])
                .manufacturer("KEBA")
                .model("P30 X")
                .name(&cfg.name)
                // sw_version
                // via_device
                .build()
                .unwrap(),
        );

        let charging_state = Sensor::builder()
            .device(device.clone())
            .name(format!("{} Ladezustand", cfg.name))
            .object_id(format!("{}.{}", cfg.object_id, CHARGING_STATE.name))
            .state_topic(format!(
                "nrg/charging_station/{}/{}",
                cfg.object_id, CHARGING_STATE.name
            ))
            .unique_id(format!("{}.{}", cfg.object_id, CHARGING_STATE.name))
            .build()
            .unwrap();

        let active_power = Sensor::builder()
            .device(device.clone())
            .name(format!("{} Leistung", cfg.name))
            .object_id(format!("{}.{}", cfg.object_id, ACTIVE_POWER.name))
            .state_topic(format!(
                "nrg/charging_station/{}/{}",
                cfg.object_id, ACTIVE_POWER.name
            ))
            .device_class(DeviceClass::Energy)
            .unique_id(format!("{}.{}", cfg.object_id, ACTIVE_POWER.name))
            .unit_of_measurement(UnitOfMeasurement::Watt)
            .icon("mdi:ev-plug-type2")
            .build()
            .unwrap();

        let total_energy = Sensor::builder()
            .device(device.clone())
            .name(format!("{} Gesamtenergie", cfg.name))
            .object_id(format!("{}.{}", cfg.object_id, TOTAL_ENERGY.name))
            .state_topic(format!(
                "nrg/charging_station/{}/{}",
                cfg.object_id, TOTAL_ENERGY.name
            ))
            .device_class(DeviceClass::Energy)
            .state_class(StateClass::TotalIncreasing)
            .unique_id(format!("{}.{}", cfg.object_id, TOTAL_ENERGY.name))
            .unit_of_measurement(UnitOfMeasurement::WattHours)
            .icon("mdi:ev-plug-type2")
            .build()
            .unwrap();

        let mode = Select::builder()
            .device(device.clone())
            .name(format!("{} Mode", cfg.name))
            .object_id(format!("{}.{}", cfg.object_id, "mode"))
            .options(vec![
                "enabled".into(),
                "excess_only".into(),
                "excess_high".into(),
                "disabled".into(),
            ])
            .state_topic(format!("nrg/charging_station/{}/{}", cfg.object_id, "mode"))
            .command_template("{{ value }}")
            .command_topic(format!(
                "nrg/charging_station/{}/{}",
                cfg.object_id, "set_mode"
            ))
            .unique_id(format!("{}.{}", cfg.object_id, "mode"))
            .build()
            .unwrap();

        let charging_current = Number {
            command_topic: Some(format!(
                "nrg/charging_station/{}/set_charging_current",
                cfg.object_id
            )),
            device: Some(device.clone()),
            name: Some("Charging Current".into()),
            object_id: format!("{}.{}", cfg.object_id, "charging_current"),
            min: Some(6000.0),  // FIXME 6000
            max: Some(16000.0), // FIXME read from device
            mode: Some(NumberMode::Box),
            device_class: Some(DeviceClass::Power),
            step: Some(100.0),
            unit_of_measurement: Some(UnitOfMeasurement::Watt),
            ..Default::default()
        };

        Self {
            charging_state,
            active_power,
            total_energy,
            mode,
            charging_current,
        }
    }
}
