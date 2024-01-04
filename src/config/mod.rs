#[derive(Debug, Default)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Debug, Default)]
pub struct ServerConfig {
    pub database_url: String,
}

pub fn load_config() -> Config {
    todo!()
}
