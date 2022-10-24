use crate::bot::Side;
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    pub symbol: String,

    #[arg(long, default_value_t = 10)]
    pub fast_ema: u32,

    #[arg(long, default_value_t = 100)]
    pub slow_ema: u32,

    #[arg(short, long, default_value_t=String::from("30m"))]
    pub interval: String,

    #[arg(short, long, default_value_t = 10)]
    pub sleep_duration: u64,

    #[arg(short, long)]
    pub quantity: f32,

    #[arg(long, value_enum)]
    pub last_move: Side,
}