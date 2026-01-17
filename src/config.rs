use cosmic::cosmic_config::{
    self, Config, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry,
};

const CONFIG_VERSION: u64 = 1;

pub const APP_ID: &str = "io.github.cosmic_utils.weather-applet";

#[derive(Clone, Default, Debug, CosmicConfigEntry)]
pub struct WeatherConfig {
    pub latitude: f64,
    pub longitude: f64,
    pub use_fahrenheit: bool,
    pub use_ip_location: bool,
}

impl WeatherConfig {
    fn config_handler() -> Option<Config> {
        Config::new(APP_ID, CONFIG_VERSION).ok()
    }

    pub fn config() -> WeatherConfig {
        match Self::config_handler() {
            Some(config_handler) => WeatherConfig::get_entry(&config_handler)
                .map_err(|error| {
                    tracing::error!("Error whilst loading config: {:#?}", error);
                })
                .unwrap_or_default(),
            None => WeatherConfig::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Flags {
    pub config: WeatherConfig,
    pub config_handler: Option<cosmic_config::Config>,
}

pub fn flags() -> Flags {
    let (config, config_handler) = (WeatherConfig::config(), WeatherConfig::config_handler());

    Flags {
        config,
        config_handler,
    }
}
