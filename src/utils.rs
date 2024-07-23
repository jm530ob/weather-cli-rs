use serde_json::Value;

use crate::{models::*};

pub async fn print_weather() {
    use crate::api::{get_weather_data, read_json};

    let city_json = read_json("city_config.json").unwrap().to_string();
    let city: City = serde_json::from_str(&city_json).unwrap();
    let api_key_json = read_json("key_config.json").unwrap().to_string();
    let api_key: ApiKey = serde_json::from_str(&api_key_json).unwrap();
    let weather = get_weather_data(city.lat, city.lon, &api_key.key).await.unwrap();

    let result = format!(
        "Weather in {} - {}
        🢒 {} {}
        🢒 Temperature: {}°C | feels_like {}°C 
        🢒 Atmospheric pressure : {} hPa
        🢒 Visibility: {} m
        🢒 Humidity: {}%
        🢒 Wind speed: {} m/s
        🢒 Clouds: {}%
        ",
        city.name, city.country,
        weather.weather[0].description, get_icon(&weather.weather[0].description),
        weather.main.temp, weather.main.feels_like,
        weather.main.pressure,
        weather.visibility,
        weather.main.humidity,
        weather.wind.speed,
        weather.clouds.all
        
    );
    println!("{}", result);
}

fn get_icon(description: &str) -> String {
    let has = |words: &[&str]| words.iter().any(|word| description.contains(word));
    match description {
        "clear sky" => "☀",
        "few clouds" => "🌤",
        "scattered clouds" | "overcast clouds" => "☁",
        "tornado" => "🌪",
        _ if has(&["rain", "drizzle"]) => "🌧",
        _ if has(&["thunderstorm"]) => "⛈",
        _ if has(&["snow", "sleet"]) => "🌨",
        _ if has(&["mist", "smoke", "sand", "dust"]) => "🌫",
        _ => "",
    }
    .to_owned()

 }