use bot::Bot;
use clap::Parser;
use std::time;

mod bot;
mod indicators;
mod klines;

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

    #[arg(short, long, default_value_t=10*60)]
    sleep_duration: u64,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let bot = Bot::new(cli.symbol);
    bot.run(
        cli.slow_ema,
        cli.fast_ema,
        &cli.interval.to_owned(),
        time::Duration::from_secs(cli.sleep_duration),
    )
    .await;
}
