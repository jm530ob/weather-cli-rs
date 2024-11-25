# weather-cli-rs
![GitHub last commit](https://img.shields.io/github/last-commit/jm530ob/weather-cli-rs)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/jm530ob/weather-cli-rs)
![Crates.io Total Downloads](https://img.shields.io/crates/d/weather-cli-rs)
![GitHub Repo stars](https://img.shields.io/github/stars/jm530ob/weather-cli-rs)
<img src="https://github.com/user-attachments/assets/287c2cfe-648b-4948-bd98-d40259dffc9b" width="600" />

A simple weather CLI application written in Rust.

This application retrieves and displays current weather information for a specified city.

Future plans include integrating a Text User Interface (TUI).

## Installation

```bash
cargo install weather-cli-rs
```

## Usage
Ensure you have created an API key at https://openweathermap.org/. You will need to provide this key as an argument!

Run in your terminal
```
$ weather-cli-rs <COMMAND>
```
## Commands
| Command  | Description |
| ------------- | ------------- |
| `key` | Ensure you have entered a valid API key before continuing |
| `set` | Sets up your city |
| `go` | Executes the app using the stored configuration |
| `help` | Print this message or the help of the given subcommand(s) |

## Example

```
$ weather-cli-rs key <API_KEY>
$ weather-cli-rs set --name "Kyoto" --country "JP"
```
Note: The `--country` (`-c`) flag is optional. However, you will likely be prompted with several options to select your city

```text
Weather in Kyoto - JP
        🢒 scattered clouds ☁
        🢒 Temperature: 31.81°C | feels_like 37.85°C
        🢒 Atmospheric pressure : 1010 hPa
        🢒 Visibility: 10000 m
        🢒 Humidity: 64%
        🢒 Wind speed: 2.24 m/s
        🢒 Clouds: 39%
```
Save yourself a few clicks next time

```
$ weather-cli-rs go
```
