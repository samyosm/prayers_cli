use crate::settings::get_local_coords;
use chrono;
use salah::prelude::*;

pub fn get_prayer_times() -> PrayerTimes {
    // use custom parameters
    let (lat, lon) = get_local_coords().expect("valid coords");
    let coords = Coordinates::new(lat, lon);
    let date = chrono::offset::Utc::now().date();
    let params = Configuration::with(Method::NorthAmerica, Madhab::Hanafi);
    PrayerSchedule::new()
        .on(date)
        .for_location(coords)
        .with_configuration(params)
        .calculate()
        .expect("valid prayer times")
}
