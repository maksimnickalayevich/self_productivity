#[derive(Debug, Default, serde::Deserialize)]
pub struct ServerConfig {
    pub server_address: String,
    pub server_port: u16,
}

impl ServerConfig {
    pub fn load_config() -> ServerConfig {
        let cfg_ = config::Config::builder()
        .add_source(config::Environment::default())
        .build()
        .expect("Unable to load server config");
        let server_address: ServerConfig = cfg_.try_deserialize().expect("Unable to deserialize server config");
        return server_address;
    }
}
