use std::{collections::HashMap, fs};

use config::Config;
use rumqttc::{Packet, QoS, SubscribeFilter};
use tokio_postgres::Statement;

mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg_data = fs::read("nrg-timescaledb.toml")?;
    let cfg_str = String::from_utf8(cfg_data)?;
    let cfg: Config = toml::from_str(&cfg_str)?;

    let db = cfg.pg.connect().await?;
    db.simple_query("CREATE EXTENSION IF NOT EXISTS timescaledb;")
        .await?;

    let (mqtt_client, mut mqtt_eventloop) = cfg.mqtt.client();

    let mut mqtt_to_stmt: HashMap<String, Statement> = HashMap::with_capacity(cfg.series.len());
    for (name, series) in &cfg.series {
        db.simple_query(&format!("CREATE TABLE IF NOT EXISTS {} (created TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL, value DOUBLE PRECISION NOT NULL)", name)).await?;
        db.query(
            "SELECT create_hypertable($1::text::regclass, by_range('created'), if_not_exists => true);",
            &[&name],
        )
        .await?;
        if series.aggregate.day {
            db.simple_query(&format!(
                r#"
                CREATE MATERIALIZED VIEW IF NOT EXISTS {}_day_by_day(time, value)
                with (timescaledb.continuous, timescaledb.materialized_only=false) as
                SELECT time_bucket('1 day', created, 'Europe/Berlin') AS "time",
                round((last(value, created) - first(value, created)) * 100.) / 100. AS value
                FROM {}
                GROUP BY 1
                "#,
                name, name
            ))
            .await?;
            db.simple_query(&format!(
                r#"
                SELECT add_continuous_aggregate_policy('{}_day_by_day',
                start_offset => NULL,
                end_offset => INTERVAL '1 hour',
                schedule_interval => INTERVAL '1 hour',
                if_not_exists => true);
                "#,
                name
            ))
            .await?;
        }
        if series.aggregate.hour {
            db.simple_query(&format!(
                r#"
                CREATE MATERIALIZED VIEW IF NOT EXISTS {}_hour_by_hour(time, value)
                with (timescaledb.continuous) as
                SELECT time_bucket('01:00:00', created, 'Europe/Berlin') AS "time",
                    round((last(value, created) - first(value, created)) * 100.) / 100. AS value
                FROM {}
                GROUP BY 1
                "#,
                name, name
            ))
            .await?;
            db.simple_query(&format!(
                r#"
                SELECT add_continuous_aggregate_policy('{}_hour_by_hour',
                start_offset => NULL,
                end_offset => INTERVAL '1 hour',
                schedule_interval => INTERVAL '1 hour',
                if_not_exists => true);
                "#,
                name
            ))
            .await?;
        }
        mqtt_to_stmt.insert(
            series.topic.clone(),
            db.prepare(&format!("INSERT INTO {} (value) VALUES ($1)", name))
                .await?,
        );
    }

    mqtt_client
        .subscribe_many(
            cfg.series
                .values()
                .map(|s| SubscribeFilter::new(s.topic.clone(), QoS::AtMostOnce)),
        )
        .await?;

    loop {
        let event = mqtt_eventloop.poll().await?;
        let rumqttc::Event::Incoming(Packet::Publish(publish)) = event else {
            continue;
        };
        let Some(stmt) = mqtt_to_stmt.get(&publish.topic) else {
            continue;
        };
        let payload = String::from_utf8(publish.payload.to_vec())?;
        let value = payload.parse::<f64>()?;
        println!("{} {}", publish.topic, value);
        db.execute(stmt, &[&value]).await?;
    }
}
