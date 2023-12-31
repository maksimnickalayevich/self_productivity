use config::{Config, ConfigError};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DbConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_name: String,
    db_max_connections: u32,
    connection_url: Option<String>
}

impl DbConfig {
    fn compose_connection_url(&self) -> String {
        return format!(
            "postgres://{}:{}@{}:{}/{}",
             self.db_username, 
             self.db_password,
             self.db_host,
             self.db_port,
             self.db_name
        );
    }

    pub fn from_env() -> DbConfig {
        let _cfg_builder = config::Config::builder()
            .add_source(config::Environment::default());

        let config_result: Result<Config, ConfigError> = _cfg_builder.build();
        let raw_config = match config_result {
            Ok(v) => v,
            Err(e) => panic!("Unable to load config from env {:?}", e)
        };
        let config: DbConfig = raw_config.try_deserialize().unwrap_or_default();
        println!("Loaded db config: {:?}", config);
        return config;
    }

    pub fn get_connection_url(&self) -> &str {
        return match &self.connection_url {
            Some(url) => url,
            _ => panic!("No connection url found in DbConfig {:?}", self)
        }
    }
    
    pub fn set_connection_url(&mut self) {
        let conn_url: String = self.compose_connection_url();
        self.connection_url = Some(conn_url);
    }

    pub fn get_max_connections(&self) -> u32 {
        self.db_max_connections
    }
}
