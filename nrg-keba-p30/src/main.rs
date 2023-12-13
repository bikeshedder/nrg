use std::{fs, path::PathBuf, sync::Arc};

use clap::Parser;
use config::Config;
use nrg_hass::{
    discovery::announce,
    models::{
        device::Device, device_class::DeviceClass, state_class::StateClass, unit::UnitOfMeasurement,
    },
    state::publish_state,
};
use nrg_mqtt::client::MqttClient;
use tokio::{sync::Mutex, time::sleep};
use tokio_modbus::{client::tcp::connect_slave, Slave};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use modbus::{read_register, write_register};
use registers::{
    ACTIVE_POWER, CHARGING_STATE, ENABLE_CHARGING_STATION, SET_CHARGING_CURRENT, TOTAL_ENERGY,
};

mod config;
mod modbus;
mod registers;

#[derive(Parser)]
struct Args {
    config_file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let args = Args::parse();

    let data = fs::read(args.config_file).expect("Could not read config.toml");
    let data = String::from_utf8(data).expect("Config file contains non-utf8 characters");
    let cfg: Config = toml::from_str(&data).expect("Error in config file");

    let mqtt = Arc::new(MqttClient::new(&cfg.mqtt));

    info!("Connecting to charging station {:?}...", cfg.modbus.addr);
    let ctx = connect_slave(cfg.modbus.addr, Slave(cfg.modbus.slave)).await?;
    let ctx = Arc::new(Mutex::new(ctx));
    info!("Connected.");

    let device = Arc::new(
        Device::builder()
            .configuration_url("http://192.168.178.40/")
            .identifiers(vec![cfg.hass.object_id.clone()])
            .manufacturer("KEBA")
            .model("P30 X")
            .name(&cfg.hass.name)
            // sw_version
            // via_device
            .build()
            .unwrap(),
    );

    let hass_charging_state = nrg_hass::models::sensor::Sensor::builder()
        .device(device.clone())
        .name(format!("{} Ladezustand", cfg.hass.name))
        .object_id(format!("{}.{}", cfg.hass.object_id, CHARGING_STATE.name))
        .state_topic(format!(
            "nrg/charging_station/{}/{}",
            cfg.hass.object_id, CHARGING_STATE.name
        ))
        .unique_id(format!("{}.{}", cfg.hass.object_id, CHARGING_STATE.name))
        .build()
        .unwrap();

    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass_charging_state).await?;

    let hass_active_power = nrg_hass::models::sensor::Sensor::builder()
        .device(device.clone())
        .name(format!("{} Leistung", cfg.hass.name))
        .object_id(format!("{}.{}", cfg.hass.object_id, ACTIVE_POWER.name))
        .state_topic(format!(
            "nrg/charging_station/{}/{}",
            cfg.hass.object_id, ACTIVE_POWER.name
        ))
        .device_class(DeviceClass::Energy)
        .unique_id(format!("{}.{}", cfg.hass.object_id, ACTIVE_POWER.name))
        .unit_of_measurement(UnitOfMeasurement::Watt)
        .icon("mdi:ev-plug-type2")
        .build()
        .unwrap();

    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass_active_power).await?;

    let hass_total_energy = nrg_hass::models::sensor::Sensor::builder()
        .device(device.clone())
        .name(format!("{} Gesamtenergie", cfg.hass.name))
        .object_id(format!("{}.{}", cfg.hass.object_id, TOTAL_ENERGY.name))
        .state_topic(format!(
            "nrg/charging_station/{}/{}",
            cfg.hass.object_id, TOTAL_ENERGY.name
        ))
        .device_class(DeviceClass::Energy)
        .state_class(StateClass::TotalIncreasing)
        .unique_id(format!("{}.{}", cfg.hass.object_id, TOTAL_ENERGY.name))
        .unit_of_measurement(UnitOfMeasurement::WattHours)
        .icon("mdi:ev-plug-type2")
        .build()
        .unwrap();

    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass_total_energy).await?;

    let hass_mode = nrg_hass::models::select::Select::builder()
        .device(device.clone())
        .name(format!("{} Mode", cfg.hass.name))
        .object_id(format!("{}.{}", cfg.hass.object_id, "mode"))
        .options(vec![
            "enabled".into(),
            "excess_only".into(),
            "excess_high".into(),
            "disabled".into(),
        ])
        .state_topic(format!(
            "nrg/charging_station/{}/{}",
            cfg.hass.object_id, "mode"
        ))
        .command_template("{{ value }}")
        .command_topic(format!(
            "nrg/charging_station/{}/{}",
            cfg.hass.object_id, "set_mode"
        ))
        .unique_id(format!("{}.{}", cfg.hass.object_id, "mode"))
        .build()
        .unwrap();

    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass_mode).await?;
    publish_state(&mqtt, &hass_mode, "disabled").await?;

    mqtt.subscribe(
        format!("nrg/charging_station/{}/set_mode", cfg.hass.object_id),
        {
            let mqtt = mqtt.clone();
            let ctx = ctx.clone();
            let hass_mode = hass_mode.clone();
            move |_, data| {
                let mqtt = mqtt.clone();
                let ctx = ctx.clone();
                let hass_mode = hass_mode.clone();
                let mode = String::from_utf8_lossy(data).to_string();
                println!("set_mode: {:?}", mode);
                tokio::spawn(async move {
                    publish_state(&mqtt, &hass_mode, &mode).await.unwrap();
                    match mode.as_str() {
                        "disabled" => write_register(&ctx, ENABLE_CHARGING_STATION, 0)
                            .await
                            .unwrap(),
                        "enabled" => write_register(&ctx, ENABLE_CHARGING_STATION, 1)
                            .await
                            .unwrap(),
                        _ => {}
                    }
                });
            }
        },
    )
    .await?;

    let hass_charging_current = nrg_hass::models::number::Number {
        command_topic: Some(format!(
            "nrg/charging_station/{}/set_charging_current",
            cfg.hass.object_id
        )),
        device: Some(device.clone()),
        name: Some("Charging Current".into()),
        object_id: format!("{}.{}", cfg.hass.object_id, "charging_current"),
        min: Some(6000.0),  // FIXME 6000
        max: Some(16000.0), // FIXME read from device
        mode: Some(nrg_hass::models::number::NumberMode::Box),
        device_class: Some(DeviceClass::Power),
        state_topic: Some(hass_active_power.state_topic.clone()),
        step: Some(100.0),
        unit_of_measurement: Some(UnitOfMeasurement::Watt),
        ..Default::default()
    };

    announce(
        &mqtt,
        &cfg.hass,
        &cfg.hass.object_id,
        &hass_charging_current,
    )
    .await?;

    mqtt.subscribe(
        hass_charging_current.command_topic.as_ref().unwrap(),
        |_, data| {
            println!("set_charging_current: {:?}", data);
        },
    )
    .await?;

    loop {
        let charging_state = read_register(&ctx, CHARGING_STATE).await?;
        publish_state(&mqtt, &hass_charging_state, charging_state.as_ref()).await?;

        let active_power = read_register(&ctx, ACTIVE_POWER).await?;
        publish_state(&mqtt, &hass_active_power, active_power).await?;

        let total_energy = read_register(&ctx, TOTAL_ENERGY).await?;
        publish_state(&mqtt, &hass_total_energy, total_energy as f64 / 10.0).await?;

        println!(
            "{:.3} W, {:.3} kWh",
            active_power as f32 / 1000.0,
            total_energy as f32 / 10000.0
        );

        sleep(cfg.modbus.poll_delay).await;
    }
}
