use crate::bot::Side;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// symbol to use for trading. ex BTCUSDT
    pub symbol: String,

    /// quantity of symbol to trade with
    #[arg(short, long)]
    pub quantity: f32,

    /// the value of span for fast ema
    #[arg(long, default_value_t = 10)]
    pub fast_ema: u32,

    /// the value of span for slow ema
    #[arg(long, default_value_t = 100)]
    pub slow_ema: u32,

    /// the interval or the time frame for the candle sticks
    #[arg(short, long, default_value_t=String::from("30m"))]
    pub interval: String,

    /// the duration the bot will wait before fetching for data
    #[arg(short, long, default_value_t = 10)]
    pub sleep_duration: u64,

    /// the initial position
    #[arg(long, value_enum)]
    pub initial_side: Side,
}