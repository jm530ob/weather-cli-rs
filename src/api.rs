use std::io;
use std::io::Write;
use reqwest::Error;
use colored::Colorize;

use crate::models::*;
use crate::token::API_KEY;

/// Asks for user input, then searches for your city. 
/// Returns latitude and longitude with other useful stuff. 
/// Visit: <https://openweathermap.org/api/geocoding-api>
pub async fn get_location_data() -> Result<Vec<GeoLocation>, reqwest::Error> {
    loop {
        print!("{}", "Please enter your location (city name): ".magenta().bold());
        io::stdout().flush().unwrap();
        let mut location = "".to_owned();
        io::stdin()
            .read_line(&mut location)
            .expect("Something went wrong");
        let location: Vec<&str> = location
            .split(" ")
            .collect();

        if location.len() > 1 { 
            eprintln!("{}", "Invalid length of arguments!".red());
            continue;
        }

        let url = format!(
            "http://api.openweathermap.org/geo/1.0/direct?q={}&appid={}", 
            &location[0], API_KEY
        );
        let res = reqwest::get(url)
            .await?
            .json::<Vec<GeoLocation>>()
            .await?;
        return Ok(res)
    }
}

/// Fetches weather data from the OpenWeatherMap API. 
/// Visit: <https://openweathermap.org/current>
pub async fn get_weather_data(coords: &Vec<GeoLocation>) -> Result<WeatherInfo, Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=metric",
        coords[0].lat, coords[0].lon, API_KEY
    );
    let res = reqwest::get(url)
        .await?
        .json::<WeatherInfo>()
        .await?;
    Ok(res)
}