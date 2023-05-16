use crate::settings::get_local_coords;
use salah::prelude::*;

pub fn string_to_prayer(name: &str) -> Prayer {
    match name {
        "fajr" => Prayer::Fajr,
        "dhuhr" => Prayer::Dhuhr,
        "asr" => Prayer::Asr,
        "maghrib" => Prayer::Maghrib,
        "isha" => Prayer::Isha,
        _ => panic!("unrecognized prayer name"),
    }
}

pub fn get_prayer_times() -> PrayerTimes {
    // use custom parameters
    let (lat, lon) = get_local_coords().expect("valid coords");

    let coords = Coordinates::new(lat, lon);
    let date = Utc::now().date();
    let params = Configuration::with(Method::NorthAmerica, Madhab::Hanafi);
    PrayerSchedule::new()
        .on(date)
        .for_location(coords)
        .with_configuration(params)
        .calculate()
        .expect("valid prayer times")
}
