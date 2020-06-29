mod jsonrpc;
mod poll;
mod statuses;

use serde::Deserialize;
use iced::{Application, Settings};

#[derive(Deserialize)]
struct Config {
    names: Vec<String>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).expect("Missing mode");
    let config_string = args.get(2).expect("Missing config");
    match mode.as_str() {
        "poll" => {
            let config: Config = serde_json::from_str(config_string).expect("Invalid config");
            poll::App::run(Settings::with_flags(config.names));
        }
        _ => {}
    };
}
