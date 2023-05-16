use clap::__derive_refs::once_cell::sync::Lazy;
use reqwest;
use salah::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

// TODO: print a helpful message when an error occurs
static CONFIG: Lazy<Config> =
    Lazy::new(|| confy::load("prayers_utils", None).expect("a valid configuration"));

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub latitude: f64,
    pub longitude: f64,
    pub time_format: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            latitude: 40.0,
            longitude: -70.0,
            time_format: String::from("%Ih%m %p"),
        }
    }
}

pub async fn init() {}

pub async fn fetch_coords() -> Result<(f64, f64), reqwest::Error> {
    // Make an HTTP GET request to the geolocation API
    let response = reqwest::get("https://freegeoip.app/json/").await?;

    // Parse the response as JSON
    let json = response.json::<serde_json::Value>().await?;

    // Extract the latitude and longitude values from the JSON
    let latitude = json["latitude"].as_f64().unwrap_or(0.0);
    let longitude = json["longitude"].as_f64().unwrap_or(0.0);

    Ok((latitude, longitude))
}

pub fn get_local_coords() -> Result<(f64, f64), &'static String> {
    Ok((CONFIG.latitude, CONFIG.longitude))
}

pub fn get_formated_date(date: DateTime<Utc>) -> String {
    let format = &CONFIG.time_format;
    date.with_timezone(&Local).format(&format).to_string()
}
