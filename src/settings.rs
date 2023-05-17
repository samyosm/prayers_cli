use clap::__derive_refs::once_cell::sync::Lazy;
use reqwest;
use salah::{DateTime, Local, Madhab, Method, Utc};
use serde::{Deserialize, Serialize};

// TODO: print a helpful message when an error occurs
static CONFIG: Lazy<Config> =
    Lazy::new(|| confy::load("prayers_utils", None).expect("a valid configuration"));

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub latitude: f64,
    pub longitude: f64,
    pub time_format: String,
    pub calculation_method: String,
    pub madhab: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            latitude: 40.0,
            longitude: -70.0,
            time_format: String::from("%Ih%m %p"),
            calculation_method: String::from("isna"),
            madhab: String::from("hanafi"),
        }
    }
}

pub fn get_calculation_method() -> Method {
    let local_method = &CONFIG.calculation_method;
    match local_method.to_lowercase().as_ref() {
        "isna" => Method::NorthAmerica,
        "mml" => Method::MuslimWorldLeague,
        "dubai" => Method::Dubai,
        "moonsightingcommittee" => Method::MoonsightingCommittee,
        "kuwait" => Method::Kuwait,
        "egyptian" => Method::Egyptian,
        "qatar" => Method::Qatar,
        _ => panic!("unkown calculation method"),
    }
}

pub fn get_madhab() -> Madhab {
    let local_madhab = &CONFIG.madhab;
    match local_madhab.to_lowercase().as_ref() {
        "hanafi" => Madhab::Hanafi,
        "shafi" => Madhab::Shafi,
        _ => panic!("unkown madhab"),
    }
}

pub async fn init() {
    let (lat, lon) = fetch_coords().await.expect("valid coords");
    let mut tmp_config: Config = confy::load("prayers_utils", None).expect("a valid configuration");
    tmp_config.latitude = lat;
    tmp_config.longitude = lon;

    confy::store("prayers_utils", None, tmp_config)
        .expect("longitude and latitude to have been initialized")
}

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
