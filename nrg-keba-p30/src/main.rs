use std::{fs, sync::Arc};

use config::Config;
use nrg_hass::{Device, DeviceClass, StateClass};
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let data = fs::read("keba-p30.toml").expect("Could not read config.toml");
    let data = String::from_utf8(data).expect("Config file contains non-utf8 characters");
    let cfg: Config = toml::from_str(&data).expect("Error in config file");

    let mqtt = Arc::new(MqttClient::new(&cfg.mqtt));

    info!("Connecting to charging station {:?}...", cfg.modbus.addr);
    let ctx = connect_slave(cfg.modbus.addr, Slave(cfg.modbus.slave)).await?;
    let ctx = Arc::new(Mutex::new(ctx));
    info!("Connected.");

    let device = Some(Arc::new(Device {
        configuration_url: Some("http://192.168.178.40/".into()),
        identifiers: Some(vec![cfg.hass.object_id.clone()]),
        manufacturer: Some("KEBA".into()),
        model: Some("P30 X".into()),
        name: Some(cfg.hass.name.clone()),
        // sw_version
        // via_device
        ..Default::default()
    }));

    let hass_charging_state = nrg_hass::Sensor {
        device: device.clone(),
        name: format!("{} Ladezustand", cfg.hass.name),
        object_id: Some(format!("{}.{}", cfg.hass.object_id, CHARGING_STATE.name)),
        state_topic: format!(
            "nrg/charging_station/{}/{}",
            cfg.hass.object_id, CHARGING_STATE.name
        ),
        unique_id: Some(format!("{}.{}", cfg.hass.object_id, CHARGING_STATE.name)),
        ..Default::default()
    };

    hass_charging_state
        .register(
            &mqtt,
            &cfg.hass.discovery_prefix,
            &cfg.hass.object_id,
            CHARGING_STATE.name,
        )
        .await?;

    let hass_active_power = nrg_hass::Sensor {
        device: device.clone(),
        name: format!("{} Leistung", cfg.hass.name),
        object_id: Some(format!("{}.{}", cfg.hass.object_id, ACTIVE_POWER.name)),
        state_topic: format!(
            "nrg/charging_station/{}/{}",
            cfg.hass.object_id, ACTIVE_POWER.name
        ),
        device_class: Some(nrg_hass::DeviceClass::Energy),
        unique_id: Some(format!("{}.{}", cfg.hass.object_id, ACTIVE_POWER.name)),
        unit_of_measurement: Some(nrg_hass::UnitOfMeasurement::Watt),
        icon: Some("mdi:ev-plug-type2".into()),
        ..Default::default()
    };

    hass_active_power
        .register(
            &mqtt,
            &cfg.hass.discovery_prefix,
            &cfg.hass.object_id,
            ACTIVE_POWER.name,
        )
        .await?;

    let hass_total_energy = nrg_hass::Sensor {
        device: device.clone(),
        name: format!("{} Gesamtenergie", cfg.hass.name),
        object_id: Some(format!("{}.{}", cfg.hass.object_id, TOTAL_ENERGY.name)),
        state_topic: format!(
            "nrg/charging_station/{}/{}",
            cfg.hass.object_id, TOTAL_ENERGY.name
        ),
        device_class: Some(nrg_hass::DeviceClass::Energy),
        state_class: Some(StateClass::TotalIncreasing),
        unique_id: Some(format!("{}.{}", cfg.hass.object_id, TOTAL_ENERGY.name)),
        unit_of_measurement: Some(nrg_hass::UnitOfMeasurement::WattHours),
        icon: Some("mdi:ev-plug-type2".into()),
        ..Default::default()
    };

    hass_total_energy
        .register(
            &mqtt,
            &cfg.hass.discovery_prefix,
            &cfg.hass.object_id,
            TOTAL_ENERGY.name,
        )
        .await?;

    let hass_mode = Arc::new(nrg_hass::select::Select {
        device: device.clone(),
        name: Some(format!("{} Mode", cfg.hass.name)),
        object_id: Some(format!("{}.{}", cfg.hass.object_id, "mode")),
        options: vec![
            "enabled".into(),
            "excess_only".into(),
            "excess_high".into(),
            "disabled".into(),
        ],
        state_topic: Some(format!(
            "nrg/charging_station/{}/{}",
            cfg.hass.object_id, "mode"
        )),
        command_template: Some("{{ value }}".into()),
        command_topic: format!("nrg/charging_station/{}/{}", cfg.hass.object_id, "set_mode"),
        unique_id: Some(format!("{}.{}", cfg.hass.object_id, "mode")),
        //value_template: Some("X".into()),
        ..Default::default()
    });

    hass_mode
        .register(
            &mqtt,
            &cfg.hass.discovery_prefix,
            &cfg.hass.object_id,
            "mode",
        )
        .await?;
    hass_mode.value(&mqtt, "disabled").await?;

    mqtt.subscribe(
        format!("nrg/charging_station/{}/set_mode", cfg.hass.object_id),
        {
            let mqtt = mqtt.clone();
            let ctx = ctx.clone();
            let hass_mode = hass_mode.clone();
            move |data| {
                let mqtt = mqtt.clone();
                let ctx = ctx.clone();
                let hass_mode = hass_mode.clone();
                let mode = String::from_utf8_lossy(data).to_string();
                println!("set_mode: {:?}", mode);
                tokio::spawn(async move {
                    hass_mode.value(&mqtt, &mode).await.unwrap();
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

    let hass_charging_current = Arc::new(nrg_hass::number::Number {
        command_topic: Some(format!(
            "nrg/charging_station/{}/set_charging_current",
            cfg.hass.object_id
        )),
        device: device.clone(),
        name: Some("Charging Current".into()),
        object_id: Some(format!("{}.{}", cfg.hass.object_id, "charging_current")),
        min: Some(6000.0),  // FIXME 6000
        max: Some(16000.0), // FIXME read from device
        mode: Some(nrg_hass::number::NumberMode::Box),
        device_class: Some(DeviceClass::Power),
        state_topic: Some(hass_active_power.state_topic.clone()),
        step: Some(100.0),
        unit_of_measurement: Some(nrg_hass::UnitOfMeasurement::Watt),
        ..Default::default()
    });
    hass_charging_current
        .register(
            &mqtt,
            &cfg.hass.discovery_prefix,
            &cfg.hass.object_id,
            "charging_current",
        )
        .await?;

    mqtt.subscribe(
        hass_charging_current.command_topic.as_ref().unwrap(),
        |data| {
            println!("set_charging_current: {:?}", data);
        },
    )
    .await?;

    loop {
        let charging_state = read_register(&ctx, CHARGING_STATE).await?;
        hass_charging_state
            .value(&mqtt, charging_state.as_ref())
            .await?;

        let active_power = read_register(&ctx, ACTIVE_POWER).await?;
        hass_active_power
            .value(&mqtt, format!("{:.3}", active_power as f64 / 1000.0))
            .await?;

        let total_energy = read_register(&ctx, TOTAL_ENERGY).await?;
        hass_total_energy
            .value(&mqtt, format!("{:.1}", total_energy as f64 / 10.0))
            .await?;

        println!(
            "{:.3} W, {:.3} kWh",
            active_power as f32 / 1000.0,
            total_energy as f32 / 10000.0
        );

        sleep(cfg.modbus.poll_delay).await;
    }
}
