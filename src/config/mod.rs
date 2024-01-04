use crate::ServerResult;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub system: SystemConfig,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SystemConfig {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ServerConfig {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn get_api_url(&self) -> String {
        format!("http://localhost:{}/api", self.port)
    }

    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn get_swigger_url(&self) -> String {
        format!("http://localhost:{}/", self.port)
    }
}

pub fn load_config() -> ServerResult<Config> {
    dotenvy::dotenv().ok();

    let mut config: Config;

    if cfg!(feature = "dev") {
        config = Figment::new()
            .merge(Serialized::defaults(Config::default()))
            .select("dev")
            .merge(Toml::file("Config.toml"))
            .extract()?;

        let server: ServerConfig = Figment::new().merge(Env::prefixed("")).extract()?;

        config.server = server;
    } else {
        config = Figment::new()
            .merge(Serialized::defaults(Config::default()))
            .select("production")
            .merge(Toml::file("Config.toml"))
            .extract()?;
    }
    Ok(config)
}
