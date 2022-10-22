use serde::Deserialize;
#[derive(Deserialize)]
pub struct _Kline(
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
pub struct _Klines(pub Vec<_Kline>);

#[derive(Debug)]
pub struct Kline {
    open_time: i64,
    open_price: f32,
    high_price: f32,
    low_price: f32,
    pub close_price: f32,
    volume: f32,
    close_time: i64,
}

impl Kline {
    pub fn new(data: &_Kline) -> Self {
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