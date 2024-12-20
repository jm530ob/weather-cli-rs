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

#[tokio::main]
pub async fn main() {
    api::init().await;
}

