use bot::Bot;
use cli::Cli;
use log::info;
use std::time;
use clap::Parser;

mod bot;
mod cli;
mod indicators;
mod klines;
mod util;

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
