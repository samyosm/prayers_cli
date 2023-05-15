use reqwest;

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
    Ok((0.0, 0.0))
}
