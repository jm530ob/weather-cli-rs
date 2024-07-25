use std::io;
use colored::Colorize;

use crate::models::*;
use crate::api::{get_weather_data, read_json, store_json};


pub async fn print_weather(prompt_user: bool) {

    let (city, key) = setup(prompt_user).await;

    let weather = get_weather_data(city.lat, city.lon, &key.key).await.unwrap();

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

async fn setup(prompt_user: bool) -> (City, ApiKey) {
    let city_json = read_json("city_config.json").unwrap().to_string();
    let city: City;
    if prompt_user {
        let cities: Vec<City> = serde_json::from_str(&city_json).unwrap();
        city = get_city_from_opts(cities);
    } else {
        city = serde_json::from_str(&city_json).unwrap();
    }
    
    let api_key_json = read_json("key_config.json").unwrap().to_string();
    let api_key: ApiKey = serde_json::from_str(&api_key_json).unwrap();
    (city, api_key)
}

fn get_city_from_opts(cities: Vec<City>) -> City {
    for (i, city) in cities.iter().enumerate() {
        println!("{}. {:?}", i + 1, city);
    }
    println!("{}", "Please select a city to monitor for weather updates".green());

    let mut option = "".to_owned();
    io::stdin().read_line(&mut option).expect("Invalid input");
    let num: i32 = option.trim().parse().expect("Please enter an integer value!");

    store_json(serde_json::to_value(cities[(num - 1) as usize].clone()).expect("Could not serialize City"), "city_config.json");
    // serde_json::

    cities[(num - 1) as usize].clone()


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