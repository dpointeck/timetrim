use chrono::prelude::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// end time of cats timebooking
    #[arg(short, long, default_value_t = Local::now().format("%H:%M").to_string())]
    pub time: String,

    /// pause time consumed in minutes
    #[arg(short, long, default_value_t = 45)]
    pub pause: u8,
}
