use serde::Deserialize;

#[derive(Deserialize)]
struct _Kline(
    i64,
    String,
    String,
    String,
    String,
    String,
    i64,
    String,
    u32,
    String,
    String,
    String,
);

#[derive(Deserialize)]
struct _Klines(Vec<_Kline>);

#[derive(Debug)]
struct Kline {
    open_time: i64,
    open_price: f32,
    high_price: f32,
    low_price: f32,
    close_price: f32,
    volume: f32,
    close_time: i64,
}

impl Kline {
    fn new(data: &_Kline) -> Self {
        Kline {
            open_time: data.0,
            open_price: data.1.parse().unwrap(),
            high_price: data.2.parse().unwrap(),
            low_price: data.3.parse().unwrap(),
            close_price: data.4.parse().unwrap(),
            volume: data.5.parse().unwrap(), 
            close_time: data.6,
        }
    }
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.binance.com/api/v3/klines")
        .query(&[("symbol", "BTCUSDT"), ("interval", "30m"), ("limit", "10")])
        .send()
        .await
        .unwrap();

    let data = response.json::<_Klines>().await.unwrap();

    let klines: Vec<Kline> = data.0.iter().map(|kline| {
        Kline::new(kline)
    }).collect();

    println!("{:?}", klines);
}
