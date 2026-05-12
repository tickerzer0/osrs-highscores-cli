use clap::Parser;

mod types;

// A CLI tool to fetch statistics of OSRS players and display them
#[derive(Parser, Debug)]
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
    println!("{}", rsn);
}
