use crate::args::Args;
use clap::Parser;
use crate::time::get_utc_time;

fn main() {
    let args = Args::parse();
    println!("{:?}", get_utc_time());
    println!("time: {}, break: {}", args.time, args.pause)
}
