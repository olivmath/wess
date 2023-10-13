use lazy_static::lazy_static;
use std::{fs, sync::Arc};
use toml::{from_str, Value};

lazy_static! {
    pub static ref CONFIG: Arc<Config> = {
        let config = Config::from_file("wess.toml").expect("Cant not find config file");
        Arc::new(config)
    };
}

pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub reader: ReaderConfig,
    pub writer: WriterConfig,
    pub runner: RunnerConfig,
}

pub struct ServerConfig {
    pub port: u16,
    pub address: String,
}

pub struct DatabaseConfig {
    pub db: String,
    pub stage: String,
}

pub struct ReaderConfig {
    pub cache_size: usize,
    pub channel_size: usize,
}

pub struct WriterConfig {
    pub channel_size: usize,
}

pub struct RunnerConfig {
    pub cache_size: usize,
    pub channel_size: usize,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, String> {
        let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let value: Value = from_str(&contents).map_err(|e| e.to_string())?;

        let server = ServerConfig {
            port: value["server"]["port"]
                .as_integer()
                .expect("missing 'server.port'") as u16,
            address: value["server"]["address"]
                .as_str()
                .expect("missing 'server.address'")
                .to_owned(),
        };

        let database = DatabaseConfig {
            db: value["database"]["db"]
                .as_str()
                .expect("missing 'database.db'")
                .to_owned(),
            stage: value["database"]["stage"]
                .as_str()
                .expect("missing 'database.stage'")
                .to_owned(),
        };

        let reader = ReaderConfig {
            cache_size: value["reader"]["cache_size"]
                .as_integer()
                .expect("missing 'reader.cache_size'") as usize,
            channel_size: value["reader"]["channel_size"]
                .as_integer()
                .expect("missing 'reader.channel_size'") as usize,
        };

        let writer = WriterConfig {
            channel_size: value["writer"]["channel_size"]
                .as_integer()
                .expect("missing 'writer.channel_size'") as usize,
        };

        let runner = RunnerConfig {
            cache_size: value["runner"]["cache_size"]
                .as_integer()
                .expect("missing 'runner.cache_size'") as usize,
            channel_size: value["runner"]["channel_size"]
                .as_integer()
                .expect("missing 'runner.channel_size'") as usize,
        };

        Ok(Self {
            server,
            database,
            reader,
            writer,
            runner,
        })
    }
}
