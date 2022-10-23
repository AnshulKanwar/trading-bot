use crate::indicators;
use crate::klines::{Kline, _Klines};
use std::{thread, time};

pub struct Bot {
    client: reqwest::Client,
    base_url: String,
    symbol: String,
}

impl Bot {
    pub fn new(symbol: String) -> Self {
        let client = reqwest::ClientBuilder::new().build().unwrap();
        Bot {
            client,
            base_url: String::from("https://api.binance.com/"),
            symbol,
        }
    }

    pub async fn run(
        &self,
        slow_ema: u32,
        fast_ema: u32,
        interval: &str,
        sleep_duration: time::Duration,
    ) {
        loop {
            let klines = self.fetch_data(interval).await;
            let slow_ema_value = indicators::get_ema(&klines, slow_ema);
            let fast_ema_value = indicators::get_ema(&klines, fast_ema);

            println!(
                "slow_ema = {} \t fast_ema = {}",
                slow_ema_value, fast_ema_value
            );
            thread::sleep(sleep_duration);
        }
    }

    async fn fetch_data(&self, interval: &str) -> Vec<Kline> {
        let response = self
            .client
            .get(self.base_url.to_owned() + "api/v3/klines")
            .query(&[
                ("symbol", &self.symbol),
                ("interval", &interval.to_string()),
            ])
            .send()
            .await
            .unwrap();

        let data = response.json::<_Klines>().await.unwrap();

        let mut klines: Vec<Kline> = data.0.iter().map(|kline| Kline::new(kline)).collect();

        // remove the open candle
        klines.pop();

        klines
    }
}
