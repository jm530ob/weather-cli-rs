use crate::models::*;

pub fn print_weather(location: &GeoLocation, weather: &WeatherInfo) {
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
        location.name, location.country,
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
    if description == "clear sky" {
        "☀".to_owned()
    }
    else if description == "few clouds" {
        "🌤".to_owned()
    }
    else if description == "scattered clouds" || description == "overcast clouds" {
        "☁".to_owned()
    }
    else if description == "broken clouds" {
        "🌥".to_owned()
    }
    else if description.contains("rain") || description.contains("drizzle") {
        "🌧".to_owned()
    }
    else if description.contains("thunderstorm") {
        "⛈".to_owned()
    }
    else if description.contains("snow") || description.contains("sleet") {
        "🌨".to_owned()
    }
    else if description == ("mist") || description.contains("smoke") || description.contains("sand") 
        || description.contains("dust") {
            "🌫".to_owned()
        }
    else if description == "tornado" {
        "🌪".to_owned()
    }
    else {
        "".to_owned()
    }

 }