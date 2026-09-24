use std::fs;

use nrg_hass::{
    discovery::announce,
    models::{device_class::DeviceClass, state_class::StateClass, unit::UnitOfMeasurement},
    state::publish_state,
};
use nrg_mqtt::client::MqttClient;
use sml_rs::{
    parser::{common::Value, complete},
    transport::Decoder,
    util::ArrayBuf,
};
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};
use tokio_serial::SerialStream;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use config::{Config, SerialConfig};

pub mod config;

fn value_to_i64(value: &Value) -> Option<i64> {
    match value {
        Value::I8(v) => Some((*v).into()),
        Value::I16(v) => Some((*v).into()),
        Value::I32(v) => Some((*v).into()),
        Value::I64(v) => Some(*v),
        Value::U8(v) => Some((*v).into()),
        Value::U16(v) => Some((*v).into()),
        Value::U32(v) => Some((*v).into()),
        Value::U64(v) => (*v).try_into().ok(),
        _ => None,
    }
}

pub(crate) fn uart_ir_sensor_data_stream(config: SerialConfig) -> impl AsyncRead {
    let ttys_location = config.device;
    let serial = tokio_serial::new(ttys_location, config.baud);
    SerialStream::open(&serial).unwrap()
}

#[tokio::main(worker_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let data = fs::read("nrg-sml.toml").expect("Could not read config.toml");
    let data = String::from_utf8(data).expect("Config file contains non-utf8 characters");
    let cfg: Config = toml::from_str(&data).expect("Error in config file");

    let mqtt = MqttClient::new(&cfg.mqtt);

    let hass_wh = nrg_hass::models::sensor::Sensor::builder()
        .name(format!("{} Verbrauch", cfg.hass.name))
        .default_entity_id(format!("{}_{}", cfg.hass.object_id, "wh"))
        .state_topic(format!("nrg/energy-meter/{}/{}", cfg.hass.object_id, "wh"))
        .unique_id(format!("{}.{}", cfg.hass.object_id, "wh"))
        .device_class(DeviceClass::Energy)
        .state_class(StateClass::TotalIncreasing)
        .unit_of_measurement(UnitOfMeasurement::WattHours)
        .icon("mdi:transmission-tower-import")
        .build()
        .unwrap();

    let hass_wh_return = nrg_hass::models::sensor::Sensor::builder()
        .name(format!("{} Einspeisung", cfg.hass.name))
        .default_entity_id(format!("{}_{}", cfg.hass.object_id, "wh_return"))
        .state_topic(format!(
            "nrg/energy-meter/{}/{}",
            cfg.hass.object_id, "wh_return"
        ))
        .unique_id(format!("{}.{}", cfg.hass.object_id, "wh_return"))
        .device_class(DeviceClass::Energy)
        .state_class(StateClass::TotalIncreasing)
        .unit_of_measurement(UnitOfMeasurement::WattHours)
        .icon("mdi:transmission-tower-export")
        .build()
        .unwrap();

    let hass_w = nrg_hass::models::sensor::Sensor::builder()
        .name(format!("{} Leistung", cfg.hass.name))
        .default_entity_id(format!("{}_{}", cfg.hass.object_id, "w"))
        .state_topic(format!("nrg/energy-meter/{}/{}", cfg.hass.object_id, "w"))
        .device_class(DeviceClass::Power)
        .unique_id(format!("{}.{}", cfg.hass.object_id, "w"))
        .unit_of_measurement(UnitOfMeasurement::Watt)
        .icon("mdi:home-lightning-bolt-outline")
        .build()
        .unwrap();

    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass_wh).await?;
    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass_wh_return).await?;
    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass_w).await?;

    let uart = uart_ir_sensor_data_stream(cfg.serial);
    let mut reader = BufReader::new(uart);
    let mut decoder = Decoder::<ArrayBuf<2048>>::new();

    loop {
        let byte = reader.read_u8().await?;
        match decoder.push_byte(byte) {
            Ok(None) => {}
            Ok(Some(decoded)) => {
                let Ok(file) = complete::parse(decoded) else {
                    println!("Parsing failed");
                    continue;
                };
                for m in file.messages {
                    let complete::MessageBody::GetListResponse(lst) = m.message_body else {
                        continue;
                    };
                    let mut w: Option<i64> = None;
                    let mut wh: Option<f64> = None;
                    let mut wh_return: Option<f64> = None;
                    for val in &lst.val_list {
                        match val.obj_name[2..2 + 3] {
                            [16, 7, 0] => {
                                match value_to_i64(&val.value) {
                                    Some(wv) => w = Some(wv),
                                    None => {
                                        eprintln!(
                                            "Unsupported power value {:?} for OBIS {:?}",
                                            val.value, val.obj_name
                                        );
                                    }
                                }
                            }
                            [1, 8, 0] => {
                                match value_to_i64(&val.value) {
                                    Some(whv) => {
                                        wh = Some(
                                            (whv as f64)
                                                * 10f64.powi(val.scaler.unwrap_or(0).into()),
                                        );
                                    }
                                    None => {
                                        eprintln!(
                                            "Unsupported import energy value {:?} for OBIS {:?}",
                                            val.value, val.obj_name
                                        );
                                    }
                                }
                            }
                            [2, 8, 0] => {
                                match value_to_i64(&val.value) {
                                    Some(whv) => {
                                        wh_return = Some(
                                            (whv as f64)
                                                * 10f64.powi(val.scaler.unwrap_or(0).into()),
                                        );
                                    }
                                    None => {
                                        eprintln!(
                                            "Unsupported export energy value {:?} for OBIS {:?}",
                                            val.value, val.obj_name
                                        );
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    println!(
                        "[<- {:.4?} kWh] [-> {:.4?} kWh] [{} W]",
                        wh.unwrap_or(0f64) / 1000f64,
                        wh_return.unwrap_or(0f64) / 1000f64,
                        w.unwrap_or(0)
                    );

                    if let Some(wh) = wh {
                        publish_state(&mqtt, &hass_wh, wh).await.unwrap();
                    }
                    if let Some(wh_return) = wh_return {
                        publish_state(&mqtt, &hass_wh_return, wh_return)
                            .await
                            .unwrap();
                    }
                    if let Some(w) = w {
                        publish_state(&mqtt, &hass_w, w).await.unwrap();
                    }
                }
            }
            Err(e) => {
                println!("Unexpected error: {:?}", e);
            }
        }
    }
}
