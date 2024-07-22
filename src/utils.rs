use crate::models::*;

// pub fn print_weather(location: &GeoLocation, weather: &WeatherInfo) {
//     let result = format!(
//         "Weather in {} - {}
//         🢒 {} {}
//         🢒 Temperature: {}°C | feels_like {}°C 
//         🢒 Atmospheric pressure : {} hPa
//         🢒 Visibility: {} m
//         🢒 Humidity: {}%
//         🢒 Wind speed: {} m/s
//         🢒 Clouds: {}%
//         ",
//         location.name, location.country,
//         weather.weather[0].description, get_icon(&weather.weather[0].description),
//         weather.main.temp, weather.main.feels_like,
//         weather.main.pressure,
//         weather.visibility,
//         weather.main.humidity,
//         weather.wind.speed,
//         weather.clouds.all
        
//     );
//     println!("{}", result);
// }

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