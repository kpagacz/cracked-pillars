use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

pub(crate) static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("hammer"))
        .add_source(config::Environment::with_prefix("HAMMER"))
        .build()
        .unwrap();

    settings.try_deserialize::<Config>().unwrap()
});

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) abilities_path: String,
}
