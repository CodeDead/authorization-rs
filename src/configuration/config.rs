use config::ConfigError;
use mongodb::{Client, Database};
use serde::Deserialize;

use super::jwt::JWT;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
}

#[derive(Deserialize)]
pub struct MongoDB {
    pub address: String,
    pub port: u32,
    pub database: String,
    pub user: String,
    pub password: String,
    pub ssl: bool,
    pub auth_source: String,
    pub permission_collection: String,
    pub role_collection: String,
    pub user_collection: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub mongodb: MongoDB,
    pub jwt: JWT,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let c = config::Config::builder()
            .add_source(config::Environment::default())
            .build();

        return match c {
            Ok(r) => r.try_deserialize(),
            Err(e) => Err(e),
        };
    }
}

pub async fn get_mongo_config(conf: &Config) -> Database {
    let client = Client::with_uri_str(format!(
        "mongodb://{}:{}@{}:{}/?authSource={}&ssl={}",
        conf.mongodb.user,
        conf.mongodb.password,
        conf.mongodb.address,
        conf.mongodb.port,
        conf.mongodb.auth_source,
        conf.mongodb.ssl
    ))
    .await;

    client.unwrap().database(&conf.mongodb.database)
}
