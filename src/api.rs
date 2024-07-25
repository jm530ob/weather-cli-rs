use std::cmp;
use std::fmt::format;
use std::io::{self, Read};
use std::io::Write;
use std::fs::File;
use std::sync::Arc;
use clap::builder::Str;
use reqwest::Error;
use colored::Colorize;
use clap::Parser;
use serde_json::{json, Value};

use crate::args::{Cli, CmdType, City};
use crate::models::*;
use crate::utils::print_weather;


pub async fn init() {
    let args = Cli::parse();
    match args.command_type {
        CmdType::Key(cmd) => {
            store_json(serde_json::json!({"key": cmd.api_key}), "key_config.json");
            return
        },
        CmdType::Set(cmd) => {
            set_location_data(&cmd.name, cmd.country)
                .await
                .expect("Something went wrong while setting a location!");
        },
        _ => {
            print_weather(false).await; // if command is "go" just print weather
            return;
        }
    }
    print_weather(true).await;

}

/// Returns latitude and longitude with other useful stuff. 
/// Visit: <https://openweathermap.org/api/geocoding-api>
pub async fn set_location_data(city_name: &str, country: Option<String>) -> Result<(), reqwest::Error> {
    let api_key_json = read_json("key_config.json").unwrap().to_string();
    let api_key: ApiKey = serde_json::from_str(&api_key_json).unwrap();
    let mut url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?&limit=5&appid={}&q={}", 
        api_key.key,
        city_name
    );

    if let Some(country) = country {
        url = format!("{url},{country}")
    }

    let response = reqwest::get(url)
        .await?
        .json::<Value>()
        .await;
    // result.get("name");

    match response {
        Ok(res) => {
            store_json(res, "city_config.json");
            println!("Your preferred city has been set!");
        }
        _ => eprintln!("Data could not be fetched. For more information try using --help")
    }

    Ok(())

}

pub fn store_json(args: Value, path: &str) {
    let mut file = File::create(path);

    match &mut file {
        Ok(f) => {
            f.write_all(args.to_string().as_bytes());
        }
        Err(e) => {
            "".to_owned();   
        }    
    }
}

pub fn read_json(path: &str) -> Result<Value, serde_json::Error> {
    let mut file = File::open(&path)
        .expect(&format!("Could not open file: {}, please set up your API key", path));
    let mut contents = String::from("");
    file.read_to_string(&mut contents).expect("Something went wrong while reading a file!");
    let json: Value = serde_json::from_str(&contents)?;
    Ok(json)
}

// TODO: Tato funkcia bude mat jeden argument -> deserializovany json - bude obsahovat ulozenu konfiguraciu mesta
/// Fetches weather data from the OpenWeatherMap API. 
pub async fn get_weather_data(lat: f32, lon: f32, key: &str) -> Result<WeatherInfo, Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=metric",
        lat, lon, key
    );
    let res = reqwest::get(url)
        .await?
        .json::<WeatherInfo>()
        .await?;
    Ok(res)
}