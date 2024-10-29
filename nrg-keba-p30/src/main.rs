use std::{
    fs,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use clap::Parser;
use config::Config;
use nrg_hass::{discovery::announce, state::publish_state};
use nrg_mqtt::{
    client::{CallbackSubscriber, MqttClient},
    command::{Commands, JsonDecoder},
};
use tokio::{sync::Mutex, time::sleep};
use tokio_modbus::{
    client::{tcp::connect_slave, Context},
    Slave,
};
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

use modbus::{read_register, write_register};
use registers::{
    ChargingState, ACTIVE_POWER, CABLE_STATE, CHARGING_STATE, ENABLE_CHARGING_STATION,
    MAX_SUPPORTED_CURRENT, SET_CHARGING_CURRENT, TOTAL_ENERGY,
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

struct State {
    hass: Hass,
    enabled: AtomicBool,
    context: Mutex<Context>,
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
    let ctx = Mutex::new(ctx);
    info!("Connected.");

    let mut hass = Hass::new(&cfg.hass);
    // Read max charging current from device
    let max_supported_current = read_register(&ctx, MAX_SUPPORTED_CURRENT).await?;
    hass.charging_current.max = Some(max_supported_current.into());
    info!("Max charging current = {}", max_supported_current);

    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass.charging_state).await?;
    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass.cable_state).await?;
    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass.active_power).await?;
    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass.total_energy).await?;
    announce(&mqtt, &cfg.hass, &cfg.hass.object_id, &hass.enabled).await?;
    announce(
        &mqtt,
        &cfg.hass,
        &cfg.hass.object_id,
        &hass.charging_current,
    )
    .await?;

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

    let state = Arc::new(State {
        context: ctx,
        enabled: AtomicBool::new(true),
        hass,
    });

    mqtt.sub(
        state.hass.enabled.state_topic.as_ref().unwrap(),
        CallbackSubscriber::new(state.clone(), |state, publish| {
            if publish.retain {
                let enabled = publish.payload.as_ref() != b"false";
                info!("state.enabled = {}", enabled);
                state.enabled.store(enabled, Ordering::Relaxed);
            }
        }),
    )
    .await?;

    tokio::spawn(process_commands(commands, state.clone()));

    // Sleep 1s to ensure the enabled state is refreshed from MQTT
    tokio::time::sleep(Duration::from_secs(1)).await;

    let mut previous_charging_state: Option<ChargingState> = None;

    loop {
        let charging_state = read_register(&state.context, CHARGING_STATE).await?;
        publish_state(&mqtt, &state.hass.charging_state, charging_state.as_ref()).await?;

        let cable_state = read_register(&state.context, CABLE_STATE).await?;
        publish_state(&mqtt, &state.hass.cable_state, cable_state.as_ref()).await?;

        // The max_charging_current lags behind the value set by set_charging_current
        // and becomes 0 when the charging is suspended. Therefore this information
        // is pretty much useless.
        //let max_charging_current = read_register(&ctx, MAX_CHARGING_CURRENT).await?;
        //publish_state(&mqtt, &hass.charging_current, max_charging_current).await?;

        let active_power = read_register(&state.context, ACTIVE_POWER).await?;
        publish_state(
            &mqtt,
            &state.hass.active_power,
            active_power as f64 / 1000.0,
        )
        .await?;

        let total_energy = read_register(&state.context, TOTAL_ENERGY).await?;
        publish_state(&mqtt, &state.hass.total_energy, total_energy as f64 / 10.0).await?;

        debug!(
            "{:.3} W, {:.3} kWh",
            active_power as f32 / 1000.0,
            total_energy as f32 / 10000.0
        );

        let enabled = state.enabled.load(Ordering::Relaxed);
        publish_state(&mqtt, &state.hass.enabled, enabled).await?;

        if charging_state == ChargingState::Active && !enabled {
            info!("Vehicle connected, charging paused");
            write_register(&state.context, ENABLE_CHARGING_STATION, false as u16)
                .await
                .unwrap()
        }

        if charging_state == ChargingState::Suspended && enabled {
            info!("Vehicle connected, charging resumed");
            write_register(&state.context, ENABLE_CHARGING_STATION, true as u16)
                .await
                .unwrap()
        }

        if charging_state == ChargingState::NotReady
            && previous_charging_state != Some(ChargingState::NotReady)
        {
            info!("Vehicle disconnected, charging enabled");
            write_register(&state.context, ENABLE_CHARGING_STATION, true as u16)
                .await
                .unwrap()
        }

        previous_charging_state = Some(charging_state);

        sleep(cfg.modbus.poll_delay).await;
    }
}

async fn process_commands(commands: Commands<Command>, state: Arc<State>) {
    loop {
        let Some(cmd) = commands.next().await else {
            break;
        };
        match cmd {
            Command::SetEnabled(enabled) => {
                state.enabled.store(enabled, Ordering::Relaxed);
                publish_state(commands.client(), &state.hass.enabled, &enabled)
                    .await
                    .unwrap();
            }
            Command::SetChargingCurrent(charging_current) => {
                publish_state(
                    commands.client(),
                    &state.hass.charging_current,
                    &charging_current,
                )
                .await
                .unwrap();
                write_register(&state.context, SET_CHARGING_CURRENT, charging_current)
                    .await
                    .unwrap();
            }
        }
    }
}
