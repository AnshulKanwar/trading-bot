use bot::{Bot, Side};
use clap::Parser;
use log::info;
use std::time;

mod bot;
mod indicators;
mod klines;
mod util;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    symbol: String,

    #[arg(long, default_value_t = 10)]
    fast_ema: u32,

    #[arg(long, default_value_t = 100)]
    slow_ema: u32,

    #[arg(short, long, default_value_t=String::from("30m"))]
    interval: String,

    #[arg(short, long, default_value_t = 10)]
    sleep_duration: u64,

    #[arg(short, long)]
    quantity: f32,

    #[arg(long, value_enum)]
    last_move: Side,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let api_key = dotenv::var("API_KEY").unwrap();
    let api_secret = dotenv::var("API_SECRET").unwrap();

    let bot = Bot::new(&api_key, &api_secret);

    info!("Starting Bot");
    bot.run(
        &cli.symbol,
        cli.quantity,
        cli.fast_ema,
        cli.slow_ema,
        &cli.interval.to_owned(),
        &time::Duration::from_secs(cli.sleep_duration * 60),
        &cli.last_move,
    );
}
