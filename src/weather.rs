use reqwest::header;
use serde::Deserialize;

use crate::config::APP_ID;

#[derive(Deserialize)]
pub struct WeatherApiResponse {
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
    next_6_hours: Next6Hours,
    next_12_hours: Next12Hours,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Instant {
    details: InstantDetails,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct InstantDetails {
    air_pressure_at_sea_level: f64,
    air_temperature: f64,
    cloud_area_fraction: f64,
    relative_humidity: f64,
    wind_from_direction: f64,
    wind_speed: f64,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Next1Hours {
    summary: Summary,
    details: Next1HoursDetails,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Next6Hours {
    summary: Summary,
    details: Next6HoursDetails,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Next12Hours {
    summary: Summary,
    details: Next12HoursDetails,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Summary {
    symbol_code: String,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Next1HoursDetails {
    precipitation_amount: f64,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Next6HoursDetails {
    precipitation_amount: f64,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Next12HoursDetails {
    precipitation_amount: f64,
}

pub async fn get_location_forecast(
    latitude: String,
    longitude: String,
) -> Result<i32, reqwest::Error> {
    let url = format!(
        "https://api.met.no/weatherapi/locationforecast/2.0/compact?lat={latitude}&lon={longitude}",
    );

    let request_builder = reqwest::Client::new()
        .get(url)
        .header(header::USER_AGENT, APP_ID);

    let response = request_builder.send().await?;
    let data = response.json::<WeatherApiResponse>().await?;

    let current_temperature = data
        .properties
        .timeseries
        .first()
        .map(|d| d.data.instant.details.air_temperature as i32)
        .unwrap_or(0);

    Ok(current_temperature)
}
