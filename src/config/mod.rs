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

#[derive(Clone)]
pub enum DataBase {
    POSTGREE,
    ROCKSDB,
    MONGO,
    MYSQL,
}

#[derive(Clone)]
pub enum Stage {
    DEV,
    PROD,
}

pub struct DatabaseConfig {
    pub db: DataBase,
    pub stage: Stage,
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
    fn read_and_validate_field<T, F>(config: &Value, field_path: &str, validator: F) -> T
    where
        T: Clone,
        F: Fn(&str) -> Option<T>,
    {
        let value = config
            .get(field_path)
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| panic!("missing '{}'", field_path));

        match validator(value) {
            Some(result) => result,
            None => panic!("not supported: {}", value),
        }
    }

    pub fn from_file(path: &str) -> Result<Self, String> {
        let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let value: Value = from_str(&contents).map_err(|e| e.to_string())?;

        let server = ServerConfig {
            port: value["server"]["port"]
                .as_integer()
                .and_then(|n| Some(n as u16))
                .unwrap_or_else(|| panic!("missing or invalid 'server.port'")),
            address: Self::read_and_validate_field(&value, "server.address", |s| {
                Some(s.to_owned())
            }),
        };

        let database = DatabaseConfig {
            db: Self::read_and_validate_field(&value, "database.db", |s| match s {
                "ROCKSDB" => Some(DataBase::ROCKSDB),
                _ => None,
            }),
            stage: Self::read_and_validate_field(&value, "database.stage", |s| match s {
                "DEV" => Some(Stage::DEV),
                "PROD" => Some(Stage::PROD),
                _ => None,
            }),
        };

        let reader = ReaderConfig {
            cache_size: value["reader"]["cache_size"]
                .as_integer()
                .and_then(|n| Some(n as usize))
                .unwrap_or_else(|| panic!("missing or invalid 'reader.cache_size'")),
            channel_size: value["reader"]["channel_size"]
                .as_integer()
                .and_then(|n| Some(n as usize))
                .unwrap_or_else(|| panic!("missing or invalid 'reader.channel_size'")),
        };

        let writer = WriterConfig {
            channel_size: value["writer"]["channel_size"]
                .as_integer()
                .and_then(|n| Some(n as usize))
                .unwrap_or_else(|| panic!("missing or invalid 'writer.channel_size'")),
        };

        let runner = RunnerConfig {
            cache_size: value["runner"]["cache_size"]
                .as_integer()
                .and_then(|n| Some(n as usize))
                .unwrap_or_else(|| panic!("missing or invalid 'runner.cache_size'")),
            channel_size: value["runner"]["channel_size"]
                .as_integer()
                .and_then(|n| Some(n as usize))
                .unwrap_or_else(|| panic!("missing or invalid 'runner.channel_size'")),
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
