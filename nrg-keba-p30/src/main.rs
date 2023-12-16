use std::{fs, path::PathBuf, sync::Arc};

use clap::Parser;
use config::Config;
use nrg_hass::{discovery::announce, state::publish_state};
use nrg_mqtt::{
    client::MqttClient,
    command::{Commands, JsonDecoder},
};
use tokio::{sync::Mutex, time::sleep};
use tokio_modbus::{
    client::{tcp::connect_slave, Context},
    Slave,
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use modbus::{read_register, write_register};
use registers::{
    ACTIVE_POWER, CHARGING_STATE, ENABLE_CHARGING_STATION, MAX_SUPPORTED_CURRENT,
    SET_CHARGING_CURRENT, TOTAL_ENERGY,
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

enum Command {
    SetEnabled(bool),
    SetChargingCurrent(u16),
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

    let mut hass = Hass::new(&cfg.hass);
    // Read max charging current from device
    let max_supported_current = read_register(&ctx, MAX_SUPPORTED_CURRENT).await?;
    hass.charging_current.max = Some(max_supported_current.try_into().unwrap());
    info!("Max charging current = {}", max_supported_current);

    let hass = Arc::new(hass);

    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass.charging_state).await?;
    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass.active_power).await?;
    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass.total_energy).await?;
    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass.enabled).await?;

    let commands = Commands::new(mqtt.clone());
    commands
        .cmd(
            &hass.enabled.command_topic,
            JsonDecoder(Command::SetEnabled),
        )
        .await?;
    commands
        .cmd(
            hass.charging_current.command_topic.as_ref().unwrap(),
            JsonDecoder(Command::SetChargingCurrent),
        )
        .await?;

    tokio::spawn(process_commands(commands, hass.clone(), ctx.clone()));

    announce(
        &mqtt,
        &cfg.hass,
        &cfg.hass.object_id,
        &hass.charging_current,
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

async fn process_commands(commands: Commands<Command>, hass: Arc<Hass>, ctx: Arc<Mutex<Context>>) {
    loop {
        let Some(cmd) = commands.next().await else {
            break;
        };
        match cmd {
            Command::SetEnabled(enabled) => {
                publish_state(commands.client(), &hass.enabled, &enabled)
                    .await
                    .unwrap();
                write_register(&ctx, ENABLE_CHARGING_STATION, enabled as u16)
                    .await
                    .unwrap()
            }
            Command::SetChargingCurrent(charging_current) => {
                publish_state(commands.client(), &hass.charging_current, &charging_current)
                    .await
                    .unwrap();
                write_register(&ctx, SET_CHARGING_CURRENT, charging_current)
                    .await
                    .unwrap();
            }
        }
    }
}
