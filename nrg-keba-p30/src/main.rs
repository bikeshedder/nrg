use std::{fs, path::PathBuf, sync::Arc};

use clap::Parser;
use config::Config;
use nrg_hass::{discovery::announce, state::publish_state};
use nrg_mqtt::client::MqttClient;
use tokio::{sync::Mutex, time::sleep};
use tokio_modbus::{client::tcp::connect_slave, Slave};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use modbus::{read_register, write_register};
use registers::{
    ACTIVE_POWER, CHARGING_STATE, ENABLE_CHARGING_STATION, SET_CHARGING_CURRENT, TOTAL_ENERGY,
};

use crate::hass::Hass;

mod config;
mod hass;
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

    let hass = Hass::new(&cfg.hass);

    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass.charging_state).await?;
    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass.active_power).await?;
    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass.total_energy).await?;
    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass.mode).await?;

    publish_state(&mqtt, &hass.mode, "disabled").await?;

    mqtt.subscribe(
        format!("nrg/charging_station/{}/set_mode", cfg.hass.object_id),
        {
            let mqtt = mqtt.clone();
            let ctx = ctx.clone();
            let hass_mode = hass.mode.clone();
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

    announce(
        &mqtt,
        &cfg.hass,
        &cfg.hass.object_id,
        &hass.charging_current,
    )
    .await?;

    mqtt.subscribe(
        hass.charging_current.command_topic.as_ref().unwrap(),
        |_, data| {
            println!("set_charging_current: {:?}", data);
        },
    )
    .await?;

    loop {
        let charging_state = read_register(&ctx, CHARGING_STATE).await?;
        publish_state(&mqtt, &hass.charging_state, charging_state.as_ref()).await?;

        let active_power = read_register(&ctx, ACTIVE_POWER).await?;
        publish_state(&mqtt, &hass.active_power, active_power).await?;

        let total_energy = read_register(&ctx, TOTAL_ENERGY).await?;
        publish_state(&mqtt, &hass.total_energy, total_energy as f64 / 10.0).await?;

        println!(
            "{:.3} W, {:.3} kWh",
            active_power as f32 / 1000.0,
            total_energy as f32 / 10000.0
        );

        sleep(cfg.modbus.poll_delay).await;
    }
}
