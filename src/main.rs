use crate::klines::{_Klines, Kline};

mod klines;

async fn get_data() -> _Klines {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.binance.com/api/v3/klines")
        .query(&[("symbol", "BTCUSDT"), ("interval", "30m"), ("limit", "10")])
        .send()
        .await
        .unwrap();

    let data = response.json::<_Klines>().await.unwrap();

    data
}

#[tokio::main]
async fn main() {
    let data = get_data().await;

    let klines: Vec<Kline> = data.0.iter().map(|kline| {
        Kline::new(kline)
    }).collect();

    println!("{:?}", klines);
}
