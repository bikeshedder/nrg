use std::{fs, path::PathBuf};

use nrg_hass::{
    models::{
        device_class::DeviceClass, sensor::Sensor, state_class::StateClass, unit::UnitOfMeasurement,
    },
    state::publish_state,
};
use nrg_mqtt::client::MqttClient;
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let config_file = "nrg-ds18b20.toml";

    let data = fs::read(config_file).expect(&format!("Could not read {config_file}"));
    let data = String::from_utf8(data).expect("Config file contains non-utf8 characters");
    let cfg: Config = toml::from_str(&data).expect("Error in config file");

    let mqtt = MqttClient::new(&cfg.mqtt);

    let devices_path = PathBuf::from("/sys/bus/w1/devices");
    let master_path = devices_path.join(cfg.w1.master);
    let therm_bulk_read_path = master_path.join("therm_bulk_read");

    let sensors = cfg
        .sensors
        .iter()
        .map(|sensor_config| {
            let serial: String = sensor_config
                .serial
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect();
            DS18B20 {
                name: sensor_config.name.clone(),
                temperature_path: devices_path
                    .join(format!("28-{serial}"))
                    .join("temperature"),
                serial: serial.clone(),
                hass_sensor: Sensor::builder()
                    .name(sensor_config.name.clone())
                    .object_id(format!("nrg_ds18b20_{serial}"))
                    .device_class(DeviceClass::Temperature)
                    .state_class(StateClass::Measurement)
                    .state_topic(format!("{}{}", cfg.mqtt.topic_prefix, serial))
                    .unique_id(format!("nrg_ds18b20_{serial}"))
                    .unit_of_measurement(UnitOfMeasurement::TempCelsius)
                    .build()
                    .unwrap(),
            }
        })
        .collect::<Vec<_>>();

    for sensor in &sensors {
        nrg_hass::discovery::announce(
            &mqtt,
            &cfg.hass,
            &sensor.hass_sensor.object_id,
            &sensor.hass_sensor,
        )
        .await?;
    }

    loop {
        debug!("Writing 'trigger' to {therm_bulk_read_path:?}");
        tokio::fs::write(&therm_bulk_read_path, "trigger\n").await?;
        for sensor in &sensors {
            debug!("Reading {:?}", sensor.temperature_path);
            let temp_str = tokio::fs::read_to_string(&sensor.temperature_path).await?;
            debug!("Got: {:?}", temp_str);
            let temp = i32::from_str_radix(temp_str.trim_end(), 10)? as f32 / 1000.0;
            info!("[{}] {:30} -> {:.3}", sensor.serial, sensor.name, temp);
            publish_state(&mqtt, &sensor.hass_sensor, temp).await?;
        }
        tokio::time::sleep(cfg.w1.interval).await;
    }
}

struct DS18B20 {
    name: String,
    serial: String,
    temperature_path: PathBuf,
    hass_sensor: nrg_hass::models::sensor::Sensor,
}
