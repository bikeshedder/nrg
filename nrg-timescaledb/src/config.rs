use std::{collections::HashMap, str::FromStr};

use nrg_mqtt::config::MqttConfig;
use serde::Deserialize;
use tokio_postgres::{Client, Config as PgConfig, Error as PgError, NoTls};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(rename = "postgresql")]
    pub pg: PostgresqlConfig,
    pub mqtt: MqttConfig,
    pub series: HashMap<String, Series>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PostgresqlConfig {
    pub url: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub dbname: Option<String>,
}

impl PostgresqlConfig {
    pub fn pg_config(&self) -> Result<PgConfig, PgError> {
        let mut cfg = if let Some(url) = &self.url {
            tokio_postgres::Config::from_str(url)?
        } else {
            tokio_postgres::Config::new()
        };
        if let Some(host) = &self.host {
            cfg.host(host);
        } else {
            cfg.host_path("/var/run/postgresql");
        }
        if let Some(port) = self.port {
            cfg.port(port);
        }
        if let Some(user) = &self.user {
            cfg.user(user);
        }
        if let Some(password) = &self.password {
            cfg.password(password);
        }
        if let Some(dbname) = &self.dbname {
            cfg.dbname(dbname);
        }
        Ok(cfg)
    }
    pub async fn connect(&self) -> Result<Client, PgError> {
        let (client, connection) = self.pg_config()?.connect(NoTls).await?;
        tokio::spawn(connection);
        Ok(client)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Series {
    pub topic: String,
    #[serde(default)]
    pub aggregate: Aggregate,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Aggregate {
    #[serde(default)]
    pub day: bool,
    #[serde(default)]
    pub hour: bool,
}
