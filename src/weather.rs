use reqwest::header;
use serde::Deserialize;

use crate::config::APP_ID;

#[derive(Deserialize)]
pub struct WeatherApi {
    properties: Properties,
}

#[derive(Deserialize)]
struct Properties {
    timeseries: Vec<Timeseries>,
}

#[derive(Deserialize)]
struct Timeseries {
    data: Data,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Data {
    instant: Instant,
    next_1_hours: Next1Hours,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Instant {
    details: InstantDetails,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct InstantDetails {
    air_pressure_at_sea_level: f32,
    air_temperature: f32,
    cloud_area_fraction: f32,
    relative_humidity: f32,
    wind_from_direction: f32,
    wind_speed: f32,
    ultraviolet_index_clear_sky: f32,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Next1Hours {
    summary: Summary,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Summary {
    symbol_code: String,
}

pub struct WeatherApiResponse {
    pub temp: i32,
    pub icon: String,
    pub uv: f32,
}

impl Default for WeatherApiResponse {
    fn default() -> Self {
        Self {
            temp: 0,
            icon: String::from("weather-clear"),
            uv: 0.0,
        }
    }
}

impl WeatherApi {
    pub async fn get_location_forecast(
        latitude: String,
        longitude: String,
    ) -> Result<WeatherApiResponse, reqwest::Error> {
        let url = format!(
            "https://api.met.no/weatherapi/locationforecast/2.0/complete?lat={latitude}&lon={longitude}",
        );

        let request_builder = reqwest::Client::new()
            .get(url)
            .header(header::USER_AGENT, APP_ID);

        let response = request_builder.send().await?;
        let data = response.json::<WeatherApi>().await?;

        let weather = data
            .properties
            .timeseries
            .first()
            .map(|ts| {
                let details = &ts.data.instant.details;

                WeatherApiResponse {
                    temp: details.air_temperature as i32,
                    icon: Self::symbol_code_to_icon(&ts.data.next_1_hours.summary.symbol_code)
                        .to_string(),
                    uv: details.ultraviolet_index_clear_sky,
                }
            })
            .unwrap_or(WeatherApiResponse::default());

        Ok(weather)
    }

    /// Maps met.no/MET Norway symbol codes to freedesktop.org weather icon names
    fn symbol_code_to_icon(symbol_code: &str) -> &'static str {
        // Parse out the time suffix (_day, _night, _polartwilight)
        let (base, is_night) = if let Some(base) = symbol_code.strip_suffix("_night") {
            (base, true)
        } else if let Some(base) = symbol_code.strip_suffix("_polartwilight") {
            (base, true)
        } else if let Some(base) = symbol_code.strip_suffix("_day") {
            (base, false)
        } else {
            (symbol_code, false)
        };

        match base {
            // Clear sky
            "clearsky" if is_night => "weather-clear-night",

            // Partly cloudy / fair
            "fair" | "partlycloudy" => {
                if is_night {
                    "weather-few-clouds-night"
                } else {
                    "weather-few-clouds"
                }
            }

            // Overcast
            "cloudy" => "weather-overcast",

            // Fog
            "fog" => "weather-fog",

            // Rain (no thunder)
            "lightrain" | "rain" | "heavyrain" => "weather-showers",

            // Rain showers (no thunder)
            "lightrainshowers" | "rainshowers" | "heavyrainshowers" => "weather-showers-scattered",

            // Snow (all variants)
            "lightsnow" | "snow" | "heavysnow" | "lightsnowshowers" | "snowshowers"
            | "heavysnowshowers" => "weather-snow",

            // Sleet (rain + snow mix)
            "lightsleet" | "sleet" | "heavysleet" | "lightsleetshowers" | "sleetshowers"
            | "heavysleetshowers" => "weather-showers",

            // Thunder variants
            "lightrainandthunder"
            | "rainandthunder"
            | "heavyrainandthunder"
            | "lightrainshowersandthunder"
            | "rainshowersandthunder"
            | "heavyrainshowersandthunder"
            | "lightsnowandthunder"
            | "snowandthunder"
            | "heavysnowandthunder"
            | "lightssnowshowersandthunder"
            | "snowshowersandthunder"
            | "heavysnowshowersandthunder"
            | "lightsleetandthunder"
            | "sleetandthunder"
            | "heavysleetandthunder"
            | "lightssleetshowersandthunder"
            | "sleetshowersandthunder"
            | "heavysleetshowersandthunder" => "weather-storm",

            // Fallback
            _ => "weather-clear",
        }
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct IpApi {
    pub lat: f32,
    pub lon: f32,
    pub city: String,
    pub regionName: String,
}

impl IpApi {
    pub async fn get_location_from_ip() -> Result<IpApi, reqwest::Error> {
        let url = "http://ip-api.com/json?fields=lat,lon,city,regionName";

        let request_builder = reqwest::Client::new()
            .get(url)
            .header(header::USER_AGENT, APP_ID);

        let response = request_builder.send().await?;
        let response = response.json::<IpApi>().await?;

        Ok(response)
    }
}
