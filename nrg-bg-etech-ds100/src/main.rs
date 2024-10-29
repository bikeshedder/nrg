use std::{fs, time::Duration};

use anyhow::Result;
use config::Config;
use nrg_hass::{
    discovery::announce,
    models::{device_class::DeviceClass, state_class::StateClass, unit::UnitOfMeasurement},
    state::publish_state,
};
use nrg_mqtt::client::MqttClient;
use tokio::time::sleep;
use tokio_modbus::{
    client::{Context, Reader},
    Slave,
};
use tokio_serial::SerialStream;

mod config;

#[tokio::main]
async fn main() -> Result<()> {
    let data = fs::read("nrg-bg-etech-ds100.toml").expect("Could not read config.toml");
    let data = String::from_utf8(data).expect("Config file contains non-utf8 characters");
    let cfg: Config = toml::from_str(&data).expect("Error in config file");

    let builder = tokio_serial::new(cfg.modbus.device, cfg.modbus.baud);
    let stream = SerialStream::open(&builder)?;
    let mut ctx = tokio_modbus::client::rtu::attach_slave(stream, Slave(cfg.modbus.slave));

    let mqtt = MqttClient::new(&cfg.mqtt);

    let hass_wh = nrg_hass::models::sensor::Sensor::builder()
        .name(format!("{} Energie", cfg.hass.name))
        .object_id(format!("{}_{}", cfg.hass.object_id, "wh"))
        .state_topic(format!("nrg/energy-meter/{}/{}", cfg.hass.object_id, "wh"))
        .unique_id(format!("{}_{}", cfg.hass.object_id, "wh"))
        .device_class(DeviceClass::Energy)
        .state_class(StateClass::TotalIncreasing)
        .unit_of_measurement(UnitOfMeasurement::WattHours)
        .icon("mdi:heat-pump-outline")
        .build()
        .unwrap();

    let hass_w = nrg_hass::models::sensor::Sensor::builder()
        .name(format!("{} Leistung", cfg.hass.name))
        .object_id(format!("{}_{}", cfg.hass.object_id, "w"))
        .state_topic(format!("nrg/energy-meter/{}/{}", cfg.hass.object_id, "w"))
        .device_class(DeviceClass::Energy)
        .unique_id(format!("{}_{}", cfg.hass.object_id, "w"))
        .unit_of_measurement(UnitOfMeasurement::Watt)
        .icon("mdi:home-lightning-bolt-outline")
        .build()
        .unwrap();

    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass_wh).await?;
    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass_w).await?;

    loop {
        let w = read_u32(&mut ctx, 0x420).await?;
        let wh = read_u32(&mut ctx, 0x010E).await? * 10;
        println!("{:4} W  {:6} Wh", w, wh);
        publish_state(&mqtt, &hass_w, w).await?;
        publish_state(&mqtt, &hass_wh, wh).await?;
        sleep(Duration::from_millis(500)).await;
    }
}

async fn read_u32(ctx: &mut Context, addr: u16) -> Result<u32> {
    let data: [u16; 2] = ctx
        .read_holding_registers(addr, 2)
        .await??
        .try_into()
        .expect("read_holding_registers returned the wrong amount of registers");
    Ok(((data[0] as u32) << 16) + (data[1] as u32))
}
