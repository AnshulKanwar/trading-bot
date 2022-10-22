use crate::klines::{_Klines, Kline};

mod klines;

async fn get_data() -> _Klines {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.binance.com/api/v3/klines")
        .query(&[("symbol", "BTCUSDT"), ("interval", "30m")])
        .send()
        .await
        .unwrap();

    let data = response.json::<_Klines>().await.unwrap();

    data
}

fn get_ema(klines: &Vec<Kline>, span: u32) -> f32 {
    let k: f32 = 2.0 / (span + 1) as f32;
    let (first, rest) = klines.split_first().unwrap();

    let mut ema = first.close_price;

    for kline in rest {
        let price = kline.close_price;
        ema = k * price + (1.0 - k) * ema;
    }

    ema
}

#[tokio::main]
async fn main() {
    let data = get_data().await;

    let mut klines: Vec<Kline> = data.0.iter().map(|kline| {
        Kline::new(kline)
    }).collect();

    // remove the open candle 
    klines.pop();

    println!("{}", get_ema(&klines, 100))
}
