use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct User {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Server {
    homeserver: String,
    channel: String,
    server: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Wallet {
    address: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    user: User,
    server: Server,
    wallet: Wallet,
}

impl Settings {
    pub fn new() -> Result<Config, ConfigError> {
        // This resolves down to ~/.config/faucet_slobber/config.toml
        // or your local equivalent
        let mut configdir = dirs::config_dir().expect("no config directory found");
        configdir.push("faucet_slobber");
        configdir.push("config");
        configdir.set_extension("toml");

        Config::builder()
            .add_source(File::with_name(&configdir.to_str().unwrap()))
            // Add in settings from the environment (with a prefix of APP)
            .add_source(Environment::with_prefix("app"))
            .build()
    }
}
