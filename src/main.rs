//! # Weather-cli-rs
//! A simple weather CLI application written in Rust.
//! 
//! This application retrieves and displays current weather information for a specified city.
//! 
//! Future plans include integrating a Text User Interface (TUI).

mod api;
mod models;
mod utils;
mod args;

use args::Cli;
use clap::Parser;
use models::*;
use api::{set_location_data};
// use utils::print_weather;

#[tokio::main]
pub async fn main() {
    api::init().await;
    // set_location_data().await.expect("Something went wrong while setting a location!");
    // let weather_data = get_weather_data(&coords)
    //     .await
    //     .unwrap();
    
    
    // print_weather(&coords[0], &weather_data);
    // println!("{}", args.city);
}

