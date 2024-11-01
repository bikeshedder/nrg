use std::{error::Error, fs, time::Duration};

use nrg_mqtt::client::MqttClient;
use rumqttc::QoS;
use sunspec::{
    client::AsyncClient,
    models::{model1::Model1, model103::Model103},
};
use tokio::time::sleep;
use tokio_modbus::{client::tcp::connect_slave, Slave};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::config::Config;

pub mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let data = fs::read("nrg-sunspec.toml").expect("Could not read config.toml");
    let data = String::from_utf8(data).expect("Config file contains non-utf8 characters");
    let cfg: Config = toml::from_str(&data).expect("Error in config file");

    let mut client = AsyncClient::new(
        connect_slave(cfg.modbus.addr, Slave(cfg.modbus.slave)).await?,
        sunspec::client::Config::default(),
    )
    .await?;
    let m1: Model1 = client.read_model().await?;

    println!("Manufacturer: {}", m1.mn);
    println!("Model: {}", m1.md);
    println!("Version: {}", m1.vr.as_deref().unwrap_or("(unspecified)"));
    println!("Serial Number: {}", m1.sn);
    println!(
        "Supported models: {:?}",
        client.models.supported_model_ids()
    );

    let mqtt = MqttClient::new(&cfg.mqtt);

    loop {
        let m103: Model103 = client.read_model().await?;
        let w = m103.w as f32 * 10f32.powf(m103.w_sf.into());
        let wh = m103.wh as f32 * 10f32.powf(m103.wh_sf.into());

        println!("{:12.3} kWh {:9.3} kW", wh / 1000.0, w / 1000.0,);

        mqtt.publish(
            "nrg/solar-inverter/wh",
            QoS::AtLeastOnce,
            true,
            wh.to_string(),
        )
        .await?;

        mqtt.publish(
            "nrg/solar-inverter/w",
            QoS::AtLeastOnce,
            true,
            format!("{:.1}", w),
        )
        .await?;

        sleep(Duration::from_secs(5)).await;
    }
}
