use reqwest::Error;
use clap::Parser;
use serde_json::Value;
use std::fs::File;
use std::io::{Read, Write};


use crate::args::{Cli, CmdType};
use crate::models::*;
use crate::utils::print_weather;

/// Initializes the app, sets up the CLI parser and listens for commands.
pub async fn init() {
    let args = Cli::parse();
    match args.command_type {
        CmdType::Key(cmd) => {
            write_json(serde_json::json!({"key": cmd.api_key}), "key_config.json");
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

/// Stores a JSON object of cities (up to a maximum of 5) from the generated URL link.
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

    match response {
        Ok(res) => {
            write_json(res, "city_config.json");
            println!("Your preferred city has been set!");
        }
        _ => eprintln!("Data could not be fetched. For more information try using --help")
    }

    Ok(())

}

pub fn write_json(args: Value, path: &str) {
    let mut file = File::create(path);

    match &mut file {
        Ok(f) => {
            f.write_all(args.to_string().as_bytes()).expect("Could not write to a file");
        }
        Err(_) => {
             eprintln!("Something went wrong!");
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