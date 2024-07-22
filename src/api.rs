use std::fmt::format;
use std::io::{self, Read};
use std::io::Write;
use std::fs::File;
use clap::builder::Str;
use reqwest::Error;
use colored::Colorize;
use clap::Parser;
use serde_json::{json, Value};

use crate::args::Args;
use crate::models::*;
use crate::token::API_KEY;

mod Urls {
    
}

/// Returns latitude and longitude with other useful stuff. 
/// Visit: <https://openweathermap.org/api/geocoding-api>
pub async fn get_location_data() -> Result<(), reqwest::Error> {
    let args = Args::parse();
    let city: String = args.name;

    let geocoding_api_url = |name: String, country: Option<String>, key: String| {
        match country {
            Some(country) => {
                format!("http://api.openweathermap.org/geo/1.0/direct?q={},{}&limit=5&appid={}", name, country, key)
            }
            None => {
                format!("http://api.openweathermap.org/geo/1.0/direct?q={}&limit=5&appid={}", name, key)
            }
        }
    }; 

    let weather_api_url = |lat: f32, lon: f32, key: String| {
        format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=metric", lat, lon, key)
    };

    let result = reqwest::get(url)
        .await?
        .json::<Value>()
        .await;
    // result.get("name");

    match result {
        Ok(a) => {
            store_json(&a);
            println!("{}", &a);
        }
        _ => ()
    }

    Ok(())
    // println!("AHOOOJ");
    // println!("{:?}", result);
    // return Ok(result)


}

fn store_json(args: &Value, path: String) {
    let mut file = File::create(path);

    match &mut file {
        Ok(f) => {
            f.write_all(args.to_string().as_bytes());
        }
        Err(e) => {
            "".to_owned();   
        }    
    }
    // file
}

fn read_json(path: String) -> Result<Value, serde_json::Error> {
    let mut file = File::open(&path)
        .expect(&format!("Could not open file: {}", path));
    let mut contents = String::from("");
    file.read_to_string(&mut contents).expect("Something went wrong while reading a file!");
    let json: Value = serde_json::from_str(&contents)?;
    Ok(json)
}

// TODO: Tato funkcia bude mat jeden argument -> deserializovany json - bude obsahovat ulozenu konfiguraciu mesta
// Fetches weather data from the OpenWeatherMap API. 
// Visit: <https://openweathermap.org/current>
// pub async fn get_weather_data(coords: &Vec<Args>) -> Result<WeatherInfo, Error> {
//     let url = format!(
//         "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=metric",
//         coords[0].lat, coords[0].lon, "f42e353ecf7c9819ca44363e10f9d058"
//     );
//     let res = reqwest::get(url)
//         .await?
//         .json::<WeatherInfo>()
//         .await?;
//     Ok(res)
// }