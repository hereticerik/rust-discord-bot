use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde::Deserialize;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::prelude::*;

const OPENWEATHERMAP_API_KEY: &str = "YOUR_API_KEY_HERE";

#[derive(Deserialize)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
}

#[derive(Deserialize)]
struct Weather {
    description: String,
}

#[derive(Deserialize)]
struct Main {
    temp: f32,
    feels_like: f32,
}

struct Handler {
    start_time: std::time::Instant,
    commands: std::collections::HashMap<String, String>,
}

impl Handler {
    fn new() -> Handler {
        let start_time = std::time::Instant::now();
        let commands = read_commands("commands.txt").unwrap_or_default();
        Handler {
            start_time,
            commands,
        }
    }

    fn handle_command(&self, ctx: &Context, msg: &Message, command: &str) -> bool {
        match command {
            "ping" => {
                let start_time = self.start_time;
                let end_time = std::time::Instant::now();

                let response_time = end_time.duration_since(start_time);
                let response_time_ms = response_time.as_millis();

                let response = format!("Pong! Response time: {}ms", response_time_ms);

                let _ = msg.channel_id.say(&ctx.http, response);
                true
            }
            "weather" => {
                if let Some(city) = msg.content.split_whitespace().nth(1) {
                    let weather = get_weather(city);

                    if let Some(response) = weather {
                        let _ = msg.channel_id.say(&ctx.http, response);
                        true
                    } else {
                        let _ = msg.channel_id.say(&ctx.http, "Unable to get weather.");
                        true
                    }
                } else {
                    let _ = msg.channel_id.say(&ctx.http, "Please provide a city name.");
                    true
                }
            }
            _ => {
                if let Some(response) = self.commands.get(command) {
                    let _ = msg.channel_id.say(&ctx.http, response);
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!") {
            let command = msg.content[1..].to_lowercase();

            if !self.handle_command(&ctx, &msg, &command) {
                let _ = msg.channel_id.say(&ctx.http, "Unknown command.");
            }
        }
    }
}

fn read_commands(filename: &str) -> std::io::Result<std::collections::HashMap<String, String>> {
    let file = std::fs::File::open(filename)?;
    let reader = std::io::BufReader::new(file);
    let mut commands = std::collections::HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<_> = line.splitn(2, ":").collect();

        if parts.len() == 2 {
            let key = parts[0].trim().to_lowercase();
            let value = parts[1].trim().to_owned();
            commands.insert(key, value);
        }
    }

    Ok(commands)
}

fn get_weather(city:
