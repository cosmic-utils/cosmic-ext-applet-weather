use cosmic::cosmic_config::{
    self, Config, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry,
};

const CONFIG_VERSION: u64 = 1;

pub const APP_ID: &str = "io.github.cosmic-utils.cosmic-ext-applet-weather";
pub const SUN_ICON: &str = "io.github.cosmic-utils.cosmic-ext-applet-weather-sun-symbolic";
pub const MOON_ICON: &str = "io.github.cosmic-utils.cosmic-ext-applet-weather-moon-symbolic";

#[derive(Default, Debug, CosmicConfigEntry)]
pub struct WeatherConfig {
    pub latitude: f64,
    pub longitude: f64,
}

impl WeatherConfig {
    fn config_handler() -> Option<Config> {
        Config::new(APP_ID, CONFIG_VERSION).ok()
    }

    pub fn config() -> WeatherConfig {
        match Self::config_handler() {
            Some(config_handler) => WeatherConfig::get_entry(&config_handler)
                .map_err(|error| {
                    tracing::info!("Error whilst loading config: {:#?}", error);
                })
                .unwrap_or_default(),
            None => WeatherConfig::default(),
        }
    }
}
