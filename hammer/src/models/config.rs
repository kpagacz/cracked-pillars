use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

pub(crate) static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let config_file =
        std::env::var("HAMMER_CONFIG_FILE").unwrap_or_else(|_| "./hammer.toml".into());
    let settings = match config::Config::builder()
        .add_source(config::File::with_name(&config_file))
        .add_source(config::Environment::with_prefix("HAMMER"))
        .build()
    {
        Ok(settings) => settings,
        Err(err) => {
            tracing::error!("Failed to load configuration file or environment variables. {err:?}");
            panic!("Configuration error!");
        }
    };

    match settings.try_deserialize::<Config>() {
        Ok(config) => config,
        Err(err) => {
            tracing::error!("Failed to deserialize configuration. {err:?}");
            panic!("Configuration error!");
        }
    }
});

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) abilities_path: String,
    pub(crate) items_path: String,
    pub(crate) db_path: String,
    pub(crate) db_migrations: String,
}
