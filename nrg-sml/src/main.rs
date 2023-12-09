use std::fs;

use nrg_hass::{DeviceClass, StateClass, UnitOfMeasurement};
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

    let hass_wh = nrg_hass::Sensor {
        name: format!("{} Verbrauch", cfg.hass.name),
        object_id: Some(format!("{}.{}", cfg.hass.object_id, "wh")),
        state_topic: format!("nrg/energy-meter/{}/{}", cfg.hass.object_id, "wh"),
        unique_id: Some(format!("{}.{}", cfg.hass.object_id, "wh")),
        device_class: Some(DeviceClass::Energy),
        state_class: Some(StateClass::TotalIncreasing),
        unit_of_measurement: Some(UnitOfMeasurement::WattHours),
        icon: Some("mdi:transmission-tower-import".into()),
        ..Default::default()
    };

    let hass_wh_return = nrg_hass::Sensor {
        name: format!("{} Einspeisung", cfg.hass.name),
        object_id: Some(format!("{}.{}", cfg.hass.object_id, "wh_return")),
        state_topic: format!("nrg/energy-meter/{}/{}", cfg.hass.object_id, "wh_return"),
        unique_id: Some(format!("{}.{}", cfg.hass.object_id, "wh_return")),
        device_class: Some(DeviceClass::Energy),
        state_class: Some(StateClass::TotalIncreasing),
        unit_of_measurement: Some(UnitOfMeasurement::WattHours),
        icon: Some("mdi:transmission-tower-export".into()),
        ..Default::default()
    };

    let hass_w = nrg_hass::Sensor {
        name: format!("{} Leistung", cfg.hass.name),
        object_id: Some(format!("{}.{}", cfg.hass.object_id, "w")),
        state_topic: format!("nrg/energy-meter/{}/{}", cfg.hass.object_id, "w"),
        device_class: Some(nrg_hass::DeviceClass::Energy),
        unique_id: Some(format!("{}.{}", cfg.hass.object_id, "w")),
        unit_of_measurement: Some(nrg_hass::UnitOfMeasurement::Watt),
        icon: Some("mdi:home-lightning-bolt-outline".into()),
        ..Default::default()
    };

    hass_wh
        .register(&mqtt, &cfg.hass.discovery_prefix, &cfg.hass.object_id, "wh")
        .await?;
    hass_wh_return
        .register(
            &mqtt,
            &cfg.hass.discovery_prefix,
            &cfg.hass.object_id,
            "wh_return",
        )
        .await?;
    hass_w
        .register(&mqtt, &cfg.hass.discovery_prefix, &cfg.hass.object_id, "w")
        .await?;

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
                                w = Some(match val.value {
                                    Value::I32(wv) => wv.into(),
                                    Value::I64(wv) => wv,
                                    _ => panic!("Unsupported value {:?}", val.value),
                                });
                            }
                            [1, 8, 0] => {
                                let whv: i64 = match val.value {
                                    Value::I64(whv) => whv,
                                    Value::U64(whv) => whv.try_into().unwrap(),
                                    Value::I32(whv) => whv.into(),
                                    _ => panic!("Unsupported value {:?}", val.value),
                                };
                                wh =
                                    Some((whv as f64) * 10f64.powi(val.scaler.unwrap_or(0).into()));
                            }
                            [2, 8, 0] => {
                                let whv: i64 = match val.value {
                                    Value::I64(whv) => whv,
                                    Value::U64(whv) => whv.try_into().unwrap(),
                                    Value::I32(whv) => whv.into(),
                                    _ => panic!("Unsupported value {:?}", val.value),
                                };
                                wh_return =
                                    Some((whv as f64) * 10f64.powi(val.scaler.unwrap_or(0).into()));
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

                    hass_wh.value(&mqtt, format!("{:.1}", wh.unwrap())).await?;
                    hass_wh_return
                        .value(&mqtt, format!("{:.1}", wh_return.unwrap()))
                        .await?;
                    hass_w.value(&mqtt, w.unwrap().to_string()).await?;
                }
            }
            Err(e) => {
                println!("Unexpected error: {:?}", e);
            }
        }
    }
}
