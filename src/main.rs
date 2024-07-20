//! # Weather-cli-rs
//! A simple weather CLI application written in Rust.
//! 
//! This application retrieves and displays current weather information for a specified city.
//! 
//! Future plans include integrating a Text User Interface (TUI).


mod models;
mod api;
mod utils;
mod token;

use models::*;
use api::{get_location_data, get_weather_data};
use utils::print_weather;

#[tokio::main]
pub async fn main() {
    let coords: Vec<GeoLocation> = get_location_data()
        .await
        .unwrap();
    let weather_data = get_weather_data(&coords)
        .await
        .unwrap();
    
    
    print_weather(&coords[0], &weather_data);

}

