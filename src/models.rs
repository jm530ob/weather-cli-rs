#![allow(dead_code)]

use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct ApiKey {
    pub key: String
}

#[derive(Deserialize, Debug)]
pub struct City {
    pub name: String,
    pub country: String,
    pub lat: f32,
    pub lon: f32
}

#[derive(Deserialize, Debug)]
pub struct WeatherInfo {
    pub visibility: i32, 
    pub weather: Vec<Weather>,
    pub main: Main,
    pub wind: Wind,
    pub clouds: Clouds
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub description: String
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub pressure: i32,
    pub humidity: i32
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: f32,
    pub deg: i32
}

#[derive(Deserialize, Debug)]
pub struct Clouds {
    pub all: i32
}