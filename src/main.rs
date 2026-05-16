use std::process;

use clap::Parser;
use request::get_user_data;

use crate::types::{osrs_api::OsrsApiErr, player::Player};

mod request;
mod types;

// A CLI tool to fetch statistics of OSRS players and display them
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // Required parameter, the player's runescape name (RSN)
    #[arg(short, long)]
    rsn: String,

    #[arg(short)]
    boss_only: Option<bool>,

    #[arg(short)]
    skills_only: Option<bool>
}

fn main() {
    let cli = Cli::parse();
    
    let rsn = cli.rsn;

    let data: Result<String, OsrsApiErr> = get_user_data(rsn);
    let body = match data {
        Ok(body) => body,
        Err(e) => {
            eprintln!("There was an issue fetching the user data: {e}");
            process::exit(1)
        }
    };

    let player: Player = match serde_json::from_str(&body) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Failed to parse JSON content to Player struct: {e}");
            process::exit(1)
        }
    };

    println!("{player}");

}
